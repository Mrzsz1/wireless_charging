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
