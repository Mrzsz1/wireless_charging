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

<!-- How styles are applied (CSS modules, styled-components, Tailwind, etc.) -->

(To be filled by the team)

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
