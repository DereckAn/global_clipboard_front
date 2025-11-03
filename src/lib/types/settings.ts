export interface AppSettings {
  // General
  maxLocalItems: number;
  autoSaveClipboard: boolean;

  // NUEVOS: Límite y retención
  maxItemsEnabled: boolean;
  retentionEnabled: boolean;
  retentionDays: number; // 7, 30, 90, 180, 365, 0 (nunca)

  // Monitoreo
  clipboardMonitorInterval: number; // ms (500 por defecto)

  // Notificaciones
  notificationsEnabled: boolean;
  notificationSound: boolean;

  // Auto-inicio
  autoStart: boolean;

  //Hot keys
  showHotkey: string;

  // Advance
  enableAnalytics: boolean;
}

export const DEFAULT_SETTINGS: AppSettings = {
  maxLocalItems: 100,
  autoSaveClipboard: true,
  showHotkey: "Control+Shift+V",
  enableAnalytics: false,
  maxItemsEnabled: false,
  retentionEnabled: false,
  retentionDays: 30, // CAMBIAR de 0 a 30
  clipboardMonitorInterval: 500, // CAMBIAR de 50 a 500
  notificationsEnabled: false,
  notificationSound: false,
  autoStart: false,
};
