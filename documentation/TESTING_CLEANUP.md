# Testing Guide: Automatic Cleanup System

This guide explains how to test and verify the automatic cleanup functionality.

## Overview

The cleanup system has two modes:
1. **Retention Cleanup**: Deletes items older than X days (based on `updated_at`)
2. **Excess Cleanup**: Deletes least recently used items when exceeding max limit

The cleanup runs automatically every 24 hours in the background.

## Test Commands

Two test commands are available in the console:

### 1. `tauriTestCleanupPreview(retentionDays, maxItems)`

**Purpose**: Preview what WOULD be deleted without actually deleting anything.

**Parameters**:
- `retentionDays`: Number (e.g., 7, 30) or `null` to skip retention check
- `maxItems`: Number (e.g., 100, 500) or `null` to skip excess check

**Returns**:
```javascript
{
  retention: {
    would_delete: 15,
    cutoff_date: "2025-10-28T12:00:00Z",
    retention_days: 7
  },
  excess: {
    current_count: 157,
    max_items: 100,
    would_delete: 57
  }
}
```

**Example Usage** (in browser console):
```javascript
import { tauriTestCleanupPreview } from '$lib/tauri/commands';

// Preview both retention (7 days) and excess (100 items)
const preview = await tauriTestCleanupPreview(7, 100);
console.log('Preview:', preview);

// Preview only retention
const retentionOnly = await tauriTestCleanupPreview(7, null);
console.log('Retention:', retentionOnly);

// Preview only excess
const excessOnly = await tauriTestCleanupPreview(null, 100);
console.log('Excess:', excessOnly);
```

### 2. `tauriTestForceCleanup()`

**Purpose**: Force immediate cleanup based on current settings in `settings.json`.

**Parameters**: None (reads from settings file)

**Returns**:
```javascript
{
  retention_deleted: 15,  // Only if retentionEnabled = true
  excess_deleted: 57      // Only if maxItemsEnabled = true
}
```

**Example Usage**:
```javascript
import { tauriTestForceCleanup } from '$lib/tauri/commands';

// Force cleanup now
const result = await tauriTestForceCleanup();
console.log('Deleted items:', result);
```

## Testing Procedure

### Step 1: Prepare Test Data

1. Open the app and copy various items to build up clipboard history
2. Create at least 100-150 items for testing excess cleanup
3. Some items should be old (you can manually update database if needed)

### Step 2: Configure Settings

1. Go to Settings page
2. Enable "Límite de items guardados" and set to 100
3. Enable "Tiempo de retención" and set to 7 days
4. Save settings

### Step 3: Preview Cleanup

Open browser console (F12) and run:

```javascript
// Check what would be deleted
const preview = await tauriTestCleanupPreview(7, 100);
console.log('Preview:', preview);
```

Expected output:
- `retention.would_delete`: Number of items older than 7 days (from last use)
- `excess.would_delete`: Number of items beyond the 100 limit

### Step 4: Force Cleanup

Run the cleanup command:

```javascript
const result = await tauriTestForceCleanup();
console.log('Cleanup result:', result);
```

### Step 5: Verify Results

