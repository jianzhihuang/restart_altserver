@echo off
echo Finding AltServer.exe process...
for /f "tokens=2" %%i in ('tasklist ^| findstr AltServer.exe') do set PID=%%i
if defined PID (
    echo Found AltServer.exe process, PID is %PID%.
    echo Killing process...
    taskkill /F /PID %PID%
    echo Process killed.
) else (
    echo AltServer.exe process not found.
)
echo Starting AltServer.exe...
start "" "C:\Program Files (x86)\AltServer\AltServer.exe"
echo Done.
