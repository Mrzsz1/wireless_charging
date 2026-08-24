# Component Guidelines

> How components are built in this project.

---

## Overview

<!--
Document your project's component conventions here.

Questions to answer:
- What component patterns do you use?
- How are props defined?
- How do you handle composition?
- What accessibility standards apply?
-->

(To be filled by the team)

---

## Component Structure

<!-- Standard structure of a component file -->

(To be filled by the team)

---

## Props Conventions

<!-- How props should be defined and typed -->

(To be filled by the team)

---

## Styling Patterns

### Single-view desktop navigation

The desktop shell is a single-view workspace. Sidebar and titlebar navigation replace the current main view directly; do not render an editor-style work-tab strip above page content.

```tsx
<main className={`main-workspace ${view === 'qa' ? 'qa-active' : ''}`}>
  {renderContent()}
</main>
```

Keep persisted navigation identifiers only when they are needed for backward-compatible state or scroll restoration. They must not reintroduce visible tabs or require users to close previous views.

### Titlebar-owned research trail control

`ResearchTrailPanel` is the last flex item in `.app-body` only while it is open. The single collapse/reopen control lives in the titlebar between `.titlebar-drag-region` and `.window-actions`.

```tsx
<div className="titlebar-drag-region" data-tauri-drag-region />
<button data-testid="trail-toggle" aria-pressed={contextOpen} />
<div className="window-actions">...</div>
```

When closed, `ResearchTrailPanel` returns `null`. Do not retain a fixed-width rail, content-edge floating button, or duplicate collapse button in the panel header. This lets the main workspace consume the full released width and keeps the window-level control in one predictable location.

For dense views such as QA:

- entering the view collapses the global research trail by default;
- the user may explicitly reopen it;
- while it is open, the view must reduce or hide duplicated internal context columns rather than overflow beneath the global panel;
- GUI E2E must assert the closed main content reaches the app-body right edge and the toggle remains immediately before the native window controls.

Do not reintroduce an absolute overlay or collapsed rail: both obscure content geometry and split one state across multiple controls.

### Chinese page headings

Page and section headings use the Chinese title directly. Do not add uppercase English eyebrow labels above Chinese headings. English technical terms remain valid inside content, metadata, model names, and source titles.

### Transient application notifications

Shell-level status messages use `components/AppToast.tsx`; they must not render inside `.main-workspace` because an inline banner shifts page content and scroll geometry.

```tsx
{notice.message && <AppToast
  key={notice.id}
  message={notice.message}
  contextOpen={contextOpen}
  onDismiss={() => setNoticeState((current) =>
    current.id === notice.id ? { ...current, message: '' } : current
  )}
/>}
```

Interaction contract:

- every publication increments an ID, including repeated text, so the lifecycle restarts;
- the toast holds for 3600 ms, fades for 450 ms, then clears only its own ID;
- pointer hover pauses dismissal and pointer leave starts a fresh hold interval;
- the component keeps the latest `onDismiss` callback in a ref so parent re-renders do not reset its timer effect;
- the root uses `role="status"`, `aria-live="polite"`, and an explicitly labelled close button;
- the toast is fixed below the native titlebar and offsets left when the full research-trail panel is open;
- preserve inline `.notice` styling for view-local errors or results that belong to page layout.

Strict GUI E2E must verify fixed positioning outside `.main-workspace`, hover pause, automatic removal, and the accessibility attributes.

---

## Accessibility

### QA conditional grid and processing status

`AskView` owns four semantic grid rows: heading, optional error, scrollable messages, and composer. Assign each child an explicit `grid-row`; never depend on CSS auto-placement because removing the conditional error node would otherwise place the composer in the flexible messages row. The textarea starts at three rows, grows only to its documented cap, then scrolls internally.

During an active QA request, render an event-driven processing chain and elapsed time. `retrieval_started`, `retrieval_completed`, first token, and `validation_started` are authoritative transitions for local retrieval, evidence organization, model Thinking/generation, and citation/completeness validation. This chain is an operational audit projection, not model chain-of-thought. Clear its monotonic timer and transient state on completion, failure, cancellation, and repository change; use `aria-live="polite"` and respect reduced motion.

Project QA progress from one deterministic active-step index: earlier steps are `done`, that index alone is `active`, and later steps are `waiting`. The processing card must remain visibly alive through a continuous rail, low-amplitude pulse, decorative ellipsis, and elapsed time, but none of these may imply fabricated completion percentage or internal reasoning. Put `role="status"`/`aria-live="polite"` on the real stage title rather than the per-second timer, mark animated decoration `aria-hidden`, switch to a streaming cursor only after the first token, and disable every loader animation under `prefers-reduced-motion: reduce`.

The current-stage leading indicator and the unique active-step indicator are both circular rotating loaders owned by `AskView` through a scoped animation class; do not rely on a global `.spin` selector or replace either one with a static glyph or equalizer bars. Keep both decorative with `aria-hidden` and use one rotation cadence.

