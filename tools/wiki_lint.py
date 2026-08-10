#!/usr/bin/env python3
"""Read-only structural lint for the local wiki."""
from __future__ import annotations
import argparse,json,re
from collections import Counter,defaultdict
from dataclasses import dataclass,asdict
from datetime import datetime
from pathlib import Path
from typing import Iterable,Sequence

ROOT=Path(__file__).resolve().parents[1]; WIKI=ROOT/'wiki'; LOGS=ROOT/'logs'
WIKILINK=re.compile(r'\[\[([^\]|#]+)(?:#[^\]|]+)?(?:\|[^\]]+)?\]\]')
TYPES={'source':('sources','src-'),'concept':('concepts','cpt-'),'system-model':('system-models','sys-'),'objective':('objectives','obj-'),'method':('methods','mtd-'),'dataset-or-sim':('datasets-sims','data-'),'synthesis':('syntheses','syn-'),'problem':('problems','prob-'),'idea':('ideas','idea-')}
DEEP_REQUIRED={
    'source': [('TL;DR',),('何时使用','适用'),('系统模型','系统设定'),('目标','约束'),('算法','方法'),('理论','复杂度','NP-hard'),('实验','仿真'),('局限','失效'),('证据',)],
    'method': [('TL;DR',),('何时使用','适用条件'),('输入','输出'),('算法','步骤'),('复杂度','理论保证','原文未报告'),('失效','适用边界'),('证据','来源')],
    'system-model': [('TL;DR',),('何时使用','使用条件','适用边界'),('形式化','变量','状态'),('证据',)],
    'objective': [('TL;DR',),('形式化','统一表达','常见表达'),('权衡','使用检查','适用'),('证据',)],
    'dataset-or-sim': [('TL;DR',),('记录','设置'),('证据','样板'),('比较','复现')],
}
@dataclass
class Finding: section:int; severity:str; path:str; message:str
def frontmatter(text):
    if not text.lstrip('\ufeff').startswith('---'): return {}
    p=text.lstrip('\ufeff').split('---',2); out={}
    if len(p)<3:return out
    for ln in p[1].splitlines():
        if ':' in ln and not ln.lstrip().startswith('#'):
            k,v=ln.split(':',1); out[k.strip()]=v.strip().strip('"\'')
    return out
def norm(s): return s.strip().replace('\\','/').removesuffix('.md').strip('/').casefold()
def markdown_files(): return sorted(p for p in WIKI.rglob('*.md') if p.is_file())
def linkable_files(): return sorted(p for p in ROOT.rglob('*.md') if p.is_file() and 'raw' not in p.relative_to(ROOT).parts and 'graphify-out' not in p.relative_to(ROOT).parts)
def resolve_link(source:Path, raw:str, targets:dict[str,list[Path]]):
    """Resolve Obsidian links by path relative to the source before stem fallback."""
    cleaned=raw.strip().replace('\\','/')
    path=Path(cleaned)
    candidates=[]
    if '/' in cleaned or cleaned.startswith('.'):
        candidate=(source.parent/path)
        if candidate.suffix.lower()!='.md': candidate=candidate.with_suffix('.md')
        try: candidate=candidate.resolve()
        except OSError: pass
        if candidate.exists() and candidate.is_file(): candidates.append(candidate)
    if cleaned.startswith('wiki/') or cleaned.startswith('schema/') or cleaned.startswith('logs/'):
        candidate=ROOT/path
        if candidate.suffix.lower()!='.md': candidate=candidate.with_suffix('.md')
        if candidate.exists() and candidate.is_file(): candidates.append(candidate.resolve())
    if not candidates: candidates.extend(targets.get(norm(cleaned),[]))
    return list(dict.fromkeys(candidates))
def vocab_ids():
    p=ROOT/'schema/vocab.yaml'; ids=set()
    if p.exists(): ids.update(re.findall(r'^\s*- id:\s*([A-Za-z0-9_-]+)',p.read_text(encoding='utf-8-sig'),re.M))
    return ids
