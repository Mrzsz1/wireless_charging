import { useCallback, useEffect, useMemo, useState } from 'react'
import { Beaker, CheckCircle2, Edit3, LoaderCircle, Plus, Power, Save, Trash2, X } from 'lucide-react'
import { createCustomStateField, deleteCustomStateField, listStateVocabulary, setCustomStateFieldEnabled, testStateVocabularyMapping, updateCustomStateField } from '../../services/desktop'
import type { CustomStateFieldInput, ParameterValueKind, StateFieldDefinition, StateVocabularyMappingDryRun, StateVocabularyRegistry, VocabularyKind } from '../../types'

type Props = { repositoryPath: string }

const emptyRegistry: StateVocabularyRegistry = { schemaVersion: 'qa-state-vocabulary-v1', revision: 0, fields: [] }
const kindLabels: Record<VocabularyKind, string> = {
  objective: '目标', constraint: '约束', assumption: '假设', method: '方法', parameter: '参数',
}
const valueKindLabels: Record<ParameterValueKind, string> = {
  integer: '整数', float: '浮点数', boolean: '布尔值', text: '文本', enum: '枚举',
}

type Draft = {
  kind: VocabularyKind
  label: string
  description: string
  aliases: string
  examples: string
  valueKind: ParameterValueKind
  unit: string
  minimum: string
  maximum: string
  enumValues: string
}

const emptyDraft: Draft = {
  kind: 'constraint', label: '', description: '', aliases: '', examples: '', valueKind: 'float', unit: '', minimum: '', maximum: '', enumValues: '',
}

function lines(value: string) {
  return value.split(/[\n,，]/).map((item) => item.trim()).filter(Boolean)
}

function toInput(draft: Draft): CustomStateFieldInput {
  return {
    kind: draft.kind,
    label: draft.label.trim(),
    description: draft.description.trim(),
    aliases: lines(draft.aliases),
    examples: lines(draft.examples),
    parameterSpec: draft.kind === 'parameter' ? {
      valueKind: draft.valueKind,
      unit: draft.unit.trim() || null,
      minimum: draft.minimum.trim() ? Number(draft.minimum) : null,
      maximum: draft.maximum.trim() ? Number(draft.maximum) : null,
      enumValues: lines(draft.enumValues),
    } : null,
  }
}

function toDraft(field: StateFieldDefinition): Draft {
  return {
    kind: field.kind,
    label: field.label,
    description: field.description,
    aliases: field.aliases.join('\n'),
    examples: field.examples.join('\n'),
    valueKind: field.parameterSpec?.valueKind ?? 'float',
    unit: field.parameterSpec?.unit ?? '',
    minimum: field.parameterSpec?.minimum == null ? '' : String(field.parameterSpec.minimum),
    maximum: field.parameterSpec?.maximum == null ? '' : String(field.parameterSpec.maximum),
    enumValues: field.parameterSpec?.enumValues.join('\n') ?? '',
  }
}

