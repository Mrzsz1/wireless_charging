[CmdletBinding()]
param(
    [switch] $DryRun
)

$ErrorActionPreference = 'Stop'
$utf8 = New-Object System.Text.UTF8Encoding($false)
[Console]::OutputEncoding = $utf8
$OutputEncoding = $utf8
$projectRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..')).Path
$runner = Join-Path $PSScriptRoot 'paper-search.ps1'

if (-not (Test-Path -LiteralPath $runner)) {
    throw "Paper Search runner was not found: $runner"
}

Set-Location -LiteralPath $projectRoot
$Host.UI.RawUI.WindowTitle = '无线充电论文搜索'

Write-Host ''
Write-Host '=== 无线充电论文搜索 ===' -ForegroundColor Cyan
Write-Host '正在搜索：arXiv / OpenAlex / Tavily / Google Scholar（SerpApi）'
Write-Host '边界：只生成新候选报告，不自动下载、不调用 MinerU、不写入 Wiki。'
Write-Host ''

$arguments = @('--new-only')
if ($DryRun) {
    $arguments += '--dry-run'
}

& $runner @arguments
$exitCode = $LASTEXITCODE

if ($exitCode -ne 0) {
    Write-Host ''
    Write-Host "搜索失败，退出码：$exitCode" -ForegroundColor Red
    exit $exitCode
}

Write-Host ''
if ($DryRun) {
    Write-Host 'Dry-run 完成：没有联网，也没有写入候选报告。' -ForegroundColor Yellow
    exit 0
}

$runsRoot = Join-Path $projectRoot 'raw\inbox\auto-discovered\runs'
$latestReport = Get-ChildItem -LiteralPath $runsRoot -Directory -ErrorAction SilentlyContinue |
    Sort-Object LastWriteTime -Descending |
    ForEach-Object { Join-Path $_.FullName 'README.md' } |
    Where-Object { Test-Path -LiteralPath $_ } |
    Select-Object -First 1

Write-Host '搜索完成。' -ForegroundColor Green
if ($latestReport) {
    Write-Host "最新候选报告：$latestReport"
}
Write-Host '关闭此窗口即可；需要扩库时，再让 Codex 审核最新候选。'



