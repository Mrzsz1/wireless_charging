import { existsSync, readFileSync } from 'node:fs'

const source = readFileSync(new URL('../index.html', import.meta.url), 'utf8')
const built = readFileSync(new URL('../dist/index.html', import.meta.url), 'utf8')
const appSource = readFileSync(new URL('../src/App.tsx', import.meta.url), 'utf8')
const toastSource = readFileSync(new URL('../src/components/AppToast.tsx', import.meta.url), 'utf8')
const serviceSource = readFileSync(new URL('../src/services/desktop.ts', import.meta.url), 'utf8')
const pageSource = readFileSync(new URL('../src/features/pages/PageView.tsx', import.meta.url), 'utf8')
const markdownSource = readFileSync(new URL('../src/features/pages/MarkdownReader.tsx', import.meta.url), 'utf8')
const booksSource = readFileSync(new URL('../src/features/books/CoreBooksView.tsx', import.meta.url), 'utf8')
const graphSource = readFileSync(new URL('../src/features/graph/GraphView.tsx', import.meta.url), 'utf8')
const comparisonSource = readFileSync(new URL('../src/features/comparison/ComparisonView.tsx', import.meta.url), 'utf8')
const qaViewSource = readFileSync(new URL('../src/features/qa/AskView.tsx', import.meta.url), 'utf8')
const researchTrailSource = readFileSync(new URL('../src/features/research-trail/ResearchTrailPanel.tsx', import.meta.url), 'utf8')
const compileViewSource = readFileSync(new URL('../src/features/compile/CompileCenterView.tsx', import.meta.url), 'utf8')
const ingestViewSource = readFileSync(new URL('../src/features/ingest/LiteratureIngestView.tsx', import.meta.url), 'utf8')
const startupPromptSource = readFileSync(new URL('../src/features/ingest/StartupIngestPrompt.tsx', import.meta.url), 'utf8')
const libraryViewSource = readFileSync(new URL('../src/features/library/LibraryView.tsx', import.meta.url), 'utf8')
const settingsViewSource = readFileSync(new URL('../src/features/settings/SettingsView.tsx', import.meta.url), 'utf8')
const stylesSource = readFileSync(new URL('../src/styles.css', import.meta.url), 'utf8')
const rustSource = readFileSync(new URL('../src-tauri/src/lib.rs', import.meta.url), 'utf8')
const qaRustSource = readFileSync(new URL('../src-tauri/src/qa.rs', import.meta.url), 'utf8')
const researchTrailRustSource = readFileSync(new URL('../src-tauri/src/research_trail.rs', import.meta.url), 'utf8')
const compileRustSource = readFileSync(new URL('../src-tauri/src/compile_center.rs', import.meta.url), 'utf8')
const capabilities = JSON.parse(readFileSync(new URL('../src-tauri/capabilities/default.json', import.meta.url), 'utf8'))
const data = JSON.parse(readFileSync(new URL('../public/data/library.json', import.meta.url), 'utf8'))

const requiredWindowPermissions = [
  'core:window:allow-minimize',
  'core:window:allow-toggle-maximize',
  'core:window:allow-close',
  'core:window:allow-unmaximize',
  'core:window:allow-unminimize',
  'core:window:allow-show',
  'core:window:allow-set-focus',
  'core:window:allow-center',
]

