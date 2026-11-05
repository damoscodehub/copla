@echo off
setlocal
set "COPLA_ROOT=%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%COPLA_ROOT%scripts\launch-copla.ps1"
set "COPLA_EXIT=%ERRORLEVEL%"
endlocal & exit /b %COPLA_EXIT%
