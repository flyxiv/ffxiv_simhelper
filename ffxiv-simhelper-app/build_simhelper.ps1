yarn electron-pack

cp .\ffxiv_simhelper_x86_64.exe .\dist\
cp .\run.exe .\dist\
rm .\dist\ffxiv-simhelper-app*

mv .\dist .\ffxiv_simhelper