const checks = [
  ['source root mount', source.includes('<div id="root"></div>')],
  ['source entry module', source.includes('/src/main.tsx')],
  ['built root mount', built.includes('<div id="root"></div>')],
  ['built JavaScript asset', /<script[^>]+assets\/index-[^>]+\.js/.test(built)],
  ['built stylesheet asset', /<link[^>]+assets\/index-[^>]+\.css/.test(built)],
  ['window control permissions', requiredWindowPermissions.every((permission) => capabilities.permissions?.includes(permission))],
  ['window visibility recovery', appSource.includes('resolveWindowPlacement') && appSource.includes('new PhysicalPosition') && appSource.includes('await appWindow.show()') && appSource.includes('await appWindow.unminimize()')],
  ['window controls outside drag region', appSource.includes('className="titlebar-drag-region"') && !appSource.includes('<header className="titlebar" data-tauri-drag-region')],
  ['single expandable sidebar', appSource.includes('className={`app-sidebar ${navCollapsed') && !appSource.includes('navigation-panel') && !appSource.includes('app-rail')],
  ['immersive toast lifecycle', appSource.includes('<AppToast') && !appSource.includes('className="notice"') && ['TOAST_HOLD_MS', 'TOAST_EXIT_MS', 'aria-live="polite"', "exiting ? 'exiting'"].every((name) => toastSource.includes(name))],
  ['immersive toast styling', stylesSource.includes('position: fixed') && stylesSource.includes('@keyframes app-toast-exit') && stylesSource.includes('.app-toast.context-open')],
  ['single-view navigation', !appSource.includes('<TabBar') && !existsSync(new URL('../src/components/TabBar.tsx', import.meta.url))],
  ['Chinese-only page headings', ![appSource, pageSource, booksSource, graphSource, comparisonSource, qaViewSource, compileViewSource, ingestViewSource, startupPromptSource, libraryViewSource, settingsViewSource].some((value) => value.includes('className="eyebrow"')) && !stylesSource.includes('.eyebrow')],
  ['collapsed research trail rail', researchTrailSource.includes('context-collapsed-rail') && stylesSource.includes('.context-collapsed-rail') && stylesSource.includes('align-items: center') && stylesSource.includes('.main-workspace.qa-active.context-visible')],
  ['stage 2 page commands', ['listPages', 'getPage', 'resolveWikilink', 'getBacklinks', 'openLocalPath'].every((name) => serviceSource.includes(name))],
  ['stage 2 page reader', pageSource.includes('MarkdownReader') && pageSource.includes('\u53cd\u5411\u94fe\u63a5')],
  ['markdown structures', ['markdown-table', 'markdown-code', 'markdown-math', 'markdown-wikilink'].every((name) => markdownSource.includes(name))],
  ['core books reader', ['listCoreBooks', 'listBookChapters', 'getBookChapter', 'PDF \u5b9a\u4f4d'].every((name) => booksSource.includes(name))],
  ['graph explorer', ['graphOverview', 'graphNeighbors', 'graphPath', 'Graphify \u6d3e\u751f'].every((name) => graphSource.includes(name))],
  ['comparison workbench', ['buildComparison', '\u81f3\u5c11\u9009\u62e9 2 \u4e2a\u9875\u9762', 'comparison-table'].every((name) => comparisonSource.includes(name))],
  ['p2 rust commands', ['list_core_books', 'get_book_chapter', 'graph_overview', 'graph_path', 'build_comparison'].every((name) => rustSource.includes(name))],
  ['p3 qa navigation', appSource.includes("label: '\u667a\u80fd\u95ee\u7b54'") && appSource.includes("view: 'qa'") && appSource.includes('<AskView')],
  ['p3 qa service', ['askLuna', 'cancelAnswer', 'listChatSessions', 'getLunaSettings'].every((name) => serviceSource.includes(name))],
  ['p3 evidence interface', ['\u672c\u8f6e\u8bc1\u636e', '\u5e93\u6c34\u4f4d', 'qa-inline-citation', '\u79bb\u7ebf\u8bc1\u636e'].every((name) => qaViewSource.includes(name))],
  ['p3 rust commands', ['prepare_question', 'ask_luna', 'cancel_answer', 'list_chat_sessions', 'save_luna_settings'].every((name) => rustSource.includes(name))],
  ['p3 chat schema', ['chat_sessions', 'chat_messages', 'chat_evidence', 'user_version'].every((name) => qaRustSource.includes(name))],
  ['p3 secret boundary', qaRustSource.includes('env::var(&settings.api_key_env)') && !qaRustSource.includes('luna.api_key"')],
  ['p3 citation contract', ['[{}]', 'physical_page_start', 'graph_hint', 'offline_answer'].every((name) => qaRustSource.includes(name))],
  ['repository ready reload', ['repositoryGeneration', 'setRepositoryGeneration', 'refreshRepository'].every((name) => appSource.includes(name))],
  ['workspace tree interaction', ['expandedWorkspaceNodes', 'toggleWorkspaceNode', 'aria-expanded', 'workspace-children'].every((name) => appSource.includes(name))],
  ['p4 compile navigation', appSource.includes("label: '\u7f16\u8bd1\u4e2d\u5fc3'") && appSource.includes("view: 'compile'") && appSource.includes('<CompileCenterView')],
  ['p4 compile service', ['getCompileCapabilities', 'startCompileRun', 'retryCompileRun', 'cancelCompileRun', 'rollbackCompileRun'].every((name) => serviceSource.includes(name))],
  ['p4 compile interface', ['\u4efb\u52a1\u76ee\u5f55', '\u5b9e\u65f6\u65e5\u5fd7', '\u76f8\u540c\u53c2\u6570\u91cd\u8bd5', '\u56de\u6eda\u5165\u53e3', 'Dry run'].every((name) => compileViewSource.includes(name))],
  ['p4 rust commands', ['get_compile_capabilities', 'start_compile_run', 'retry_compile_run', 'cancel_compile_run', 'rollback_compile_run'].every((name) => rustSource.includes(name))],
  ['p4 task schema', ['compile_runs', 'compile_run_events', 'compile_artifacts', 'COMPILE_SCHEMA_VERSION'].every((name) => compileRustSource.includes(name))],
  ['p4 allowlist and redaction', compileRustSource.includes('task kind is not in the compile allowlist') && compileRustSource.includes('[REDACTED]') && !compileRustSource.includes('cmd /c')],
  ['contextual research trail service', serviceSource.includes('prepareResearchTrail') && rustSource.includes('prepare_research_trail')],
  ['contextual research trail interface', ['retrievalReason', 'degradedChannels', '添加证据', '查看脉络图'].every((name) => researchTrailSource.includes(name))],
  ['contextual research trail retrieval', ['outgoing_link', 'backlink', 'graph_neighbor', 'method_candidates', 'select_diverse'].every((name) => researchTrailRustSource.includes(name))],
  ['context panel has no catalog fallback', appSource.includes('<ResearchTrailPanel') && !appSource.includes('const relatedMethods = useMemo')],
  ['library source count', data.waterline?.sources === 23],
  ['library method count', data.waterline?.methods === 20],
  ['library chapter count', data.waterline?.chapters === 61],
]

const failed = checks.filter(([, passed]) => !passed)
for (const [name, passed] of checks) console.log(`${passed ? 'PASS' : 'FAIL'} ${name}`)
if (failed.length) process.exit(1)
