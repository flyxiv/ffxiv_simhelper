import { spawn } from 'child_process';
import { app, BrowserWindow, dialog } from 'electron';
import url from 'url';
import pkg from 'electron-updater';
const { autoUpdater } = pkg;
import log from 'electron-log';

const exePath = "./ffxiv_simhelper_x86_64.exe";
const backendProcess = spawn(exePath, [], {
    stdio: 'ignore', // 이 옵션 설정 시 콘솔 창을 띄우지 않음
});
function createWindow() {
    /*
* 넓이 1920에 높이 1080의 FHD 풀스크린 앱을 실행시킵니다.
* */
    const win = new BrowserWindow({
        width: 1920,
        height: 1080,
        icon: './images/icon.png',
    });

    /*
    * ELECTRON_START_URL을 직접 제공할경우 해당 URL을 로드합니다.
    * 만일 URL을 따로 지정하지 않을경우 (프로덕션빌드) React 앱이
    * 빌드되는 build 폴더의 index.html 파일을 로드합니다.
    * */
    const startUrl = process.env.ELECTRON_START_URL || url.format({
        pathname: '/index.html',
        protocol: 'file:',
        slashes: true
    });

    /*
    * startUrl에 배정되는 url을 맨 위에서 생성한 BrowserWindow에서 실행시킵니다.
    * */
    win.loadURL(startUrl);

}

autoUpdater.on('checking-for-update', () => {
    log.info('Checking for update...');
});

autoUpdater.on('update-not-available', () => {
    log.info('Update not available.');
});

autoUpdater.on('update-available', () => {
    log.info('Update available');

    const dialogOpts = {
        type: 'info',
        buttons: ['Restart', 'Later'],
        title: 'Application Update',
        message: 'A new version has been downloaded.',
        detail: 'It will be installed after a restart.'
    };

    dialog.showMessageBox(dialogOpts).then((returnValue) => {
        if (returnValue.response === 0) { // 'Restart' 클릭 시
            autoUpdater.quitAndInstall();
        }
    });
});

autoUpdater.on('update-downloaded', () => {
    log.info('Update downloaded');

});

app.on('ready', () => {
    createWindow();

    autoUpdater.logger = log;
    autoUpdater.logger.transports.file.level = 'info';


    autoUpdater.checkForUpdatesAndNotify().catch(err => {
        console.error('Update check failed: ', err);
    });
});

app.on("window-all-closed", async () => {
    app.quit();
    backendProcess.kill('SIGINT');
});