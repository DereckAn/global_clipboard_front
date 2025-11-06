use crate::db::models::{ClipboardItem, CreateClipboardItemDto, UpdateClipboardItemDto};
use chrono::Utc;
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

pub struct ClipboardRepository {
    pub conn: Connection,
}

impl ClipboardRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        crate::db::schema::init_database(&conn)?;
        Ok(Self { conn })
    }

    pub fn get_item(&self, id: &str) -> Result<Option<ClipboardItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content_type, content_text, content_metadata, source_app, code_language,
                      file_url, file_name, file_size_bytes, file_mime_type, file_hash,
                      is_favorite, is_snippet, snippet_name,
                      created_at, updated_at, synced, server_id
               FROM clipboard_items
               WHERE id = ?1",
        )?;

        let mut rows = stmt.query([id])?;

        if let Some(row) = rows.next()? {
            Ok(Some(ClipboardItem {
                id: row.get(0)?,
                content_type: row.get(1)?,
                content_text: row.get(2)?,
                content_metadata: row.get(3)?,
                source_app: row.get(4)?,
                code_language: row.get(5)?,
                file_url: row.get(6)?,
                file_name: row.get(7)?,
                file_size_bytes: row.get(8)?,
                file_mime_type: row.get(9)?,
                file_hash: row.get(10)?,
                is_favorite: row.get(11)?,
                is_snippet: row.get(12)?,
                snippet_name: row.get(13)?,
                created_at: row.get(14)?,
                updated_at: row.get(15)?,
                synced: row.get(16)?,
                server_id: row.get(17)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// Find existing item by content_text to avoid duplicates
    pub fn find_by_content(&self, content_text: &str) -> Result<Option<ClipboardItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content_type, content_text, content_metadata, source_app, code_language,
                      file_url, file_name, file_size_bytes, file_mime_type, file_hash,
                      is_favorite, is_snippet, snippet_name,
                      created_at, updated_at, synced, server_id
               FROM clipboard_items
               WHERE content_text = ?1
               ORDER BY updated_at DESC
               LIMIT 1",
        )?;

        let mut rows = stmt.query([content_text])?;

        if let Some(row) = rows.next()? {
            Ok(Some(ClipboardItem {
                id: row.get(0)?,
                content_type: row.get(1)?,
                content_text: row.get(2)?,
                content_metadata: row.get(3)?,
                source_app: row.get(4)?,
                code_language: row.get(5)?,
                file_url: row.get(6)?,
                file_name: row.get(7)?,
                file_size_bytes: row.get(8)?,
                file_mime_type: row.get(9)?,
                file_hash: row.get(10)?,
                is_favorite: row.get(11)?,
                is_snippet: row.get(12)?,
                snippet_name: row.get(13)?,
                created_at: row.get(14)?,
                updated_at: row.get(15)?,
                synced: row.get(16)?,
                server_id: row.get(17)?,
            }))
        } else {
            Ok(None)
        }
    }

    /// "Bump" an existing item to the top by updating its timestamp
    pub fn bump_item(&self, id: &str) -> Result<ClipboardItem> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "UPDATE clipboard_items SET updated_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;

        self.get_item(id).map(|opt| opt.unwrap())
    }

    pub fn create_item(&self, dto: CreateClipboardItemDto) -> Result<ClipboardItem> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        let metadata = dto.content_metadata.unwrap_or_else(|| "{}".to_string());

        self.conn.execute(
            "INSERT INTO clipboard_items (
                  id, content_type, content_text, content_metadata, source_app, code_language,
                  is_favorite, is_snippet, created_at, updated_at, synced
              ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0, 0, ?7, ?8, 0)",
            params![
                id,
                dto.content_type,
                dto.content_text,
                metadata,
                dto.source_app,
                dto.code_language, // <- AGREGADO
                now,
                now
            ],
        )?;

        self.get_item(&id).map(|opt| opt.unwrap())
    }

    /// Create or update item - prevents duplicates
    pub fn upsert_item(&self, dto: CreateClipboardItemDto) -> Result<ClipboardItem> {
        // Check if item with same content already exists
        if let Some(existing) = self.find_by_content(&dto.content_text)? {
            // Item exists, just bump it to the top
            self.bump_item(&existing.id)
        } else {
            // New item, create it
            self.create_item(dto)
        }
    }

    pub fn update_item(&self, id: &str, dto: UpdateClipboardItemDto) -> Result<ClipboardItem> {
        let now = Utc::now().to_rfc3339();

        // Build dynamic update query
        let mut updates = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(content_text) = dto.content_text {
            updates.push("content_text = ?");
            params.push(Box::new(content_text));
        }
        if let Some(is_favorite) = dto.is_favorite {
            updates.push("is_favorite = ?");
            params.push(Box::new(is_favorite));
        }
        if let Some(is_snippet) = dto.is_snippet {
            updates.push("is_snippet = ?");
            params.push(Box::new(is_snippet));
        }
        if let Some(snippet_name) = dto.snippet_name {
            updates.push("snippet_name = ?");
            params.push(Box::new(snippet_name));
        }

        updates.push("updated_at = ?");
        params.push(Box::new(now));

        params.push(Box::new(id.to_string()));

        let query = format!(
            "UPDATE clipboard_items SET {} WHERE id = ?",
            updates.join(", ")
        );

        let params_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        self.conn.execute(&query, params_refs.as_slice())?;

        self.get_item(id).map(|opt| opt.unwrap())
    }

    pub fn delete_item(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM clipboard_items WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn clear_all(&self) -> Result<()> {
        self.conn.execute("DELETE FROM clipboard_items", [])?;
        Ok(())
    }

    // Removed: Use search_items_paginated() instead for better performance
    pub fn remove_duplicates(&self) -> Result<usize> {
        // Delete duplicate items, keeping only the most recent one for each content
        let deleted = self.conn.execute(
            "DELETE FROM clipboard_items
               WHERE id NOT IN (
                   SELECT MIN(id)
                   FROM clipboard_items
                   GROUP BY content_text
               )",
            [],
        )?;

        Ok(deleted)
    }

    // Get items con paginación
    pub fn get_items_paginated(&self, limit: i64, offset: i64) -> Result<Vec<ClipboardItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content_type, content_text, content_metadata, source_app, code_language,
                    file_url, file_name, file_size_bytes, file_mime_type, file_hash,
                    is_favorite, is_snippet, snippet_name,
                    created_at, updated_at, synced, server_id
             FROM clipboard_items
             ORDER BY updated_at DESC
             LIMIT ?1 OFFSET ?2",
        )?;

        let items = stmt.query_map([limit, offset], |row| {
            Ok(ClipboardItem {
                id: row.get(0)?,
                content_type: row.get(1)?,
                content_text: row.get(2)?,
                content_metadata: row.get(3)?,
                source_app: row.get(4)?,
                code_language: row.get(5)?,
                file_url: row.get(6)?,
                file_name: row.get(7)?,
                file_size_bytes: row.get(8)?,
                file_mime_type: row.get(9)?,
                file_hash: row.get(10)?,
                is_favorite: row.get(11)?,
                is_snippet: row.get(12)?,
                snippet_name: row.get(13)?,
                created_at: row.get(14)?,
                updated_at: row.get(15)?,
                synced: row.get(16)?,
                server_id: row.get(17)?,
            })
        })?;

        items.collect()
    }

    // Contar total de items
    pub fn count_items(&self) -> Result<i64> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM clipboard_items")?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;
        Ok(count)
    }

    pub fn search_items_fts(
        &self,
        query: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ClipboardItem>> {
        // Preparar query para FTS5
        // Si tiene espacios, asumimos búsqueda de frase exacta
        let fts_query = if query.contains(' ') {
            format!("\"{}\"", query)
        } else {
            // Búsqueda de palabra con prefijo (para autocompletado)
            format!("{}*", query)
        };

        let mut stmt = self.conn.prepare(
            "SELECT ci.id, ci.content_type, ci.content_text, ci.content_metadata,
                  ci.source_app, ci.code_language,
                  ci.file_url, ci.file_name, ci.file_size_bytes, ci.file_mime_type, ci.file_hash,
                  ci.is_favorite, ci.is_snippet, ci.snippet_name,
                  ci.created_at, ci.updated_at, ci.synced, ci.server_id,
                  bm25(clipboard_items_fts) as rank
           FROM clipboard_items ci
           INNER JOIN clipboard_items_fts fts ON ci.id = fts.id
           WHERE clipboard_items_fts MATCH ?1
           ORDER BY rank
           LIMIT ?2 OFFSET ?3",
        )?;

        let items = stmt.query_map(params![&fts_query, limit, offset], |row| {
            Ok(ClipboardItem {
                id: row.get(0)?,
                content_type: row.get(1)?,
                content_text: row.get(2)?,
                content_metadata: row.get(3)?,
                source_app: row.get(4)?,
                code_language: row.get(5)?,
                file_url: row.get(6)?,
                file_name: row.get(7)?,
                file_size_bytes: row.get(8)?,
                file_mime_type: row.get(9)?,
                file_hash: row.get(10)?,
                is_favorite: row.get(11)?,
                is_snippet: row.get(12)?,
                snippet_name: row.get(13)?,
                created_at: row.get(14)?,
                updated_at: row.get(15)?,
                synced: row.get(16)?,
                server_id: row.get(17)?,
                // Ignoramos rank (row 18) por ahora
            })
        })?;

        items.collect()
    }

    // Contar resultados FTS
    pub fn count_search_results_fts(&self, query: &str) -> Result<i64> {
        let fts_query = if query.contains(' ') {
            format!("\"{}\"", query)
        } else {
            format!("{}*", query)
        };

        let mut stmt = self.conn.prepare(
            "SELECT COUNT(*) 
           FROM clipboard_items_fts 
           WHERE clipboard_items_fts MATCH ?1",
        )?;

        let count: i64 = stmt.query_row([&fts_query], |row| row.get(0))?;
        Ok(count)
    }

    // Update file information for an item
    pub fn update_file_info(
        &self,
        id: &str,
        file_url: &str,
        file_name: &str,
        file_size_bytes: i64,
        file_mime_type: &str,
        file_hash: Option<&str>,
    ) -> Result<()> {
        self.conn.execute(
            "UPDATE clipboard_items
            SET file_url = ?1, file_name = ?2, file_size_bytes = ?3, file_mime_type = ?4, file_hash = ?5, updated_at = ?6
            WHERE id = ?7",
            rusqlite::params![
                file_url,
                file_name,
                file_size_bytes,
                file_mime_type,
                file_hash,
                chrono::Utc::now().to_rfc3339(),
                id
            ],
        )?;
        Ok(())
    }

    /// Find existing item by file_hash to avoid duplicate files
    pub fn find_by_file_hash(&self, file_hash: &str) -> Result<Option<ClipboardItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content_type, content_text, content_metadata, source_app, code_language,
                      file_url, file_name, file_size_bytes, file_mime_type, file_hash,
                      is_favorite, is_snippet, snippet_name,
                      created_at, updated_at, synced, server_id
               FROM clipboard_items
               WHERE file_hash = ?1
               ORDER BY updated_at DESC
               LIMIT 1",
        )?;

        let mut rows = stmt.query([file_hash])?;

        if let Some(row) = rows.next()? {
            Ok(Some(ClipboardItem {
                id: row.get(0)?,
                content_type: row.get(1)?,
                content_text: row.get(2)?,
                content_metadata: row.get(3)?,
                source_app: row.get(4)?,
                code_language: row.get(5)?,
                file_url: row.get(6)?,
                file_name: row.get(7)?,
                file_size_bytes: row.get(8)?,
                file_mime_type: row.get(9)?,
                file_hash: row.get(10)?,
                is_favorite: row.get(11)?,
                is_snippet: row.get(12)?,
                snippet_name: row.get(13)?,
                created_at: row.get(14)?,
                updated_at: row.get(15)?,
                synced: row.get(16)?,
                server_id: row.get(17)?,
            }))
        } else {
            Ok(None)
        }
    }
}
