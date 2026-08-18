import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import test from 'node:test'

const read = (path: string) => readFileSync(new URL(path, import.meta.url), 'utf8')

test('provider connection settings stay global while per-turn Codex controls live in composer', () => {
  const settings = read('../src/features/settings/SettingsView.tsx')
  const ask = read('../src/features/qa/AskView.tsx')
  assert.match(settings, /data-testid="qa-engine-settings"/)
  assert.match(settings, /Codex 订阅/)
  assert.match(settings, /兼容 API/)
  assert.match(settings, /证据浏览模式/)
  assert.match(settings, /上下文窗口 Token/)
  assert.match(settings, /历史轮数不设固定上限/)
  assert.doesNotMatch(settings, /保留近期完整轮次|recentExchangeLimit/)
  assert.match(settings, /getCodexSubscriptionStatus/)
  assert.doesNotMatch(settings, /qa-codex-selection/)
  assert.match(settings, /每次对话的模型与推理强度在智能问答输入框中选择/)
  assert.match(ask, /aria-label="Codex 模型"/)
  assert.match(ask, /aria-label="推理强度"/)
  assert.match(ask, /supportedReasoningEfforts/)
  assert.match(ask, /codexReasoningEffort/)
  assert.match(ask, /data-testid="qa-open-settings"/)
  assert.doesNotMatch(ask, /Luna 设置|qa-settings-dialog|saveLunaSettings|settingsDraft/)
})

test('Codex status DTO and settings expose no authentication secret', () => {
  const types = read('../src/types.ts')
  const start = types.indexOf('export type CodexSubscriptionStatus')
  const end = types.indexOf('export type WaterlineSnapshot')
  const status = types.slice(start, end)
  assert.match(status, /authenticated: boolean/)
  assert.match(status, /ready: boolean/)
  assert.match(status, /availableModels: CodexModelOption\[\]/)
  assert.match(status, /configuredReasoningEffort: string/)
  assert.doesNotMatch(status, /token|cookie|apiKey|credentialPath/)
})

test('semantic model deployment settings stay global and separate offline check from repair', () => {
  const settings = read('../src/features/settings/SettingsView.tsx')
  const services = read('../src/services/desktop.ts')
  const types = read('../src/types.ts')
  assert.match(settings, /data-testid="semantic-model-settings"/)
  assert.match(settings, /本地语义模型/)
  assert.match(settings, /复制现有缓存并切换/)
  assert.match(settings, /切换并重新部署/)
  assert.match(settings, /部署检查严格离线/)
  assert.match(settings, /checkSemanticModelDeployment/)
  assert.match(settings, /repairSemanticModelDeployment/)
  assert.match(settings, /data-testid="semantic-download-progress"/)
  assert.match(settings, /semanticProgress\.bytesPerSecond/)
  assert.match(services, /invoke<SemanticDeploymentStatus>\('check_semantic_model_deployment'\)/)
  assert.match(services, /invoke<SemanticDeploymentStatus>\('repair_semantic_model_deployment', \{ onEvent \}\)/)
  assert.match(services, /new Channel<SemanticDownloadProgress>/)
  assert.match(types, /SemanticDeploymentState = 'missing' \| 'partial' \| 'invalid' \| 'ready' \| 'error'/)
  assert.match(types, /bytesPerSecond: number/)
})

test('Windows Codex discovery covers desktop binaries, persistent PATH and script shims', () => {
  const rust = readFileSync(new URL('../src-tauri/src/codex_subscription.rs', import.meta.url), 'utf8')
  for (const contract of ['CODEX_CLI_PATH', 'OpenAI', 'Codex', 'read_registry_path', 'codex.exe', 'codex.cmd', 'codex.bat']) {
    assert.match(rust, new RegExp(contract.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')))
  }
  assert.match(rust, /executable_has_valid_version\(&executable\)/)
})

test('App routes the AskView settings action to the global settings section', () => {
  const app = read('../src/App.tsx')
  assert.match(app, /onOpenSettings=\{\(\) => openSettings\('qa-engine-settings'\)\}/)
  assert.match(app, /focusSection=\{settingsFocusSection\}/)
})

test('QA generation UI exposes validation-driven Thinking and compact composer contracts', () => {
  const ask = read('../src/features/qa/AskView.tsx')
  const css = read('../src/features/qa/AskView.css')
  const types = read('../src/types.ts')
  assert.match(types, /type: 'validation_started'/)
  assert.match(ask, /Thinking · \{elapsedSeconds\}s/)
  assert.match(ask, /引用与完整性校验/)
  assert.match(ask, /const activeThinkingStep = phase === 'retrieving'/)
  assert.match(ask, /index === activeThinkingStep \? 'active' : 'waiting'/)
  assert.match(ask, /className="qa-thinking-loader"/)
  assert.match(ask, /className="qa-thinking-flow" aria-hidden="true"/)
  assert.match(ask, /className="qa-thinking-current" role="status" aria-live="polite" aria-atomic="true"/)
  assert.match(ask, /正在生成回答/)
  assert.match(ask, /className="qa-stream-cursor" aria-hidden="true"/)
  assert.match(ask, /ref=\{composerRef\}/)
  assert.match(ask, /codexModel: settings\.codexModel/)
  assert.match(ask, /codexReasoningEffort: settings\.codexReasoningEffort/)
  for (const contract of ['@keyframes qa-thinking-flow', '@keyframes qa-thinking-dot', '@keyframes qa-stream-cursor', '@media(prefers-reduced-motion:reduce)', '.qa-thinking-chain>div.active']) {
    assert.ok(css.includes(contract), contract)
  }
  for (const contract of ['.qa-chat-heading{grid-row:1}', '.qa-error{grid-row:2}', '.qa-messages{grid-row:3}', '.qa-composer{grid-row:4}', 'max-height:148px']) {
    assert.ok(css.includes(contract), contract)
  }
})
