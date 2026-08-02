[CmdletBinding()]
param(
    [switch] $DryRun
)

$ErrorActionPreference = 'Stop'
$projectRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..')).Path
$homeFile = Join-Path $projectRoot 'HOME.md'

if (-not (Test-Path -LiteralPath $homeFile)) {
    throw "Wiki entry was not found: $homeFile"
}

function Test-ObsidianProtocol {
    $protocolKeys = @(
        'Registry::HKEY_CURRENT_USER\Software\Classes\obsidian\shell\open\command',
        'Registry::HKEY_CLASSES_ROOT\obsidian\shell\open\command'
    )
    return [bool]($protocolKeys | Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1)
}

$encodedHome = [Uri]::EscapeDataString($homeFile)
$obsidianUri = "obsidian://open?path=$encodedHome"

if (Test-ObsidianProtocol) {
    if ($DryRun) {
        Write-Output "launcher=obsidian-uri"
        Write-Output "target=$homeFile"
        exit 0
    }
    Start-Process -FilePath $obsidianUri
    exit 0
}

$obsidianCandidates = @(
    (Join-Path $env:LOCALAPPDATA 'Programs\Obsidian\Obsidian.exe'),
    (Join-Path $env:LOCALAPPDATA 'Obsidian\Obsidian.exe'),
    (Join-Path $env:ProgramFiles 'Obsidian\Obsidian.exe')
) | Where-Object { $_ -and (Test-Path -LiteralPath $_) }

$obsidianExe = $obsidianCandidates | Select-Object -First 1
if ($obsidianExe) {
    if ($DryRun) {
        Write-Output "launcher=obsidian-exe"
        Write-Output "executable=$obsidianExe"
        Write-Output "target=$homeFile"
        exit 0
    }
    Start-Process -FilePath $obsidianExe -ArgumentList @($projectRoot)
    exit 0
}

if ($DryRun) {
    Write-Output "launcher=default-markdown-app"
    Write-Output "target=$homeFile"
    exit 0
}

try {
    Start-Process -FilePath $homeFile
} catch {
    Start-Process -FilePath 'explorer.exe' -ArgumentList @($projectRoot)
}
