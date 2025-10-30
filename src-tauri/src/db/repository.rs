use crate::db::models::{ClipboardItem, CreateClipboardItemDto, UpdateClipboardItemDto};
use chrono::Utc;
use rusqlite::{params, Connection, Result};
use uuid::Uuid;

pub struct ClipboardRepository {
    conn: Connection,
}

impl ClipboardRepository {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        crate::db::schema::init_database(&conn)?;
        Ok(Self { conn })
    }

    // Removed: Use get_items_paginated() instead for better performance

    pub fn get_item(&self, id: &str) -> Result<Option<ClipboardItem>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content_type, content_text, content_metadata, source_app, code_language,
                      file_url, file_name, file_size_bytes, file_mime_type,
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
                is_favorite: row.get(10)?,
                is_snippet: row.get(11)?,
                snippet_name: row.get(12)?,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
                synced: row.get(15)?,
                server_id: row.get(16)?,
            }))
        } else {
            Ok(None)
        }
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
                    file_url, file_name, file_size_bytes, file_mime_type,
                    is_favorite, is_snippet, snippet_name,
                    created_at, updated_at, synced, server_id
             FROM clipboard_items
             ORDER BY created_at DESC
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
                is_favorite: row.get(10)?,
                is_snippet: row.get(11)?,
                snippet_name: row.get(12)?,
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
                synced: row.get(15)?,
                server_id: row.get(16)?,
            })
        })?;

        items.collect()
    }

    // NUEVO: Contar total de items
    pub fn count_items(&self) -> Result<i64> {
        let mut stmt = self.conn.prepare("SELECT COUNT(*) FROM clipboard_items")?;
        let count: i64 = stmt.query_row([], |row| row.get(0))?;
        Ok(count)
    }

    // NUEVO: Búsqueda con paginación
    pub fn search_items_paginated(
        &self,
        query: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ClipboardItem>> {
        let search_term = format!("%{query}%");
        let mut stmt = self.conn.prepare(
            "SELECT id, content_type, content_text, content_metadata, source_app, code_language,
                    file_url, file_name, file_size_bytes, file_mime_type,
                    is_favorite, is_snippet, snippet_name,
                    created_at, updated_at, synced, server_id
             FROM clipboard_items
             WHERE content_text LIKE ?1
             ORDER BY created_at DESC
             LIMIT ?2 OFFSET ?3",
        )?;

        let items = stmt.query_map(
            [search_term, limit.to_string(), offset.to_string()],
            |row| {
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
                    is_favorite: row.get(10)?,
                    is_snippet: row.get(11)?,
                    snippet_name: row.get(12)?,
                    created_at: row.get(13)?,
                    updated_at: row.get(14)?,
                    synced: row.get(15)?,
                    server_id: row.get(16)?,
                })
            },
        )?;

        items.collect()
    }
}
