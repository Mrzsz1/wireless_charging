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

### Fixed right-side context rail

`ResearchTrailPanel` is the last flex item in `.app-body`. Its collapsed state must remain a real, fixed-width rail at the far right instead of an absolutely positioned overlay.

```tsx
<aside className="context-collapsed-rail" data-testid="research-trail-rail">
  <button data-testid="trail-reopen" aria-label="展开研究脉络" />
</aside>
```

For dense views such as QA:

- entering the view collapses the global research trail by default;
- the user may explicitly reopen it;
- while it is open, the view must reduce or hide duplicated internal context columns rather than overflow beneath the global panel;
- GUI E2E must assert the rail touches the right edge and the main content ends before the rail begins.

Do not use `position: absolute` for the collapsed rail because it overlays page content and makes viewport geometry assertions unreliable.

### Chinese page headings

Page and section headings use the Chinese title directly. Do not add uppercase English eyebrow labels above Chinese headings. English technical terms remain valid inside content, metadata, model names, and source titles.

---

## Accessibility

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

---

## Common Mistakes

<!-- Component-related mistakes your team has made -->

(To be filled by the team)
