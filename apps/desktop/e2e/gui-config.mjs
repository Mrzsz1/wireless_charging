import { existsSync, statSync } from 'node:fs'
import { homedir } from 'node:os'
import { dirname, join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const e2eDirectory = dirname(fileURLToPath(import.meta.url))
const defaultDesktopRoot = resolve(e2eDirectory, '..')

function uniquePaths(paths) {
  return [...new Set(paths.map((value) => resolve(value)))]
}

function isFile(path) {
  try {
    return statSync(path).isFile()
  } catch {
    return false
  }
}

function executableName(platform) {
  return platform === 'win32' ? 'app.exe' : 'app'
}

/**
 * Return the desktop roots used when the command is launched from either the
 * repository root or apps/desktop. The module location is always included so
 * npm scripts and direct node invocations behave identically.
 */
export function desktopRoots({ cwd = process.cwd(), desktopRoot = defaultDesktopRoot } = {}) {
  return uniquePaths([
    desktopRoot,
    cwd,
    join(cwd, 'apps', 'desktop'),
  ])
}

export function appCandidates({ cwd = process.cwd(), desktopRoot = defaultDesktopRoot, platform = process.platform } = {}) {
  const name = executableName(platform)
  const candidates = []
  for (const root of desktopRoots({ cwd, desktopRoot })) {
    candidates.push(
      join(root, 'src-tauri', 'target', 'release', name),
      join(root, 'src-tauri', 'target', 'debug', name),
    )
  }
  return uniquePaths(candidates)
}

/**
 * Resolve the Tauri application without changing the user's environment.
 * An explicit TAURI_APP_PATH is authoritative: an invalid override is
 * reported instead of silently selecting a different build.
 */
export function resolveAppPath({ env = process.env, cwd = process.cwd(), desktopRoot = defaultDesktopRoot, platform = process.platform } = {}) {
  const override = (env.TAURI_APP_PATH ?? '').trim()
  if (override) {
    const path = resolve(cwd, override)
    return {
      path: isFile(path) ? path : null,
      explicit: true,
      source: 'TAURI_APP_PATH',
      requested: path,
      candidates: [path],
    }
  }

  const candidates = appCandidates({ cwd, desktopRoot, platform })
  const path = candidates.find(isFile) ?? null
  return {
    path,
    explicit: false,
    source: path ? 'auto' : null,
    requested: null,
    candidates,
  }
}

function cargoDriverCandidate({ env = process.env, platform = process.platform, home = homedir() } = {}) {
  const cargoHome = (env.CARGO_HOME ?? '').trim() || join(home, '.cargo')
  const suffix = platform === 'win32' ? '.exe' : ''
  return join(cargoHome, 'bin', `tauri-driver${suffix}`)
}

/**
 * Resolve a driver command. A command name is deliberately retained when no
 * Cargo-bin file exists so the operating system can search PATH at probe time.
 */
export function resolveDriver({ env = process.env, platform = process.platform, home = homedir(), explicit = null } = {}) {
  const requested = explicit ?? (env.TAURI_DRIVER ?? '').trim()
  if (requested) {
    return {
      executable: requested,
      explicit: true,
      source: 'TAURI_DRIVER',
      candidates: [requested],
    }
  }

  const cargoCandidate = cargoDriverCandidate({ env, platform, home })
  if (isFile(cargoCandidate)) {
    return {
      executable: cargoCandidate,
      explicit: false,
      source: 'CARGO_HOME',
      candidates: [cargoCandidate, 'tauri-driver'],
    }
  }

  return {
    executable: 'tauri-driver',
    explicit: false,
    source: 'PATH',
    candidates: ['tauri-driver', cargoCandidate],
  }
}

export function formatCandidates(candidates) {
  return candidates.map((candidate) => `  - ${candidate}`).join('\n')
}