def inspect(files:Iterable[Path]):
    files=list(files); f=[]; targets=defaultdict(list); incoming=Counter(); cache={}; titles=defaultdict(list)
    for p in linkable_files():
        rel=p.relative_to(ROOT).as_posix(); keys={norm(rel),norm(p.stem),norm(rel.removeprefix('wiki/'))};
        for k in keys: targets[k].append(p)
    ids=vocab_ids()
    for p in files:
        t=p.read_text(encoding='utf-8-sig'); fm=frontmatter(t); cache[p]=(t,fm); 
        if fm.get('title'): titles[fm['title'].casefold()].append(p)
        rel=p.relative_to(ROOT).as_posix(); typ=fm.get('type','')
        if '\ufffd' in t or re.search(r'[锟烫鐧莙莨][^\n]{1,3}[绔滃浘]',t): f.append(Finding(1,'warning',rel,'疑似中文乱码'))
        for k in ('type','title','status'):
            if not fm.get(k): f.append(Finding(1,'error',rel,f'缺少必填 frontmatter: {k}'))
        if typ in TYPES:
            folder,prefix=TYPES[typ]
            if p.parent.name!=folder: f.append(Finding(1,'error',rel,f'type={typ} 目录应为 wiki/{folder}/'))
            if not p.stem.startswith(prefix): f.append(Finding(1,'warning',rel,f'文件名前缀应为 {prefix}'))
        if typ=='source':
            for k in ('year','source_type','acquisition_method','paper_keywords','keyword_source','triage_status','ingest_status'):
                if not fm.get(k): f.append(Finding(1,'warning',rel,f'source 缺少字段: {k}'))
            if not fm.get('pdf_path') and not fm.get('raw_md'): f.append(Finding(1,'warning',rel,'source 缺少 pdf_path/raw_md'))
            if fm.get('acquisition_method')=='auto_discovery':
                for k in ('discovered_via','discovery_run'):
                    if not fm.get(k): f.append(Finding(1,'warning',rel,f'auto source 缺少字段: {k}'))
        if fm.get('updated','') >= '2026-08-11' and typ in DEEP_REQUIRED:
            missing_groups=[group for group in DEEP_REQUIRED[typ] if not any(term.casefold() in t.casefold() for term in group)]
            if missing_groups:
                labels=['/'.join(group) for group in missing_groups]
                f.append(Finding(3,'warning',rel,'研究档案缺少结构：'+ '、'.join(labels)))
        resolved_links=[]
        for m in WIKILINK.finditer(t):
            raw=m.group(1).strip(); hits=resolve_link(p,raw,targets)
            resolved_links.extend(hits)
            if len(hits)==1: incoming[hits[0]]+=1
            elif not hits:f.append(Finding(2,'warning',rel,f'未解析 wikilink: [[{raw}]]'))
            else:f.append(Finding(5,'warning',rel,f'歧义 wikilink: [[{raw}]]'))
        if typ=='source' and not any('/methods/' in x.as_posix() or '/syntheses/' in x.as_posix() for x in resolved_links):
            f.append(Finding(3,'info',rel,'来源页尚未链接到方法页或综合页'))
        for field in ('scenario','entities','constraints','objectives'):
            val=fm.get(field,'')
            if val:
                for x in re.findall(r'[A-Za-z][A-Za-z0-9_-]+',val):
                    if x not in ids:f.append(Finding(1,'warning',rel,f'词表外 {field}: {x}'))
        if typ in ('problem','idea') and not fm.get('inspired_by'): f.append(Finding(1,'warning',rel,'B 类页面缺少 inspired_by'))
        if typ=='idea' and fm.get('user_confirmed','').lower() not in ('true','yes'): f.append(Finding(6,'warning',rel,'idea 未 user_confirmed=true'))
        if typ not in ('problem','idea') and re.search(r'我们的贡献|我(们)?的 idea|our contribution',t,re.I): f.append(Finding(6,'warning',rel,'A 类页面含主观贡献表述'))
        if re.search(r'二选一|只能选择|排他|either/or',t,re.I) and '并列不裁断' not in t: f.append(Finding(7,'warning',rel,'可能存在冲突裁断表述'))
    for p in files:
        rel=p.relative_to(ROOT).as_posix()
        if not incoming[p] and '/maps/' not in rel and rel!='wiki/index.md': f.append(Finding(4,'info',rel,'没有检测到 Wiki 入链'))
    for title,ps in titles.items():
        if len(ps)>1:
            for p in ps:f.append(Finding(5,'warning',p.relative_to(ROOT).as_posix(),f'重复标题: {title}'))
    actual=Counter(frontmatter(cache[p][0]).get('type','') for p in files)
    status=ROOT/'wiki/maps/library-status.md'
    if status.exists():
        sfm=frontmatter(status.read_text(encoding='utf-8-sig'))
        declared={'source':sfm.get('source_count'),'synthesis':sfm.get('synthesis_count')}
        for typ,value in declared.items():
            if value and value.isdigit() and int(value)!=actual[typ]:
                f.append(Finding(2,'warning','wiki/maps/library-status.md',f'{typ} 水位声明 {value} 与实际 {actual[typ]} 不一致'))
    home=ROOT/'wiki/maps/map-home.md'
    if home.exists():
        home_text=home.read_text(encoding='utf-8-sig')
        expected_tokens=(f"{actual['source']} sources",f"{actual['method']} methods",f"{actual['synthesis']} syntheses")
        for token in expected_tokens:
            if token not in home_text:
                f.append(Finding(2,'warning','wiki/maps/map-home.md',f'总图未声明当前实际水位：{token}'))
    g=ROOT/'graphify-out/graph.json'
    if not g.exists(): f.append(Finding(8,'info','graphify-out/graph.json','Graphify 图不存在'))
    else:
        try:
            graph=json.loads(g.read_text(encoding='utf-8-sig'))
            graph_sources={norm(str(n.get('source_file',''))) for n in graph.get('nodes',[]) if isinstance(n,dict) and n.get('source_file')}
            missing=[p.relative_to(ROOT).as_posix() for p in files if norm(p.relative_to(ROOT).as_posix()) not in graph_sources]
            if missing:
                sample='、'.join(missing[:5]); suffix=f' 等 {len(missing)} 页' if len(missing)>5 else ''
                f.append(Finding(8,'warning','graphify-out/graph.json',f'派生图未覆盖 Wiki 页面：{sample}{suffix}'))
        except (OSError,ValueError,TypeError) as exc:
            f.append(Finding(8,'warning','graphify-out/graph.json',f'Graphify 图无法解析：{exc}'))
    summary={'pages':len(files),'errors':sum(x.severity=='error' for x in f),'warnings':sum(x.severity=='warning' for x in f),'info':sum(x.severity=='info' for x in f),'broken_links':sum('未解析 wikilink' in x.message for x in f),'orphans':sum(x.section==4 for x in f)}
    return f,summary