1. Check the console output - it should match the preview counts
2. Refresh the app and verify:
   - Old items are gone
   - Total items ≤ 100 (or your max limit)
   - Favorites are NOT deleted (they're protected)
   - Most recently used items remain

### Step 6: Check Database Stats

```javascript
import { tauriGetDatabaseStats } from '$lib/tauri/commands';

const stats = await tauriGetDatabaseStats();
console.log('Database stats:', stats);
```

Expected output:
```javascript
{
  total_items: 100,  // Should match your max limit
  favorites: 5,      // Favorites are preserved
  snippets: 3,
  database_size_bytes: 123456,
  database_size_mb: 0.12
}
```

## Manual Database Testing

If you need to manually set old dates for testing:

```bash
# Open database
sqlite3 ~/Library/Application\ Support/clip/clipboard.db

# Check current items
SELECT id, content_text, updated_at FROM clipboard_items ORDER BY updated_at DESC LIMIT 10;

# Manually set some items to old dates (for testing)
UPDATE clipboard_items
SET updated_at = '2025-10-20T12:00:00Z'
WHERE id IN (
  SELECT id FROM clipboard_items
  WHERE is_favorite = 0
  LIMIT 20
);

# Verify changes
SELECT COUNT(*) as old_items
FROM clipboard_items
WHERE updated_at < datetime('now', '-7 days');

# Exit
.quit
```

## Automatic Background Task

The background task runs every 24 hours automatically. To verify it's working:

1. Check the app logs after startup:
   ```
   🧹 Running daily automatic cleanup...
   🧹 Deleted X old items (retention policy)
   🧹 Deleted Y excess items (max limit)
   ✅ Daily cleanup completed
   ```

2. Settings are read from: `~/Library/Application Support/clip/settings.json`

3. Verify settings file contains:
   ```json
   {
     "maxItemsEnabled": true,
     "maxLocalItems": 100,
     "retentionEnabled": true,
     "retentionDays": 7
   }
   ```

## Important Notes

### What Gets Deleted

✅ **WILL be deleted**:
- Items older than retention days (based on `updated_at`)
- Excess items beyond max limit (least recently used first)
- Items with `is_favorite = 0`

❌ **WILL NOT be deleted**:
- Favorite items (`is_favorite = 1`)
- Items within retention period
- Items within max limit

### Date Field: `updated_at` vs `created_at`

The system uses `updated_at` for all cleanup operations:

- **created_at**: When item was first created
- **updated_at**: When item was last used (copied, double-clicked, or bumped)

This means:
- Old items that you re-copy won't be deleted
- Items are deleted based on last use, not creation
- Each item has individual expiration timer

### Testing Tips

1. **Use small retention periods**: Use 1-2 days for testing instead of 7-30 days
2. **Use small max limits**: Use 10-20 items for testing instead of 100-500
3. **Check before and after**: Always preview before forcing cleanup
4. **Verify favorites**: Make sure favorites are never deleted
5. **Check database stats**: Use `tauriGetDatabaseStats()` to verify counts

## Troubleshooting

### Cleanup doesn't work

1. Check settings file exists: `~/Library/Application Support/clip/settings.json`
2. Verify settings have correct structure (see above)
3. Check app logs for error messages
4. Ensure `retentionEnabled` and `maxItemsEnabled` are `true`

### Wrong items deleted

1. Verify you're using `updated_at` not `created_at` in queries
2. Check the preview results before cleanup
3. Make sure favorites have `is_favorite = 1`

### Background task not running

1. Check app logs on startup
2. Verify the 24-hour loop is active
3. For immediate testing, use `tauriTestForceCleanup()` instead

## Example Test Session

```javascript
// 1. Check current state
const before = await tauriGetDatabaseStats();
console.log('Before:', before);
// { total_items: 157, favorites: 5, ... }

// 2. Preview what will be deleted
const preview = await tauriTestCleanupPreview(7, 100);
console.log('Preview:', preview);
// { retention: { would_delete: 15 }, excess: { would_delete: 42 } }

// 3. Force cleanup
const result = await tauriTestForceCleanup();
console.log('Result:', result);
// { retention_deleted: 15, excess_deleted: 42 }

// 4. Verify final state
const after = await tauriGetDatabaseStats();
console.log('After:', after);
// { total_items: 100, favorites: 5, ... }

// 5. Confirm math
console.log('Math check:', before.total_items - result.retention_deleted - result.excess_deleted === after.total_items);
// true (157 - 15 - 42 = 100)
```

## Summary

The automatic cleanup system:
- ✅ Runs every 24 hours automatically
- ✅ Uses `updated_at` for all operations (MRU - Most Recently Used)
- ✅ Protects favorites from deletion
- ✅ Configurable retention days and max items
- ✅ Testable with preview and force commands
- ✅ No restart required when changing settings
