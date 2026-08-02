param(
    [Parameter(Position = 0, Mandatory = $true)]
    [string] $Manifest,

    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $TriageArgs
)

$ErrorActionPreference = "Stop"
$script = Join-Path $PSScriptRoot "paper_triage.py"

if (Get-Command py -ErrorAction SilentlyContinue) {
    & py -3 $script $Manifest @TriageArgs
} elseif (Get-Command python -ErrorAction SilentlyContinue) {
    & python $script $Manifest @TriageArgs
} else {
    throw "Python 3 was not found. Install Python and add py or python to PATH."
}

exit $LASTEXITCODE
