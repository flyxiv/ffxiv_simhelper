import { contextBridge } from "electron";

contextBridge.exposeInMainWorld('pathAPI', {
  __dirname: () => __dirname,
  join: (...args) => path.join(...args)
});