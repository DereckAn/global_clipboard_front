export interface AppSettings {
    // General
    maxLocalItems:number
    autoSaveClipboard:boolean

    //Hot keys
    showHotkey:string

    // Advance
    enableAnalytics:boolean
}

export const DEFAULT_SETTINGS: AppSettings = {
    maxLocalItems: 100,
    autoSaveClipboard: true,
    showHotkey: "Control+Shift+V",
    enableAnalytics: false,
}