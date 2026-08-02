[CmdletBinding()]
param(
    [Parameter(Position = 0)]
    [string]$InputPath,

    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$ExtraArgs
)

$ErrorActionPreference = 'Stop'
$scriptPath = Join-Path $PSScriptRoot 'mineru_to_md.py'
$launcher = Get-Command py -ErrorAction SilentlyContinue

if (-not $launcher) {
    throw 'Python Launcher (py.exe) was not found. Install Python 3 and verify that py -3 works.'
}

$pythonArgs = @('-3', $scriptPath)
if ($InputPath) {
    $pythonArgs += $InputPath
}
if ($ExtraArgs) {
    $pythonArgs += $ExtraArgs
}

& $launcher.Source @pythonArgs
exit $LASTEXITCODE
