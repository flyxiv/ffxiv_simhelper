cargo build --release
rm -r .\ffxiv-simhelper-app\ffxiv_simhelper_x86_64.exe
cp .\target\release\ffxiv-simhelper.exe .\ffxiv-simhelper-app\ffxiv_simhelper_x86_64.exe