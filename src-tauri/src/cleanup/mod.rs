use chrono::{Duration, Utc};
use rusqlite::{Connection, Result};

// Elimina items mas antiguos que el limite establecido
pub fn cleanup_old_items(conn: &Connection, retention_days: Option<i32>) -> Result<usize> {
    if let Some(days) = retention_days {
        let cutoff_date = Utc::now() - Duration::days(days as i64);
        let cutoff_str = cutoff_date.to_rfc3339();

        let deleted = conn.execute(
            "DELETE FROM clipboard_items WHERE created_at < ?1 AND is_favorite = 0",
            [cutoff_str],
        )?;
        Ok(deleted)
    } else {
        Ok(0)
    }
}

// elimina items excedentes cuando se supera el limite
pub fn cleanup_excess_items(conn: &Connection, max_items: Option<i32>) -> Result<usize> {
    if let Some(max) = max_items {
        // Contar iteems actuales (excluyendo favoritos)
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM clipboard_items WHERE is_favorite = 0",
            [],
            |row| row.get(0),
        )?;

        if count > max as i64 {
            let to_delete = count - max as i64;

            // Eliminar los items mas antiguos que no sean favoritos
            let deleted = conn.execute(
                "DELETE FROM clipboard_items 
                 WHERE id IN (
                     SELECT id FROM clipboard_items 
                     WHERE is_favorite = 0 
                     ORDER BY created_at ASC 
                     LIMIT ?1
                 )",
                [to_delete],
            )?;
            Ok(deleted)
        } else {
            Ok(0)
        }
    } else {
        Ok(0)
    }
}

// Calcula el tamaño de la base de datos
pub fn get_database_size(db_path: &str) -> Result<u64> {
    use std::fs;
    match fs::metadata(db_path) {
        Ok(metadata) => Ok(metadata.len()),
        Err(_) => Ok(0),
    }
}

// Optimiza la base de datos (VACUUM)
pub fn optimize_database(conn: &Connection) -> Result<()> {
    conn.execute("VACUUM", [])?;
    Ok(())
}
