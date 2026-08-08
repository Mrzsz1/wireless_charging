# State Management

> How state is managed in this project.

---

## Overview

<!--
Document your project's state management conventions here.

Questions to answer:
- What state management solution do you use?
- How is local vs global state decided?
- How do you handle server state?
- What are the patterns for derived state?
-->

(To be filled by the team)

## Scenario: Idempotent Multi-Channel Research State

### 1. Scope / Trigger

This contract applies to state that can be updated by both a Tauri stream event
and the promise returned by the same command. It also covers navigation targets
that cross `AskView` into `CoreBooksView`, and external Graphify refreshes.

### 2. Signatures

- `claimCompletion(ledger, repositoryPath, requestId): boolean`
- `mergeCompletedMessages(messages, result): ChatMessage[]`
- `BookTarget = { bookId: string; chapterId: string }`
- `matchesBookTarget(chapter, target): boolean`
- `GraphView({ onOpenPage, refreshVersion? })`
- `nextGraphRefreshVersion(version, graphRefresh): number`

### 3. Contracts

- A completion result is claimed by `requestId` before applying message,
  evidence, waterline, phase, or session-refresh side effects.
- The completion ledger is reset when `repositoryPath` changes.
- Completed messages replace local placeholders and existing messages with the
  same persisted IDs; one request produces exactly one user/assistant pair.
- `AskView` forwards both `bookId` and `chapterId`; `App` stores them as a
  `BookTarget` and passes the target to `CoreBooksView`.
- Full chapter IDs (`book-id:chapter-id`) and short IDs (`chapter-id`) match,
  but the selected book must also match.
- `graphRefreshVersion` increments only when the watcher status has
  `graphRefresh === true`; `GraphView` reloads while preserving the query and
  drops stale selection/path state.

### 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| Same completion request arrives twice | Apply the first result only |
| Completion arrives for a new repository | Reset ledger and allow its request ID |
| Book target has wrong book ID | Do not select a chapter |
| Book target uses full or short chapter ID | Select the matching chapter once |
| Chapter list response is stale | Ignore it using the request sequence guard |
| Graph status has `graphRefresh: false` | Do not increment refresh version |
| Refreshed graph omits selected/path IDs | Clear only stale selection/path state |

### 5. Good/Base/Bad Cases

- **Good**: `completed` event and `askLuna` return share `requestId`; the
  visible conversation has one persisted pair and one session refresh.
- **Base**: A user opens a book evidence item and the target chapter loads after
  the chapter list; the target request wins over the default first chapter.
- **Bad**: Pass `onOpenBook={() => activateView('books')}` or render
  `target={null}`. The UI navigates to the book page but loses evidence
  location.

### 6. Tests Required (with assertion points)

- Node built-in `node:test` coverage for completion claim idempotence,
  placeholder merge, full/short chapter IDs, and graph version projection.
- Assert a second completion does not change message count or trigger a second
  side-effect path.
- Assert a stale chapter response cannot replace the current book/chapter.
- Assert `graphRefresh: false` leaves the version unchanged and stale graph
  selection/path are reconciled.
- Run `npm run test:p1 --prefix apps/desktop` and `npm run build --prefix apps/desktop`.

### 7. Wrong vs Correct

#### Wrong

```tsx
if (event.type === 'completed') applyCompleted(event.payload.result)
const result = await askLuna(request, handleEvent)
applyCompleted(result)
```

#### Correct

```tsx
if (!claimCompletion(ledger.current, repositoryPath, result.requestId)) return
setMessages((current) => mergeCompletedMessages(current, result))
```

---

## State Categories

<!-- Local state, global state, server state, URL state -->

(To be filled by the team)

---

## When to Use Global State

<!-- Criteria for promoting state to global -->

(To be filled by the team)

---

## Server State

<!-- How server data is cached and synchronized -->

(To be filled by the team)

---

## Common Mistakes

<!-- State management mistakes your team has made -->

(To be filled by the team)
