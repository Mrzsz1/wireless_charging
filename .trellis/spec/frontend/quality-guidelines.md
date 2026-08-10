# Quality Guidelines

> Code quality standards for frontend development.

---

## Overview

<!--
Document your project's quality standards here.

Questions to answer:
- What patterns are forbidden?
- What linting rules do you enforce?
- What are your testing requirements?
- What code review standards apply?
-->

(To be filled by the team)

---

## Forbidden Patterns

<!-- Patterns that should never be used and why -->

(To be filled by the team)

---

## Required Patterns

<!-- Patterns that must always be used -->

(To be filled by the team)

---

## Testing Requirements

<!-- What level of testing is expected -->

(To be filled by the team)

---

## Code Review Checklist

<!-- What reviewers should check -->

(To be filled by the team)

## GUI E2E environment contract

### 1. Scope / Trigger

This contract applies to `apps/desktop/e2e/gui-smoke.mjs` and its configuration resolver whenever a GUI E2E run needs a Tauri executable or WebDriver.

### 2. Signatures

- `resolveAppPath({ env, cwd, desktopRoot, platform }) -> { path, explicit, source, requested, candidates }`
- `resolveDriver({ env, platform, home, explicit }) -> { executable, explicit, source, candidates }`
- `TAURI_APP_PATH`, `TAURI_DRIVER`, and `TAURI_NATIVE_DRIVER` are optional environment inputs.

### 3. Contracts

- `TAURI_APP_PATH` is authoritative when non-empty; it must point to a file.
- Without an override, app discovery checks bundled `release` before `debug` targets.
- Drivers resolve from the explicit override, PATH, then `$CARGO_HOME/bin/tauri-driver(.exe)`.
- Missing prerequisites are an exit-0 `SKIP` in normal mode and a non-zero result in strict mode.

### 4. Validation & Error Matrix

| Condition | Normal mode | Strict mode |
| --- | --- | --- |
| Explicit app path is missing | `SKIP` with requested path | exit 2 with requested path |
| No release/debug app exists | `SKIP` with all candidates | exit 2 with all candidates |
| Driver probe fails | `SKIP` with install/native-driver hint | exit 2 with install/native-driver hint |
| Driver starts but GUI assertion fails | fail the test | fail the test |

### 5. Good/Base/Bad Cases

- Good: a freshly built release app is found without setting `TAURI_APP_PATH`.
- Base: a user-provided app/driver path is used unchanged.
- Bad: a debug executable that only points to an unavailable Vite dev server is selected while a release bundle exists; release must win.

### 6. Tests Required

- `tests/gui-e2e-config.test.mjs` asserts release-before-debug, explicit-path authority, and Cargo-bin driver discovery.
- `e2e:gui:strict` asserts the real sidebar, navigation, books, graph, QA, compile, settings, and viewport contracts.
- A missing native driver must produce normal exit 0 and strict exit 2 with an actionable hint.

### 7. Wrong vs Correct

#### Wrong

```js
const app = process.env.TAURI_APP_PATH
if (!app) process.exit(0)
```

#### Correct

```js
const app = resolveAppPath().path
if (!app) finishUnavailable('GUI E2E SKIP: build the app or set TAURI_APP_PATH')
```

### Common Mistake: tauri-driver without the native Edge driver

`tauri-driver` can be installed and executable while still exiting immediately because Windows cannot find a matching `msedgedriver.exe`. Keep the native driver on PATH or set `TAURI_NATIVE_DRIVER`; the smoke script reports this as an environment skip rather than an opaque stack trace.

## Windows release version synchronization contract

- A desktop release version is one atomic value across `package.json`, the root
  package entry in `package-lock.json`, `src-tauri/Cargo.toml`, the `app`
  package in `Cargo.lock`, and `src-tauri/tauri.conf.json`.
- Release-related fixtures must move with the current version: the no-update
  manifest equals the current version and the update fixture uses the next
  patch version. `verify-config.mjs` must assert the same current version.
- Run the configuration and updater fixture validators before `tauri build`.
  The build is accepted only when both MSI and NSIS bundles contain the target
  version in their file names.
- After a silent NSIS installation, verify the uninstall registry
  `DisplayVersion`, the installed executable's product version, and a live,
  responding main window. A successful installer exit code alone is not
  sufficient evidence of a usable release.
- Generated `target/` bundles are local artifacts and are not added to Git;
  commit version declarations, release notes, and the Trellis task only.
