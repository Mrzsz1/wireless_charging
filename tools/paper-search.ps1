param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]] $PaperSearchArgs
)

$ErrorActionPreference = "Stop"
$script = Join-Path $PSScriptRoot "paper_search.py"

if (Get-Command py -ErrorAction SilentlyContinue) {
    & py -3 $script @PaperSearchArgs
} elseif (Get-Command python -ErrorAction SilentlyContinue) {
    & python $script @PaperSearchArgs
} else {
    throw "Python 3 was not found. Install Python and add py or python to PATH."
}

exit $LASTEXITCODE