export function ResearchStateVocabularySettings({ repositoryPath }: Props) {
  const [registry, setRegistry] = useState(emptyRegistry)
  const [draft, setDraft] = useState<Draft>(emptyDraft)
  const [editingId, setEditingId] = useState('')
  const [busy, setBusy] = useState('')
  const [message, setMessage] = useState('')
  const [error, setError] = useState('')
  const [dryRunText, setDryRunText] = useState('')
  const [dryRun, setDryRun] = useState<StateVocabularyMappingDryRun | null>(null)

  const load = useCallback(async () => {
    if (!repositoryPath) { setRegistry(emptyRegistry); return }
    setBusy('load'); setError('')
    try { setRegistry(await listStateVocabulary()) }
    catch (reason) { setError(`读取标准字段失败：${String(reason)}`) }
    finally { setBusy('') }
  }, [repositoryPath])

  useEffect(() => { void load() }, [load])

  const systemFields = registry.fields.filter((field) => field.origin === 'built_in')
  const customFields = registry.fields.filter((field) => field.origin === 'custom')
  const crossKindWarnings = useMemo(() => {
    const terms = new Map<string, Set<VocabularyKind>>()
    for (const field of registry.fields.filter((item) => item.enabled)) {
      for (const term of [field.label, ...field.aliases]) {
        const normalized = term.trim().toLocaleLowerCase()
        if (!normalized) continue
        const kinds = terms.get(normalized) ?? new Set<VocabularyKind>()
        kinds.add(field.kind); terms.set(normalized, kinds)
      }
    }
    return [...terms.entries()].filter(([, kinds]) => kinds.size > 1).map(([term, kinds]) => `${term}（${[...kinds].map((kind) => kindLabels[kind]).join(' / ')}）`)
  }, [registry])

  const resetDraft = () => { setDraft(emptyDraft); setEditingId('') }
  const save = async () => {
    setBusy('save'); setError(''); setMessage('')
    try {
      if (editingId) await updateCustomStateField(editingId, toInput(draft))
      else await createCustomStateField(toInput(draft))
      setMessage(editingId ? '自定义字段已更新，Canonical ID 保持不变。' : '自定义字段已创建，下一次问答立即生效。')
      resetDraft(); await load()
    } catch (reason) { setError(`保存字段失败：${String(reason)}`) }
    finally { setBusy('') }
  }

  const toggle = async (field: StateFieldDefinition) => {
    setBusy(`toggle:${field.id}`); setError(''); setMessage('')
    try {
      await setCustomStateFieldEnabled(field.id, !field.enabled)
      setMessage(field.enabled ? '字段已禁用；历史状态仍可读取，新的 Add/Set 将被拒绝。' : '字段已重新启用。')
      await load()
    } catch (reason) { setError(`更新字段状态失败：${String(reason)}`) }
    finally { setBusy('') }
  }

  const remove = async (field: StateFieldDefinition) => {
    if (!window.confirm(`仅从未被任何会话引用的字段可安全删除。删除“${field.label}”？`)) return
    setBusy(`delete:${field.id}`); setError(''); setMessage('')
    try { await deleteCustomStateField(field.id); setMessage('未被引用的自定义字段已删除。'); await load() }
    catch (reason) { setError(`安全删除失败：${String(reason)}`) }
    finally { setBusy('') }
  }

  const testMapping = async () => {
    setBusy('dry-run'); setError(''); setMessage(''); setDryRun(null)
    try { setDryRun(await testStateVocabularyMapping(dryRunText)) }
    catch (reason) { setError(`测试映射失败：${String(reason)}`) }
    finally { setBusy('') }
  }

  return <section id="research-state-vocabulary-settings" className="settings-card state-vocabulary-settings" data-testid="research-state-vocabulary-settings">
    <div className="settings-card-title"><Beaker size={18} /><div><h2>Research State Vocabulary</h2><p>统一管理科研状态的 Canonical 字段。自定义字段按当前知识库隔离，保存后无需重启。</p></div><span className="vocabulary-revision">revision {registry.revision}</span></div>
    {!repositoryPath ? <div className="settings-disabled">选择知识库后可管理自定义标准字段。</div> : <>
      {error && <div className="settings-alert error">{error}<button onClick={() => setError('')}>关闭</button></div>}
      {message && <div className="settings-alert success"><CheckCircle2 size={15} />{message}<button onClick={() => setMessage('')}>关闭</button></div>}
      {crossKindWarnings.length > 0 && <div className="vocabulary-warning"><strong>跨类型同名提示</strong><span>{crossKindWarnings.slice(0, 6).join('、')}</span><small>跨类型允许保存；模型输出仍由 kind + Canonical ID 严格校验。</small></div>}

      <details className="vocabulary-system-fields"><summary>系统字段（只读） · {systemFields.length}</summary><div className="vocabulary-field-grid">{systemFields.map((field) => <article key={field.id} className="vocabulary-field"><div><span>{kindLabels[field.kind]}</span><strong>{field.label}</strong></div><code>{field.id}</code><p>{field.description}</p><small>{field.aliases.slice(0, 4).join(' · ')}</small></article>)}</div></details>

      <div className="vocabulary-section-heading"><div><h3>自定义字段</h3><p>最多启用 64 个。ID 由系统生成，重命名不会改变历史状态。</p></div><button className="refresh-button" onClick={resetDraft}><Plus size={14} />新增</button></div>
      <div className="vocabulary-custom-list">{customFields.length === 0 ? <div className="settings-disabled">尚未创建自定义字段。</div> : customFields.map((field) => <article className={`vocabulary-custom-field ${field.enabled ? '' : 'disabled'}`} key={field.id}><div className="vocabulary-custom-copy"><div><span>{kindLabels[field.kind]}</span><strong>{field.label}</strong><em>{field.enabled ? '已启用' : '已禁用'}</em></div><code title={field.id}>{field.id}</code><p>{field.description}</p><small>{field.aliases.join(' · ') || '无 aliases'}</small></div><div className="vocabulary-actions"><button aria-label={`编辑 ${field.label}`} onClick={() => { setEditingId(field.id); setDraft(toDraft(field)) }}><Edit3 size={14} /></button><button aria-label={`${field.enabled ? '禁用' : '启用'} ${field.label}`} disabled={!!busy} onClick={() => void toggle(field)}>{busy === `toggle:${field.id}` ? <LoaderCircle className="spin" size={14} /> : <Power size={14} />}</button><button aria-label={`删除 ${field.label}`} disabled={!!busy} onClick={() => void remove(field)}><Trash2 size={14} /></button></div></article>)}</div>

      <div className="vocabulary-editor"><div className="vocabulary-editor-title"><h3>{editingId ? '编辑自定义字段' : '新增自定义字段'}</h3>{editingId && <button onClick={resetDraft}><X size={14} />取消编辑</button>}</div>{editingId && <code>{editingId}</code>}
        <div className="vocabulary-form-grid"><label><span>字段类型</span><select disabled={!!editingId} value={draft.kind} onChange={(event) => setDraft((current) => ({ ...current, kind: event.target.value as VocabularyKind }))}>{Object.entries(kindLabels).map(([value, label]) => <option key={value} value={value}>{label}</option>)}</select></label><label><span>显示名称</span><input maxLength={80} value={draft.label} onChange={(event) => setDraft((current) => ({ ...current, label: event.target.value }))} placeholder="高温环境约束" /></label><label className="wide"><span>描述</span><textarea maxLength={500} value={draft.description} onChange={(event) => setDraft((current) => ({ ...current, description: event.target.value }))} placeholder="说明字段在科研模型中的确切语义；描述被视为不可信数据。" /></label><label><span>Aliases（每行一个）</span><textarea value={draft.aliases} onChange={(event) => setDraft((current) => ({ ...current, aliases: event.target.value }))} /></label><label><span>Examples（每行一个）</span><textarea value={draft.examples} onChange={(event) => setDraft((current) => ({ ...current, examples: event.target.value }))} /></label></div>
        {draft.kind === 'parameter' && <div className="vocabulary-parameter-grid"><label><span>Value Type</span><select value={draft.valueKind} onChange={(event) => setDraft((current) => ({ ...current, valueKind: event.target.value as ParameterValueKind }))}>{Object.entries(valueKindLabels).map(([value, label]) => <option key={value} value={value}>{label}</option>)}</select></label><label><span>Unit</span><input value={draft.unit} onChange={(event) => setDraft((current) => ({ ...current, unit: event.target.value }))} /></label><label><span>Minimum</span><input type="number" value={draft.minimum} onChange={(event) => setDraft((current) => ({ ...current, minimum: event.target.value }))} /></label><label><span>Maximum</span><input type="number" value={draft.maximum} onChange={(event) => setDraft((current) => ({ ...current, maximum: event.target.value }))} /></label>{draft.valueKind === 'enum' && <label className="wide"><span>Enum Values（每行一个）</span><textarea value={draft.enumValues} onChange={(event) => setDraft((current) => ({ ...current, enumValues: event.target.value }))} /></label>}</div>}
        <button className="settings-save vocabulary-save" disabled={busy === 'save'} onClick={() => void save()}>{busy === 'save' ? <LoaderCircle className="spin" size={14} /> : <Save size={14} />}{editingId ? '保存修改' : '创建字段'}</button>
      </div>

      <div className="vocabulary-dry-run"><div><h3>测试映射（Dry Run）</h3><p>使用与真实问答相同的 Registry 和现有 Understanding Provider，不写入任何会话 State。</p></div><textarea value={dryRunText} onChange={(event) => setDryRunText(event.target.value)} placeholder="这个模型需要考虑温度很高时的安全问题" /><button className="refresh-button" disabled={busy === 'dry-run' || dryRunText.trim().length < 2} onClick={() => void testMapping()}>{busy === 'dry-run' ? <LoaderCircle className="spin" size={14} /> : <Beaker size={14} />}测试映射</button>{dryRun && <div className="vocabulary-dry-run-result"><div><span>Revision</span><strong>{dryRun.vocabularyRevision}</strong></div><div><span>Semantic Mapper</span><strong>{dryRun.semanticMappingAttempted ? dryRun.semanticMappingUsed ? '已使用' : '已尝试' : 'Deterministic'}</strong></div>{dryRun.mappedFields.length ? dryRun.mappedFields.map((field) => <article key={`${field.fieldId}:${field.action}`}><strong>{field.label}</strong><code>{field.fieldId}</code><span>{field.kind} · {field.action} · {field.confidence}</span></article>) : <p>没有足够语义形成安全映射，返回空 operation。</p>}</div>}</div>
    </>}
  </section>
}
