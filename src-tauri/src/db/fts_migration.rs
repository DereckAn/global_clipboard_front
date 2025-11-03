use rusqlite::{Connection, Result};

/// Inicializa la tabla FTS5 para búsqueda full-text
pub fn init_fts_table(conn: &Connection) -> Result<()> {
    // Crear tabla virtual FTS5
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS clipboard_items_fts USING fts5(
              id UNINDEXED,
              content_text,
              content_metadata,
              file_name,
              snippet_name,
              code_language,
              tokenize = 'porter unicode61 remove_diacritics 2'
          )",
        [],
    )?;

    // Verificar si ya tiene datos
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM clipboard_items_fts", [], |row| {
        row.get(0)
    })?;

    // Si está vacía, poblar con datos existentes
    if count == 0 {
        conn.execute(
              "INSERT INTO clipboard_items_fts (id, content_text, content_metadata, file_name, snippet_name, code_language)
               SELECT id, 
                      COALESCE(content_text, ''),
                      COALESCE(content_metadata, ''),
                      COALESCE(file_name, ''),
                      COALESCE(snippet_name, ''),
                      COALESCE(code_language, '')
               FROM clipboard_items",
              [],
          )?;
    }

    Ok(())
}

/// Trigger para mantener FTS sincronizado cuando se inserta un item
pub fn create_fts_triggers(conn: &Connection) -> Result<()> {
    // Trigger INSERT
    conn.execute(
          "CREATE TRIGGER IF NOT EXISTS clipboard_items_fts_insert 
           AFTER INSERT ON clipboard_items
           BEGIN
               INSERT INTO clipboard_items_fts (id, content_text, content_metadata, file_name, snippet_name, code_language)
               VALUES (
                   new.id,
                   COALESCE(new.content_text, ''),
                   COALESCE(new.content_metadata, ''),
                   COALESCE(new.file_name, ''),
                   COALESCE(new.snippet_name, ''),
                   COALESCE(new.code_language, '')
               );
           END",
          [],
      )?;

    // Trigger DELETE
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS clipboard_items_fts_delete 
           AFTER DELETE ON clipboard_items
           BEGIN
               DELETE FROM clipboard_items_fts WHERE id = old.id;
           END",
        [],
    )?;

    // Trigger UPDATE
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS clipboard_items_fts_update 
           AFTER UPDATE ON clipboard_items
           BEGIN
               UPDATE clipboard_items_fts 
               SET content_text = COALESCE(new.content_text, ''),
                   content_metadata = COALESCE(new.content_metadata, ''),
                   file_name = COALESCE(new.file_name, ''),
                   snippet_name = COALESCE(new.snippet_name, ''),
                   code_language = COALESCE(new.code_language, '')
               WHERE id = old.id;
           END",
        [],
    )?;

    Ok(())
}
