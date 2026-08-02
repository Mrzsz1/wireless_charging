[CmdletBinding()]
param(
    [string] $ShortcutPath
)

$ErrorActionPreference = 'Stop'
$projectRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..')).Path
$launcher = Join-Path $PSScriptRoot 'launch-wiki.ps1'

if (-not (Test-Path -LiteralPath $launcher)) {
    throw "Launcher was not found: $launcher"
}

if (-not $ShortcutPath) {
    $desktopCandidates = @(
        [Environment]::GetFolderPath('Desktop'),
        (Join-Path $env:USERPROFILE 'OneDrive\Desktop'),
        (Join-Path $env:USERPROFILE 'Desktop')
    ) | Where-Object { $_ -and (Test-Path -LiteralPath $_) }
    $desktop = $desktopCandidates | Select-Object -First 1
    if (-not $desktop) {
        throw 'Desktop directory was not found. Pass -ShortcutPath explicitly.'
    }
    $ShortcutPath = Join-Path $desktop '无线充电 LLM Wiki.lnk'
}

$shortcutDirectory = Split-Path -Parent $ShortcutPath
if (-not (Test-Path -LiteralPath $shortcutDirectory)) {
    throw "Shortcut directory does not exist: $shortcutDirectory"
}

$powershell = Join-Path $env:SystemRoot 'System32\WindowsPowerShell\v1.0\powershell.exe'
$shell = New-Object -ComObject WScript.Shell
$shortcut = $shell.CreateShortcut($ShortcutPath)
$shortcut.TargetPath = $powershell
$shortcut.Arguments = '-NoProfile -ExecutionPolicy Bypass -File "' + $launcher + '"'
$shortcut.WorkingDirectory = $projectRoot
$shortcut.Description = '打开无线充电调度 LLM Wiki'
$shortcut.IconLocation = "$env:SystemRoot\System32\imageres.dll,102"
$shortcut.Save()

Write-Output "shortcut=$ShortcutPath"
Write-Output "launcher=$launcher"
