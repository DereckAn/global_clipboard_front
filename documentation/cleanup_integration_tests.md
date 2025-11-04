# Testing Documentation

This document explains the automated testing system for the Global Clipboard Manager.

## Quick Start

```bash
# Run all tests
bun run test

# Run only unit tests
bun run test:unit

# Run only integration tests
bun run test:integration

# Run tests + type checking
bun run test:all
```

## Overview

The project uses **Rust's built-in testing framework** (`cargo test`) for backend tests. We have two types of tests:

1. **Unit Tests**: Test individual functions in isolation (8 tests)
2. **Integration Tests**: Test complete workflows and interactions (4 tests)

**Total: 12 automated tests** ✅

## Test Structure

```
src-tauri/
├── src/
│   └── cleanup/
│       └── mod.rs              # Unit tests at bottom of file
└── tests/
    └── cleanup_integration_tests.rs  # Integration tests
```

## Running Tests

### All Tests
```bash
bun run test
# or
cargo test
```

### Unit Tests Only
```bash
bun run test:unit
# or
cargo test --lib
```

### Integration Tests Only
```bash
bun run test:integration
# or
cargo test --test cleanup_integration_tests
```

### Type Checking + All Tests
```bash
bun run test:all
```

### Run Specific Test
```bash
cargo test test_cleanup_old_items_deletes_old_non_favorites
```

### Run with Output
```bash
cargo test -- --nocapture
```

## Test Coverage

### Unit Tests (8 tests)

Located in: `src-tauri/src/cleanup/mod.rs`

1. **test_cleanup_old_items_deletes_old_non_favorites**
   - Verifies old items (>7 days) are deleted
   - Verifies recent items are kept
   - Verifies favorites are protected

2. **test_cleanup_old_items_with_none_deletes_nothing**
   - Verifies nothing deleted when `retention_days = None`

3. **test_cleanup_excess_items_keeps_most_recent**
   - Verifies LRU (Least Recently Used) deletion
   - Keeps the 5 most recent items when limit = 5

4. **test_cleanup_excess_items_protects_favorites**
   - Verifies favorites don't count toward limit
   - Only non-favorites are deleted

5. **test_cleanup_excess_items_when_under_limit**
   - Verifies nothing deleted when under limit

6. **test_cleanup_excess_items_with_none_deletes_nothing**
   - Verifies nothing deleted when `max_items = None`

7. **test_combined_cleanup_scenario**
   - Tests retention + excess cleanup together
   - Verifies correct order of operations
   - Complex scenario with 25 items

8. **test_optimize_database_runs_successfully**
   - Verifies VACUUM command works

### Integration Tests (4 tests)

Located in: `src-tauri/tests/cleanup_integration_tests.rs`

1. **test_cleanup_integration_with_settings_file**
   - Tests complete cleanup workflow
   - Reads settings from JSON file
   - Simulates background task behavior
   - Verifies retention + excess cleanup

2. **test_cleanup_respects_disabled_settings**
   - Verifies cleanup is skipped when disabled
   - Tests both `retentionEnabled` and `maxItemsEnabled`

3. **test_database_stats_calculation**
   - Tests stats calculation logic
   - Counts total, favorites, and snippets

4. **test_cleanup_preview_accuracy**
   - Verifies preview matches actual deletion
   - Tests the `test_cleanup_preview` command logic

## Test Helpers

### setup_test_db()
Creates in-memory SQLite database with full schema for testing.

```rust
let conn = setup_test_db();
```

### insert_test_item()
Helper to insert test items with specific dates.

```rust
insert_test_item(&conn, "item1", false, 10); // 10 days old, not favorite
insert_test_item(&conn, "fav1", true, 5);    // 5 days old, favorite
```

### count_items() / count_favorites()
Quick helpers to verify database state.

```rust
assert_eq!(count_items(&conn), 10);
assert_eq!(count_favorites(&conn), 2);
```

## Writing New Tests

### Unit Test Example

Add to `src-tauri/src/cleanup/mod.rs`:

```rust
#[test]
fn test_my_new_feature() {
    let conn = setup_test_db();

    // Setup
    insert_test_item(&conn, "item1", false, 5);

    // Execute
    let result = my_function(&conn);

    // Assert
    assert_eq!(result, expected_value);
}
```

### Integration Test Example

Add to `src-tauri/tests/cleanup_integration_tests.rs`:

```rust
#[test]
fn test_new_integration_scenario() {
    let (temp_dir, db_path, conn) = setup_test_db();

    // Setup test data
    // ...

    // Execute workflow
    // ...

    // Verify results
    assert_eq!(actual, expected);
}
```

## CI/CD Integration

Tests run automatically on:
- Every push to main/develop branches
- Every pull request
- Manual trigger via GitHub Actions

See `.github/workflows/test.yml` for configuration.

## Test Best Practices

1. **Isolation**: Each test should be independent
2. **Clarity**: Test names should describe what they test
3. **Coverage**: Test both success and failure cases
4. **Speed**: Use in-memory databases for fast tests
5. **Assertions**: Clear assertion messages

### Good Test Structure

```rust
#[test]
fn test_descriptive_name() {
    // Arrange (Setup)
    let conn = setup_test_db();
    insert_test_item(&conn, "item1", false, 10);

    // Act (Execute)
    let result = cleanup_old_items(&conn, Some(7)).unwrap();

    // Assert (Verify)
    assert_eq!(result, 1, "Should delete 1 old item");
    assert_eq!(count_items(&conn), 0, "Should have 0 items remaining");
}
```

## Debugging Failed Tests

### View Detailed Output
```bash
cargo test -- --nocapture --test-threads=1
```

### Run Single Test
```bash
cargo test test_cleanup_old_items_deletes_old_non_favorites -- --nocapture
```

### Check Test Coverage
```bash
cargo tarpaulin --out Html
```

## Current Test Results

All tests passing ✅

```
Unit Tests:        8 passed, 0 failed
Integration Tests: 4 passed, 0 failed
Total:            12 passed, 0 failed
```

## Future Test Plans

- [ ] Frontend tests with Vitest
- [ ] E2E tests with Playwright
- [ ] Performance benchmarks
- [ ] Coverage reports in CI
- [ ] Mutation testing

## Common Issues

### Issue: Test fails with "database locked"
**Solution**: Use in-memory databases or unique temp files

### Issue: Date-based tests fail intermittently
**Solution**: Use fixed dates in tests, not `Utc::now()`

### Issue: Tests pass locally but fail in CI
**Solution**: Check for platform-specific code or timezone issues

## Manual Testing Complement

While automated tests are comprehensive, also perform manual testing for:

1. UI interactions
2. Clipboard monitoring
3. Global hotkeys
4. Window behavior
5. macOS-specific features

See `TESTING_CLEANUP.md` for manual testing procedures.

## Resources

- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Cargo Test Documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [SQLite Testing](https://www.sqlite.org/testing.html)
