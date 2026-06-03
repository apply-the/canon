$ErrorActionPreference = "Stop"

# Navigate to the project root
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location -Path "$ScriptDir\.."

Write-Host "Starting Canon VitePress dev server..." -ForegroundColor Magenta
npm run site:dev
