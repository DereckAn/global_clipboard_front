import { relaunch } from "@tauri-apps/plugin-process";
import { check, type Update } from "@tauri-apps/plugin-updater";

export type UpdateStatus =
  | "idle"
  | "checking"
  | "available"
  | "not-available"
  | "downloading"
  | "ready"
  | "error";

class UpdaterStore {
  status = $state<UpdateStatus>("idle");
  update = $state<Update | null>(null);
  error = $state<string | null>(null);
  downloadProgress = $state<number>(0);
  currentVersion = $state<string>("");
  newVersion = $state<string>("");
  private totalBytes = 0;

  async checkForUpdates(): Promise<boolean> {
    if (this.status === "checking" || this.status === "downloading")
      return false;

    this.status = "checking";
    this.error = null;

    try {
      console.log("Checking for updates...");
      const update = await check();

      if (update) {
        console.log("Update available:", update.version);

        this.update = update;
        this.newVersion = update.version;
        this.status = "available";

        return true;
      } else {
        console.log("No updates available.");
        this.status = "not-available";
        return false;
      }
    } catch (err) {
      console.error("Error checking for updates:", err);

      this.error = err instanceof Error ? err.message : String(err);
      this.status = "error";
      return false;
    }
  }

  async downloadAndInstall(): Promise<void> {
    if (this.status === "downloading" || !this.update) return;

    this.status = "downloading";
    this.error = null;
    this.downloadProgress = 0;
    this.totalBytes = 0;

    try {
      console.log("Downloading update...");

      let bytesReceived = 0;
      await this.update.downloadAndInstall((event) => {
        switch (event.event) {
          case "Started":
            this.totalBytes = event.data.contentLength ?? 0;
            console.log("Download started, total size:", this.totalBytes);
            break;
          case "Progress":
            bytesReceived += event.data.chunkLength;
            this.downloadProgress = this.totalBytes > 0
              ? Math.min(Math.round((bytesReceived / this.totalBytes) * 100), 100)
              : 0;
            console.log(`Download progress: ${this.downloadProgress}%`);
            break;
          case "Finished":
            console.log("Download finished");
            this.downloadProgress = 100;
            break;
        }
      });
      console.log("Update ready to install.");
      this.status = "ready";
    } catch (err) {
      console.error("Failed to download/install update:", err);
      this.error =
        err instanceof Error ? err.message : "Failed to install update";
      this.status = "error";
    }
  }

  async relaunchApp(): Promise<void> {
    try {
      console.log("Relaunching app to apply update...");
      await relaunch();
    } catch (err) {
      console.error("Failed to relaunch app:", err);
      this.error =
        err instanceof Error ? err.message : "Failed to relaunch app";
      this.status = "error";
    }
  }

  cancelUpdate(): void {
    this.status = "idle";
    this.update = null;
    this.downloadProgress = 0;
    this.error = null;
  }

  clearError(): void {
    this.error = null;
  }

  reset(): void {
    this.status = "idle";
    this.update = null;
    this.error = null;
    this.downloadProgress = 0;
    this.newVersion = "";
  }
}

export const updaterStore = new UpdaterStore();
