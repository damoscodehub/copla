[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
$repositoryRoot = Split-Path -Parent $PSScriptRoot
$applicationPath = Join-Path $repositoryRoot 'src-tauri\target\release\copla.exe'

if (-not (Test-Path -LiteralPath $applicationPath -PathType Leaf)) {
    Write-Host 'Copla has not been built yet.' -ForegroundColor Yellow
    Write-Host 'Run "npm run desktop:build" from the repository root, then launch this file again.'
    exit 1
}

Start-Process -FilePath $applicationPath -WorkingDirectory (Split-Path -Parent $applicationPath)
