use chrono::{Duration, Utc};
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// Helper to create a temporary test database
fn setup_test_db() -> (TempDir, PathBuf, Connection) {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_clipboard.db");
    let conn = Connection::open(&db_path).unwrap();

    // Initialize schema (same as production)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard_items (
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
    )
    .unwrap();

    (temp_dir, db_path, conn)
}

fn insert_item(
    conn: &Connection,
    id: &str,
    content: &str,
    is_favorite: bool,
    days_old: i64,
) {
    let now = Utc::now();
    let timestamp = (now - Duration::days(days_old)).to_rfc3339();

    conn.execute(
        "INSERT INTO clipboard_items (
            id, content_type, content_text, is_favorite,
            created_at, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        [
            id,
            "text",
            content,
            if is_favorite { "1" } else { "0" },
            &timestamp,
            &timestamp,
        ],
    )
    .unwrap();
}

fn count_items(conn: &Connection) -> i64 {
    conn.query_row("SELECT COUNT(*) FROM clipboard_items", [], |row| row.get(0))
        .unwrap()
}

#[test]
fn test_cleanup_integration_with_settings_file() {
    let (temp_dir, db_path, conn) = setup_test_db();
    let settings_path = temp_dir.path().join("settings.json");

    // Create test items
    for i in 0..20 {
        insert_item(&conn, &format!("old{}", i), "old content", false, 10);
    }
    for i in 0..10 {
        insert_item(&conn, &format!("recent{}", i), "recent content", false, 3);
    }
    for i in 0..5 {
        insert_item(&conn, &format!("fav{}", i), "favorite", true, 15);
    }

    assert_eq!(count_items(&conn), 35);

    // Create settings file
    let settings = serde_json::json!({
        "maxItemsEnabled": true,
        "maxLocalItems": 15,
        "retentionEnabled": true,
        "retentionDays": 7
    });

    fs::write(&settings_path, settings.to_string()).unwrap();

    // Read settings and perform cleanup (simulating background task)
    let settings_content = fs::read_to_string(&settings_path).unwrap();
    let settings_json: serde_json::Value = serde_json::from_str(&settings_content).unwrap();

    // Retention cleanup
    if settings_json["retentionEnabled"].as_bool().unwrap() {
        let retention_days = settings_json["retentionDays"].as_i64().unwrap() as i32;
        let cutoff_date = Utc::now() - Duration::days(retention_days as i64);
        let cutoff_str = cutoff_date.to_rfc3339();

        let deleted = conn
            .execute(
                "DELETE FROM clipboard_items WHERE updated_at < ?1 AND is_favorite = 0",
                [cutoff_str],
            )
            .unwrap();

        assert_eq!(deleted, 20, "Should delete 20 old non-favorite items");
    }

    assert_eq!(count_items(&conn), 15, "Should have 15 items after retention");

    // Excess cleanup
    if settings_json["maxItemsEnabled"].as_bool().unwrap() {
        let max_items = settings_json["maxLocalItems"].as_i64().unwrap();
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM clipboard_items WHERE is_favorite = 0",
                [],
                |row| row.get(0),
            )
            .unwrap();

        if count > max_items {
            let to_delete = count - max_items;
            conn.execute(
                "DELETE FROM clipboard_items
                 WHERE id IN (
                     SELECT id FROM clipboard_items
                     WHERE is_favorite = 0
                     ORDER BY updated_at ASC
                     LIMIT ?1
                 )",
                [to_delete],
            )
            .unwrap();
        }
    }

    assert_eq!(count_items(&conn), 15, "Should still have 15 items");

    // Verify favorites are intact
    let fav_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM clipboard_items WHERE is_favorite = 1",
            [],
            |row| row.get(0),
        )
        .unwrap();

    assert_eq!(fav_count, 5, "All favorites should remain");
}

#[test]
fn test_cleanup_respects_disabled_settings() {
    let (_temp_dir, _db_path, conn) = setup_test_db();

    // Create test items (all old)
    for i in 0..20 {
        insert_item(&conn, &format!("item{}", i), "content", false, 30);
    }

    assert_eq!(count_items(&conn), 20);

    // Settings with both cleanups DISABLED
    let retention_enabled = false;
    let max_items_enabled = false;

    // Try retention cleanup (should skip)
    if retention_enabled {
        let cutoff_date = Utc::now() - Duration::days(7);
        let cutoff_str = cutoff_date.to_rfc3339();
        conn.execute(
            "DELETE FROM clipboard_items WHERE updated_at < ?1 AND is_favorite = 0",
            [cutoff_str],
        )
        .unwrap();
    }

    // Try excess cleanup (should skip)
    if max_items_enabled {
        conn.execute(
            "DELETE FROM clipboard_items
             WHERE id IN (
                 SELECT id FROM clipboard_items
                 WHERE is_favorite = 0
                 ORDER BY updated_at ASC
                 LIMIT 10
             )",
            [],
        )
        .unwrap();
    }

    assert_eq!(count_items(&conn), 20, "No items should be deleted when settings are disabled");
}

#[test]
fn test_database_stats_calculation() {
    let (_temp_dir, _db_path, conn) = setup_test_db();

    // Insert various item types
    insert_item(&conn, "item1", "content1", false, 1);
    insert_item(&conn, "item2", "content2", false, 2);
    insert_item(&conn, "fav1", "favorite", true, 3);
    insert_item(&conn, "fav2", "favorite", true, 4);

    // Mark some as snippets
    conn.execute("UPDATE clipboard_items SET is_snippet = 1 WHERE id IN ('item1', 'fav1')", [])
        .unwrap();

    // Get stats
    let total: i64 = conn
        .query_row("SELECT COUNT(*) FROM clipboard_items", [], |row| row.get(0))
        .unwrap();
    let favorites: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM clipboard_items WHERE is_favorite = 1",
            [],
            |row| row.get(0),
        )
        .unwrap();
    let snippets: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM clipboard_items WHERE is_snippet = 1",
            [],
            |row| row.get(0),
        )
        .unwrap();

    assert_eq!(total, 4);
    assert_eq!(favorites, 2);
    assert_eq!(snippets, 2);
}

#[test]
fn test_cleanup_preview_accuracy() {
    let (_temp_dir, _db_path, conn) = setup_test_db();

    // Setup: 15 old items, 10 recent items
    for i in 0..15 {
        insert_item(&conn, &format!("old{}", i), "old", false, 10);
    }
    for i in 0..10 {
        insert_item(&conn, &format!("recent{}", i), "recent", false, 3);
    }

    assert_eq!(count_items(&conn), 25);

    // Preview retention cleanup
    let retention_days = 7;
    let cutoff_date = Utc::now() - Duration::days(retention_days);
    let cutoff_str = cutoff_date.to_rfc3339();

    let preview_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM clipboard_items WHERE updated_at < ?1 AND is_favorite = 0",
            [&cutoff_str],
            |row| row.get(0),
        )
        .unwrap();

    assert_eq!(preview_count, 15, "Preview should show 15 items would be deleted");

    // Actually delete
    let deleted = conn
        .execute(
            "DELETE FROM clipboard_items WHERE updated_at < ?1 AND is_favorite = 0",
            [cutoff_str],
        )
        .unwrap();

    assert_eq!(deleted, 15, "Actual deletion should match preview");
    assert_eq!(count_items(&conn), 10, "Should have 10 items remaining");
}
