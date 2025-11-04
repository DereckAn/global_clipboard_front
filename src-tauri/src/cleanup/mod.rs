use chrono::{Duration, Utc};
use rusqlite::{Connection, Result};

// Elimina items mas antiguos que el limite establecido
// IMPORTANTE: Usa updated_at para que items "bumped" no se eliminen prematuramente
pub fn cleanup_old_items(conn: &Connection, retention_days: Option<i32>) -> Result<usize> {
    if let Some(days) = retention_days {
        let cutoff_date = Utc::now() - Duration::days(days as i64);
        let cutoff_str = cutoff_date.to_rfc3339();

        let deleted = conn.execute(
            "DELETE FROM clipboard_items WHERE updated_at < ?1 AND is_favorite = 0",
            [cutoff_str],
        )?;
        Ok(deleted)
    } else {
        Ok(0)
    }
}

// elimina items excedentes cuando se supera el limite
// IMPORTANTE: Usa updated_at para mantener los items más recientemente usados
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

            // Eliminar los items menos recientemente usados (updated_at más antiguo)
            let deleted = conn.execute(
                "DELETE FROM clipboard_items
                 WHERE id IN (
                     SELECT id FROM clipboard_items
                     WHERE is_favorite = 0
                     ORDER BY updated_at ASC
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();

        // Create schema
        conn.execute(
            "CREATE TABLE clipboard_items (
                id TEXT PRIMARY KEY,
                content_type TEXT NOT NULL,
                content_text TEXT,
                content_metadata TEXT,
                source_app TEXT,
                code_language TEXT,
                file_url TEXT,
                file_name TEXT,
                file_size_bytes INTEGER,
                file_mime_type TEXT,
                is_favorite INTEGER DEFAULT 0,
                is_snippet INTEGER DEFAULT 0,
                snippet_name TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                synced INTEGER DEFAULT 0,
                server_id TEXT
            )",
            [],
        ).unwrap();

        conn
    }

    fn insert_test_item(
        conn: &Connection,
        id: &str,
        is_favorite: bool,
        days_old: i64,
    ) -> Result<()> {
        let now = Utc::now();
        let created_at = (now - Duration::days(days_old)).to_rfc3339();
        let updated_at = (now - Duration::days(days_old)).to_rfc3339();

        conn.execute(
            "INSERT INTO clipboard_items (
                id, content_type, content_text, is_favorite,
                created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            [
                id,
                "text",
                &format!("Test content {}", id),
                if is_favorite { "1" } else { "0" },
                &created_at,
                &updated_at,
            ],
        )?;
        Ok(())
    }

    fn count_items(conn: &Connection) -> i64 {
        conn.query_row("SELECT COUNT(*) FROM clipboard_items", [], |row| row.get(0))
            .unwrap()
    }

    fn count_favorites(conn: &Connection) -> i64 {
        conn.query_row(
            "SELECT COUNT(*) FROM clipboard_items WHERE is_favorite = 1",
            [],
            |row| row.get(0),
        )
        .unwrap()
    }

    #[test]
    fn test_cleanup_old_items_deletes_old_non_favorites() {
        let conn = setup_test_db();

        // Insert items: 2 old, 2 recent, 1 old favorite
        insert_test_item(&conn, "old1", false, 10).unwrap(); // Should delete
        insert_test_item(&conn, "old2", false, 8).unwrap();  // Should delete
        insert_test_item(&conn, "recent1", false, 3).unwrap(); // Should keep
        insert_test_item(&conn, "recent2", false, 1).unwrap(); // Should keep
        insert_test_item(&conn, "fav_old", true, 15).unwrap(); // Should keep (favorite)

        assert_eq!(count_items(&conn), 5);

        // Delete items older than 7 days
        let deleted = cleanup_old_items(&conn, Some(7)).unwrap();

        assert_eq!(deleted, 2, "Should delete 2 old non-favorite items");
        assert_eq!(count_items(&conn), 3, "Should have 3 items remaining");
        assert_eq!(count_favorites(&conn), 1, "Favorite should not be deleted");
    }

    #[test]
    fn test_cleanup_old_items_with_none_deletes_nothing() {
        let conn = setup_test_db();

        insert_test_item(&conn, "item1", false, 30).unwrap();
        insert_test_item(&conn, "item2", false, 60).unwrap();

        let deleted = cleanup_old_items(&conn, None).unwrap();

        assert_eq!(deleted, 0, "Should delete nothing when retention_days is None");
        assert_eq!(count_items(&conn), 2, "All items should remain");
    }

    #[test]
    fn test_cleanup_excess_items_keeps_most_recent() {
        let conn = setup_test_db();

        // Insert 10 items with different ages
        for i in 0..10 {
            insert_test_item(&conn, &format!("item{}", i), false, i).unwrap();
        }

        assert_eq!(count_items(&conn), 10);

        // Keep only 5 most recent
        let deleted = cleanup_excess_items(&conn, Some(5)).unwrap();

        assert_eq!(deleted, 5, "Should delete 5 oldest items");
        assert_eq!(count_items(&conn), 5, "Should have 5 items remaining");

        // Verify the newest items remain (0-4 days old)
        let remaining: Vec<String> = conn
            .prepare("SELECT id FROM clipboard_items ORDER BY updated_at DESC")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(remaining, vec!["item0", "item1", "item2", "item3", "item4"]);
    }

    #[test]
    fn test_cleanup_excess_items_protects_favorites() {
        let conn = setup_test_db();

        // Insert 10 non-favorites + 5 favorites
        for i in 0..10 {
            insert_test_item(&conn, &format!("item{}", i), false, i).unwrap();
        }
        for i in 0..5 {
            insert_test_item(&conn, &format!("fav{}", i), true, i + 20).unwrap();
        }

        assert_eq!(count_items(&conn), 15);
        assert_eq!(count_favorites(&conn), 5);

        // Keep only 5 non-favorites (should not touch favorites)
        let deleted = cleanup_excess_items(&conn, Some(5)).unwrap();

        assert_eq!(deleted, 5, "Should delete 5 oldest non-favorite items");
        assert_eq!(count_items(&conn), 10, "Should have 10 items total");
        assert_eq!(count_favorites(&conn), 5, "All favorites should remain");
    }

    #[test]
    fn test_cleanup_excess_items_when_under_limit() {
        let conn = setup_test_db();

        insert_test_item(&conn, "item1", false, 1).unwrap();
        insert_test_item(&conn, "item2", false, 2).unwrap();

        let deleted = cleanup_excess_items(&conn, Some(10)).unwrap();

        assert_eq!(deleted, 0, "Should delete nothing when under limit");
        assert_eq!(count_items(&conn), 2, "All items should remain");
    }

    #[test]
    fn test_cleanup_excess_items_with_none_deletes_nothing() {
        let conn = setup_test_db();

        for i in 0..20 {
            insert_test_item(&conn, &format!("item{}", i), false, i).unwrap();
        }

        let deleted = cleanup_excess_items(&conn, None).unwrap();

        assert_eq!(deleted, 0, "Should delete nothing when max_items is None");
        assert_eq!(count_items(&conn), 20, "All items should remain");
    }

    #[test]
    fn test_combined_cleanup_scenario() {
        let conn = setup_test_db();

        // Scenario: 25 items total
        // - 10 old non-favorites (>7 days) - should delete
        // - 10 recent non-favorites (<7 days) - should keep
        // - 2 old favorites (>7 days) - should keep (protected)
        // - 3 recent favorites (<7 days) - should keep

        // Old non-favorites: 8-17 days old (all > 7)
        for i in 0..10 {
            insert_test_item(&conn, &format!("old{}", i), false, 8 + i).unwrap();
        }

        // Recent non-favorites: 0-6 days old (all <= 7)
        for i in 0..10 {
            insert_test_item(&conn, &format!("recent{}", i), false, i % 7).unwrap();
        }

        // Old favorites: 15 days old (> 7 but protected)
        for i in 0..2 {
            insert_test_item(&conn, &format!("fav_old{}", i), true, 15).unwrap();
        }

        // Recent favorites: 3 days old (< 7)
        for i in 0..3 {
            insert_test_item(&conn, &format!("fav_recent{}", i), true, 3).unwrap();
        }

        assert_eq!(count_items(&conn), 25, "Should start with 25 items");
        assert_eq!(count_favorites(&conn), 5, "Should have 5 favorites");

        // Step 1: Delete old items (>7 days, non-favorites only)
        let deleted_old = cleanup_old_items(&conn, Some(7)).unwrap();
        assert_eq!(deleted_old, 10, "Should delete 10 old non-favorite items");

        // After retention cleanup: 10 recent + 5 favorites = 15 items
        assert_eq!(count_items(&conn), 15, "Should have 15 items after retention cleanup");

        // Step 2: Delete excess (keep max 10 non-favorites)
        // We have 10 non-favorites, so nothing should be deleted
        let deleted_excess = cleanup_excess_items(&conn, Some(10)).unwrap();
        assert_eq!(deleted_excess, 0, "Should delete nothing (exactly at limit)");

        assert_eq!(count_items(&conn), 15, "Should have 15 items total");
        assert_eq!(count_favorites(&conn), 5, "All 5 favorites should remain");
    }

    #[test]
    fn test_optimize_database_runs_successfully() {
        let conn = setup_test_db();

        insert_test_item(&conn, "item1", false, 1).unwrap();
        insert_test_item(&conn, "item2", false, 2).unwrap();

        // Should not panic
        let result = optimize_database(&conn);
        assert!(result.is_ok(), "VACUUM should complete successfully");
    }
}
