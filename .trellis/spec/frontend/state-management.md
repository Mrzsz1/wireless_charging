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

## Scenario: Contextual Research Trail State

### Contracts

- The active anchor is one of `page`, `question`, `search`, or idle. Page anchors are published only after `getPage` succeeds; question anchors are published on submit/history-open, never on each draft keystroke; search anchors use a 350 ms debounce and clear below two characters.
- Every research-trail request uses a latest-request guard. Repository/context changes invalidate earlier success and failure responses.
- Pins use `desktop.research-trail-pins.v1` and are isolated by repository path plus backend `contextKey`. Corrupt JSON falls back to an empty versioned store.
- Pins lead the visible list but do not mutate automatic rank. Duplicate identity is `kind:id`.
- `GraphView.targetNodeId` is an explicit navigation target and is reconciled against the returned neighborhood.

### Required tests

- Node tests cover corrupt pin data, repository/context isolation, and pin/rank deduplication.
- Strict GUI E2E must open the panel and prove a library search produces an anchor and at least one auditable result.

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

## Scenario: Transient Secrets and Derived List Pagination

### Contracts

- A search-provider API Key draft lives only in the owning settings component. Saving clears the draft; reloading receives provider status only and never reconstructs or displays the saved value.
- Blank drafts do not call the save command. Clearing an existing credential is a separate, explicit action.
- List pagination is derived from the final filtered and sorted array. It must not alter backend result limits, BM25 order, catalog order, or the source collection.
- Search text, type/year/status filters, sort order, or page size changes reset the page to one. If a refreshed result set makes the current page invalid, clamp it to the last valid page.
- Empty results use the normal empty state and report zero items; do not render `1 / 0` as a valid page.
- Page-size options are shared constants (`10`, `20`, `50`), page controls are keyboard-accessible, and boundary actions are disabled rather than silently wrapping.

### Required tests

- Pure helper tests cover 66 items at size 10, the six-item last page, empty input, out-of-range pages, all supported sizes, and bounded page-number windows.
- Structural UI tests verify that credential inputs use `type="password"` by default, settings owns the automation editor, and the ingestion page contains only a settings link.
- Strict GUI E2E verifies page navigation and both target viewport sizes.

## Scenario: Repository-Scoped Answer Provider Settings

### 1. Scope / Trigger

This contract applies to AI answer-engine selection and connection state. Settings owns all editable provider configuration; `AskView` consumes status and routes users to Settings instead of maintaining a second draft/modal.

### 2. Signatures

- `AnswerProvider = 'codex-subscription' | 'compatible-api' | 'offline-evidence'`
- `getQaSettings(): Promise<QaSettings>`
- `saveQaSettings(settings: QaSettings): Promise<QaSettings>`
- `getCodexSubscriptionStatus(): Promise<CodexSubscriptionStatus>`
- `SettingsView({ repositoryPath, focusSection? })`
- `AskView({ repositoryPath, onOpenSettings })`

### 3. Contracts

- `SettingsView` is the only editor for provider selection, Codex login/status/model override, and compatible API fields.
- Codex status is machine-global and visible without a repository. Provider preference and API settings are repository-scoped and cannot be saved until a repository is selected.
- `AskView` shows the current provider label/readiness and calls `onOpenSettings`; it contains no endpoint, model, API environment, or temperature draft.
- Opening from Ask routes to `focusSection='qa-engine-settings'` and scrolls the existing settings page section into view.
- Initial provider controls remain disabled while asynchronous settings load. `data-loaded` changes to `true` only after the server snapshot has replaced defaults, preventing a late load from overwriting a user click.

### 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| No repository selected | Show Codex status; disable repository-scoped saving/edit fields |
| Initial settings request is pending | Disable provider tabs and expose `data-loaded=false` |
| Settings request fails | Show a dismissible error; do not claim values were saved |
| Codex not authenticated | Show login action and diagnostic, never a fake ready badge |
| Ask settings button clicked | Navigate to the global Settings card, not an Ask modal |

### 5. Good/Base/Bad Cases

- **Good**: Wait for `data-loaded=true`, select a provider, save it, leave/re-enter, and observe the same repository-scoped value.
- **Base**: Open Settings without a repository and inspect Codex readiness while save remains disabled.
- **Bad**: Render provider tabs from defaults as immediately interactive; the asynchronous load can revert a just-clicked selection.

### 6. Tests Required

- Structural Node tests assert all three provider editors live in `SettingsView` and the Ask modal/state no longer exists.
- Type/DTO tests reject secret-shaped Codex status fields.
- Strict GUI waits for `data-loaded=true`, switches all three provider panes, follows Ask-to-Settings navigation, and verifies 1366×768 plus 1920×1080.
- Run `npm run test:qa-settings`, `npm run build`, `npm run verify`, and `npm run verify:p5`.

### 7. Wrong vs Correct

#### Wrong

```tsx
const [settingsOpen, setSettingsOpen] = useState(false)
return <AskView><LunaSettingsModal /></AskView>
```

#### Correct

```tsx
<AskView onOpenSettings={() => openSettings('qa-engine-settings')} />
<SettingsView focusSection={settingsFocusSection} />
```
