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

## Scenario: DPI-Safe Desktop Window Restoration

### 1. Scope / Trigger

This contract applies whenever the Tauri desktop shell persists or restores
window geometry. It prevents a window from remaining alive only in the Windows
taskbar after DPI, resolution, taskbar, or monitor topology changes.

### 2. Signatures

- `parsePersistedWindowState(raw): PersistedWindowState | null`
- `resolveWindowPlacement(state, monitors, primary, fallbackSize): WindowPlacement`
- `createPersistedWindowState(rect, maximized): PersistedWindowState`
- Storage keys: `desktop.window-state.v3` and read-only migration input
  `desktop.window-state.v2`.

### 3. Contracts

- Persisted `x`, `y`, `width`, and `height` are physical pixels and must be
  restored with `PhysicalPosition` and `PhysicalSize`; never reinterpret them
  as logical pixels.
- Restore against current monitor work areas. A saved rectangle that intersects
  a monitor is fitted into that work area; a fully off-screen rectangle is
  centered in the primary work area.
- Negative coordinates are valid when they intersect a real monitor.
- Minimized geometry is never persisted. Maximized state preserves the latest
  normal rectangle and only updates the `maximized` flag.
- Startup finishes with best-effort `unminimize`, `show`, and `setFocus` after
  placement; `center` is the fallback when placement APIs fail.

### 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| JSON is malformed or dimensions are non-finite/negative/too small | Ignore it and use the centered fallback |
| Saved rectangle intersects a current monitor | Clamp size and position to that work area |
| Saved rectangle intersects no current monitor | Center in the primary work area |
| Primary monitor lookup fails but monitors exist | Use the first current monitor |
| No monitor API result is available | Use Tauri `center()` and still show/focus |
| Window is minimized during a move/resize event | Keep the previous normal rectangle |

### 5. Good/Base/Bad Cases

- **Good**: A v2 rectangle at `x=-2858` on a removed monitor migrates to v3 and
  opens centered on the current primary monitor.
- **Base**: A normal single-monitor rectangle is restored without changing its
  visible position.
- **Bad**: Values returned by `outerPosition()` are restored with
  `LogicalPosition`; high-DPI scaling can move the window completely off-screen.

### 6. Tests Required (with assertion points)

- Node unit tests cover removed-monitor coordinates, valid negative-coordinate
  monitors, resolution shrink, corrupt values, and DPI-scaled fallback sizes.
- Structural verification asserts required Tauri window permissions and the
  visibility-recovery module wiring.
- Strict GUI E2E must read the native window rectangle and assert positive
  intersection area with the active monitor work area before navigation tests.
- Installer smoke must terminate the complete launched process tree before
  uninstalling and assert that the process exited.

### 7. Wrong vs Correct

#### Wrong

```tsx
const position = await appWindow.outerPosition()
await appWindow.setPosition(new LogicalPosition(position.x, position.y))
```

#### Correct

```tsx
const placement = resolveWindowPlacement(saved, monitors, primary, fallback)
await appWindow.setSize(new PhysicalSize(placement.rect.width, placement.rect.height))
await appWindow.setPosition(new PhysicalPosition(placement.rect.x, placement.rect.y))
await appWindow.unminimize()
await appWindow.show()
await appWindow.setFocus()
```
