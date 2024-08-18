Start-Process -FilePath ".\ffxiv-simbot-x86_64.exe" -NoNewWindow
Set-Location -Path "ffxiv-simbot-app/dist"
Start-Process -FilePath ".\win-unpacked\ffxiv-simbot-app.exe"
Set-Location -Path "../.."