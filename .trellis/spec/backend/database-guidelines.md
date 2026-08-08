# Database Guidelines

> Database patterns and conventions for this project.

---

## Overview

<!--
Document your project's database conventions here.

Questions to answer:
- What ORM/query library do you use?
- How are migrations managed?
- What are the naming conventions for tables/columns?
- How do you handle transactions?
-->

(To be filled by the team)

## Scenario: Repository-Scoped Derived Index

### 1. Scope / Trigger

This contract applies whenever the desktop client opens or switches a local
knowledge repository while reusing the shared `knowledge.db` cache. It was
added after a cross-repository index leak was found in `open_repository_state`.

### 2. Signatures

- `fn canonical_repository_root(root: &Path) -> Result<PathBuf, String>`
- `fn repository_identity(root: &Path) -> String`
- `fn ensure_repository_index(connection: &mut Connection, root: &Path) -> Result<IndexStats, String>`
- SQLite table: `repository_metadata(key TEXT PRIMARY KEY, value TEXT NOT NULL)`
- Metadata key: `knowledge_index_repository_id`

### 3. Contracts

- The root is canonicalized before validation, state replacement, watcher
  creation, and `repository.json` persistence.
- Identity uses `/` separators, trims trailing `/`, and is case-insensitive on
  Windows. It must match the canonical root used by the active state.
- A missing or mismatched metadata value rebuilds only derived knowledge
  tables (`pages`, FTS tables, `wikilinks`, `books`, and `book_chapters`).
- Chat sessions, chat messages/evidence, compile history, and `app_settings`
  are not deleted by an index rebuild.
- The metadata value is written only after a successful rebuild. A rebuild
  error leaves the old identity and `RepositoryState` untouched.

### 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| Root is missing or lacks `AGENTS.md`, `wiki`, or `schema` | Return an error before opening state |
| Stored identity equals canonical identity | Reuse derived rows and return current stats |
| Stored identity is missing | Rebuild derived rows, then write identity |
| Stored identity differs | Rebuild derived rows, then write the new identity |
| Rebuild fails | Return error; do not replace active state or write new identity |
| Metadata table is absent in a legacy DB | `db_schema` creates it before identity validation |

### 5. Good/Base/Bad Cases

- **Good**: DB indexed for repository A is opened for B; pages/FTS/books now
  contain only B while A's chat session remains queryable by its repository ID.
- **Base**: The same canonical root is opened again; no unnecessary rebuild is
  performed.
- **Bad**: Decide repository validity from `pages` count alone. A non-empty
  count can belong to a different repository and must never authorize reuse.

### 6. Tests Required (with assertion points)

- Rust identity normalization test: separators, trailing slash, Windows case.
- Rust switch test: A DB + A identity opened for B removes A derived rows and
  indexes B rows.
- Preservation assertions: a chat session and an `app_settings` row survive
  the switch.
- Failure test: a failed rebuild does not update metadata or active state.
- Full Rust suite and `cargo clippy --all-targets --all-features -- -D warnings`.

### 7. Wrong vs Correct

#### Wrong

```rust
let indexed_pages = connection.query_row("SELECT COUNT(*) FROM pages", [], ...)?;
if indexed_pages > 0 { reuse_the_cache(); }
```

#### Correct

```rust
let identity = repository_identity(&canonical_root);
if read_repository_identity(connection)?.as_deref() == Some(identity.as_str()) {
    current_index_stats(connection, &canonical_root)
} else {
    let stats = rebuild_connection(connection, &canonical_root)?;
    write_repository_identity(connection, &identity)?;
    Ok(stats)
}
```

---

## Query Patterns

<!-- How should queries be written? Batch operations? -->

(To be filled by the team)

---

## Migrations

<!-- How to create and run migrations -->

(To be filled by the team)

---

## Naming Conventions

<!-- Table names, column names, index names -->

(To be filled by the team)

---

## Common Mistakes

<!-- Database-related mistakes your team has made -->

(To be filled by the team)