`AskView`, `LiteratureIngestView`, and `CompileCenterView` are persistent long-running workspaces. `App` mounts each exactly once and toggles its wrapper with the native `hidden` attribute; navigation must not conditionally unmount them because that would detach Channel callbacks and discard timers, request IDs, logs, and progress. Hidden wrappers must not participate in layout, focus, or the accessibility tree. Repository changes retain each component's own cancellation/reset boundary. The two essential circular QA busy indicators continue rotating at a slower cadence under reduced-motion; decorative rail, pulse, ellipsis, sheen, and cursor animations may stop.

For Codex subscription mode, the per-turn model and reasoning-effort controls live in the composer footer, not in a settings card or the chat header. Populate them from the local Codex list-visible model catalog, restrict the effort menu to the selected model's reported capabilities, and keep an “automatic” fallback when the catalog is missing. Persist selection as the next default but always send the current selection in the request snapshot. Use transparent blue-gray 10px controls, a 1px hover/focus border, keyboard labels, and horizontal overflow on narrow widths; do not add a floating selector panel or a second set of model controls in Settings.

The local semantic embedding model is provider-independent and therefore owns a separate Settings card rather than appearing inside Codex/API tabs. Its status must distinguish file presence from a successful 384-dimensional inference probe. “检查部署” is visibly offline; “下载/修复” is the explicit network action. When a cache path changes, expose separate “copy and switch” and “switch and redeploy” actions, state that the old directory is retained, and keep the card usable before a repository is selected.

During semantic model download/repair, render the Channel-owned progress projection immediately to the right of the action buttons. Show the current runtime/model/tokenizer/inference phase, safe file label, actual downloaded/total bytes, percentage, and bytes/second. Unknown totals use an indeterminate bar rather than a fabricated percentage. Preserve the last byte values when the command fails, retain completed state after the spinner stops, wrap the progress block below buttons on narrow widths, and disable its animation under reduced-motion preferences.

On the automatic literature page, readiness is scoped to the selected operation. Candidate preparation requires only discovery and download; automatic full ingest requires the composite full-ingest capability. Do not label the current preparation flow as restricted merely because compile, MinerU, or Graphify is unavailable. Active retrieval owns a local, visibly asymmetric rotating indicator and explicit running copy. The persistent run strip uses at least 12px status text, 11px action/log text, comfortable line height, `role="status"`, and a reduced-motion fallback.

`mixed` QA messages remain fully visible but use the amber citation-summary treatment to distinguish the explicitly labelled model-supplement section. The summary must continue to say that semantic entailment is not automatically checked.

Natural v2 answers stream as Markdown. After backend validation, their final `参考证据` links display the backend-owned short label (`来源类型 · 标题 · 小节`) rather than a raw filename or absolute path. Legacy inline tokens remain compact `[E#]` links. Both forms are keyboard-focusable, map deterministically into the shared eight-color palette, and activate only the registered evidence object; color is supplementary to visible text, focus state, and the accessible label.

When registered evidence has a `SourceLocator`, clicking it opens the internal, read-only `SourceDocumentView` instead of parsing a file path from rendered prose. The view uses `MarkdownReader.focus` to scroll/highlight the resolved heading or closest line and displays `matchedBy` plus any degradation reason. It may offer a deliberate “在文件夹中显示” action, but ordinary answer/evidence surfaces must not expose `sourcePath` or `markdownPath`. Full paths remain confined to backend payloads and copied audit data.

### Contextual help convention

Use `components/DelayedHelp.tsx` for optional page- or section-level explanatory copy that would otherwise create persistent visual noise. Do not hide field values, validation errors, security state, or action consequences in a tooltip.

```tsx
<div className="settings-title-row">
  <h2>文献自动化</h2>
  <DelayedHelp label="配置自动添加的启动与筛选规则。" />
</div>
```

Interaction contract:

- Pointer hover opens after 1000 ms; leaving before the deadline cancels it.
- Keyboard focus and click expose the explanation immediately.
- The trigger has an accessible label and the visible popup uses `role="tooltip"` with `aria-describedby`.
- The component clears its timer when the pointer leaves and when it unmounts.
- GUI E2E must assert both the pre-delay absence and post-delay presence of the tooltip.

Use direct text instead when the information is required to understand the current state or safely complete an action.

### Faceted library categories

The general library is a heterogeneous Wiki browser, so expose governed page types as one compact category strip instead of mixing internal type IDs into an undifferentiated list.

- Keep the type-to-label mapping and filter/count logic in a pure module shared by the view and tests.
- Use Chinese product labels. In particular, render `map` as “知识地图”; never expose the internal identifier `map` as the user-facing type.
- Apply category filtering after query/year/status/sort processing and before pagination. Reset to page 1 whenever the category changes.
- The dedicated methods route is not another facet: force `method` and hide the general category strip.
- Preserve unknown page types in “全部” and label them “其他页面” so schema drift is visible rather than silently discarded.
- Category buttons use `aria-pressed`, a visible keyboard focus state, stable test IDs, and horizontal overflow on narrow windows.
- GUI E2E must select “知识地图”, verify the current repository count, and assert every visible result carries `data-page-type="map"`.

---

## Common Mistakes

<!-- Component-related mistakes your team has made -->

(To be filled by the team)
