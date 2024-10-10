rm -r .\dist
rm -r .\ffxiv_simhelper

yarn electron-pack

cp .\ffxiv_simhelper_x86_64.exe .\dist\
rm .\dist\ffxiv-simhelper-app*
mv .\dist\win-unpacked\* .\dist
rm -r .\dist\win-unpacked

mv .\dist .\ffxiv_simhelper