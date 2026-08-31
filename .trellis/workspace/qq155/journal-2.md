# Journal - qq155 (Part 2)

> Continuation from `journal-1.md` (archived at ~2000 lines)
> Started: 2026-08-30

---



## Session 59: P1-3 Query Planner Provider reliability diagnosis

**Date**: 2026-08-30
**Task**: P1-3 Query Planner Provider reliability diagnosis
**Branch**: `master`

### Summary

Added Report v5 Planner diagnostics, strict Research/Exploratory gates, stable failure taxonomy and lifecycle logs; both permitted real Research runs reproduced provider_exit, so no speculative production patch was made and the strict final status is FAIL.

### Main Changes

(Add details)

### Git Commits

| Hash | Message |
|------|---------|
| `17c1c37` | (see git log) |
| `0bcccbb` | (see git log) |
| `8ce5b3f` | (see git log) |

### Testing

- [OK] (Add test results)

### Status

[WARN] **PARTIAL-BLOCKED**

### Next Steps

- P1-1D Research final grounding repair reliability


## Session 60: P1-3F Codex Planner provider exit diagnosis

**Date**: 2026-08-30
**Task**: P1-3F Codex Planner provider exit diagnosis
**Branch**: `master`

### Summary

Preserved and classified Codex JSONL terminal failures, added repository-external raw diagnostics and isolated A/B/C probes, and proved Probe A is blocked by an external transport failure. B/C and final Research were correctly not run; Planner Schema/input/timeout/budgets and answer safety behavior remain unchanged.

### Main Changes

(Add details)

### Git Commits

| Hash | Message |
|------|---------|
| `c7d6bd6` | (see git log) |
| `f3c918a` | (see git log) |
| `324704f` | (see git log) |
| `4c8c3c0` | (see git log) |
| `d2a946e` | (see git log) |
| `1d3d46f` | (see git log) |

### Testing

- [OK] (Add test results)

### Status

[OK] **Completed**

### Next Steps

- None - task complete


## Session 61: P1-3S v2 Provider Schema 与 Codex 代理修复

**Date**: 2026-08-31
**Task**: P1-3S v2 Provider Schema 与 Codex 代理修复
**Branch**: `master`

### Summary

仅移除 Provider Schema 的 uniqueItems 并保留本地严格校验；临时 7890 下 Probe B/C 与真实 Research 通过后接入子进程默认代理。清空 Shell 代理后 A/B/C 通过，最终单次 Research 因独立 citation_validation_failed 门禁记录为 PARTIAL-BLOCKED，未重跑或修改被禁止层。

### Main Changes

(Add details)

### Git Commits

| Hash | Message |
|------|---------|
| `601f23a` | (see git log) |
| `9e8c20c` | (see git log) |
| `24537a0` | (see git log) |
| `44cf680` | (see git log) |
| `5bfd9b9` | (see git log) |
| `d4fc743` | (see git log) |
| `5fdb861` | (see git log) |
| `74e58e8` | (see git log) |
| `794ac42` | (see git log) |
| `331e731` | (see git log) |
| `dc45a40` | (see git log) |

### Testing

- [OK] (Add test results)

### Status

[OK] **Completed**

### Next Steps

- None - task complete


## Session 62: P1-1D Research Grounding Repair Reliability

**Date**: 2026-08-31
**Task**: P1-1D Research Grounding Repair Reliability
**Branch**: `master`

### Summary

Reproduced the legacy global replacen collision, replaced claim repair with fail-closed ordered UTF-8 source-span reconstruction, added repair projection invariants/audits/logging and R1-R12 regressions, then fixed a second inline notice-boundary drift found by the single permitted real Research run. Deterministic quality is green; status remains PARTIAL-BLOCKED because the post-fix real close gate was not rerun.

### Main Changes

(Add details)

### Git Commits

| Hash | Message |
|------|---------|
| `d519909` | (see git log) |
| `ed5575e` | (see git log) |
| `9f6b038` | (see git log) |
| `5f4eee9` | (see git log) |
| `bbce464` | (see git log) |
| `7d9377f` | (see git log) |
| `69582f3` | (see git log) |

### Testing

- [OK] (Add test results)

### Status

[OK] **Completed**

### Next Steps

- None - task complete


## Session 63: P1-1D2 Post-Fix Real Research Close Gate

**Date**: 2026-08-31
**Task**: P1-1D2 Post-Fix Real Research Close Gate
**Branch**: `master`

### Summary

Revalidated every deterministic P1-1D gate and ran exactly one post-fix real-research-improvement case. Planner and Semantic succeeded, repair projection completed 6 source-span operations with 3 safe repairs, Final Grounding was 3/3/0 with coverage 1.0 and valid visible projection, and persistence succeeded. The selected Research close gate is PASS; no production code or forbidden subsystem changed.

### Main Changes

(Add details)

### Git Commits

| Hash | Message |
|------|---------|
| `86545e2` | (see git log) |
| `60e92ae` | (see git log) |
| `4dc3db5` | (see git log) |
| `98c4e70` | (see git log) |

### Testing

- [OK] (Add test results)

### Status

[OK] **Completed**

### Next Steps

- None - task complete


## Session 64: 完成 P1-5R Zero-Evidence Closeout Hardening

**Date**: 2026-08-31
**Task**: 完成 P1-5R Zero-Evidence Closeout Hardening
**Branch**: `master`

### Summary

统一 support-eligible evidence 判定；按 required facet ID 计算覆盖；拆分用户指代历史与可信事实历史；完成 Rust、Frozen 22/22、前端 6/6 与构建验证；记录 P1-5R closeout，未增加真实 Provider 或 Heldout 运行。

### Main Changes

(Add details)

### Git Commits

| Hash | Message |
|------|---------|
| `d7a290c8187dcb9d727e3feef6fb822aef5fbc1d` | (see git log) |
| `6f7048001018e7ab54dfbb258dd7a5533e3e3d83` | (see git log) |
| `696dd369e239662aaf3cda95657fa5ef568a4dc5` | (see git log) |
| `95353b87779986a76c51d8ad49cc9124f497abc4` | (see git log) |
| `ce0e1aa255badc87958b7566ea2fc17d45128a6a` | (see git log) |

### Testing

- [OK] (Add test results)

### Status

[OK] **Completed**

### Next Steps

- None - task complete
