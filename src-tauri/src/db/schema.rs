use rusqlite::{Connection, Result};

pub fn init_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard_items (
              id TEXT PRIMARY KEY,
              content_type TEXT NOT NULL,
              content_text TEXT,
              content_metadata TEXT NOT NULL DEFAULT '{}',
              source_app TEXT,
              code_language TEXT,
              
              file_url TEXT,
              file_name TEXT,
              file_size_bytes INTEGER,
              file_mime_type TEXT,
              
              is_favorite BOOLEAN NOT NULL DEFAULT 0,
              is_snippet BOOLEAN NOT NULL DEFAULT 0,
              snippet_name TEXT,
              
              created_at TEXT NOT NULL,
              updated_at TEXT NOT NULL,
              
              synced BOOLEAN NOT NULL DEFAULT 0,
              server_id TEXT
          )",
        [],
    )?;

    // Create indexes for better performance
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_created_at ON 
  clipboard_items(created_at DESC)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_content_type ON 
  clipboard_items(content_type)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_is_favorite ON 
  clipboard_items(is_favorite)",
        [],
    )?;

    Ok(())
}