def render(findings,summary,generated_at):
    names={1:'Schema 完整性',2:'链接与水位',3:'覆盖缺口与详细度',4:'孤儿页',5:'重复与歧义',6:'A/B 污染',7:'冲突表述',8:'Graphify 一致性'}; lines=[f'# Lint Report — {generated_at[:10]}','', '## Summary','',f"{summary['pages']} 页；{summary['errors']} errors；{summary['warnings']} warnings；{summary['info']} info",'']
    for i in range(1,9):
        lines += [f'## {i}. {names[i]}','']+[f'- **{x.severity.upper()}** `{x.path}` — {x.message}' for x in findings if x.section==i] or ['- 未发现确定性问题'] ; lines.append('')
    return '\n'.join(lines)
def main(argv:Sequence[str]|None=None):
    ap=argparse.ArgumentParser(); ap.add_argument('--write-report',action='store_true'); ap.add_argument('--json',action='store_true'); ap.add_argument('--strict-graphify',action='store_true'); a=ap.parse_args(argv); now=datetime.now().astimezone().isoformat(timespec='seconds'); fs,s=inspect(markdown_files()); report=render(fs,s,now); rp=None
    if a.write_report:
        LOGS.mkdir(exist_ok=True); rp=LOGS/f'{now[:10]}-wiki-lint.md'; rp.write_text(report,encoding='utf-8'); print(f'ARTIFACT:{rp.relative_to(ROOT).as_posix()}')
    if a.json: print(json.dumps({'generatedAt':now,'summary':s,'findings':[asdict(x) for x in fs]},ensure_ascii=False,indent=2))
    else: print(f"Lint 完成：{s['pages']} 页，{s['errors']} errors，{s['warnings']} warnings，{s['info']} info")
    return 1 if s['errors'] or (a.strict_graphify and any(x.section==8 and x.severity=='warning' for x in fs)) else 0
if __name__=='__main__': raise SystemExit(main())
