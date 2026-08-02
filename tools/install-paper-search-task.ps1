param(
    [string] $TaskName = "WirelessChargingPaperSearch",
    [string] $DailyAt = "09:00"
)

$ErrorActionPreference = "Stop"
$runner = Join-Path $PSScriptRoot "paper-search.ps1"
$arguments = '-NoProfile -ExecutionPolicy Bypass -File "' + $runner + '" --new-only'
$action = New-ScheduledTaskAction -Execute "powershell.exe" -Argument $arguments
$trigger = New-ScheduledTaskTrigger -Daily -At $DailyAt

Register-ScheduledTask `
    -TaskName $TaskName `
    -Description "Search arXiv/OpenAlex candidates for the wireless charging wiki" `
    -Action $action `
    -Trigger $trigger `
    -Force | Out-Null

Write-Output "Scheduled task '$TaskName' installed. It runs daily at $DailyAt."
