<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";
  import { settingsStore } from "$lib/stores/settings.svelte";
  import HotkeyRecorder from "../HotkeyRecorder.svelte";

  interface Props {
    globalHotkey: string;
    screenshotHotkeyFull: string;
    screenshotHotkeyRegion: string;
    hotkeyFeedback: { type: "success" | "error"; message: string } | null;
  }

  let {
    globalHotkey = $bindable(""),
    screenshotHotkeyFull = $bindable(""),
    screenshotHotkeyRegion = $bindable(""),
    hotkeyFeedback,
  }: Props = $props();

  const screenshotFeature = $derived.by(() =>
    settingsStore.labFeatures.find((f) => f.id === "screenshot")
  );

  const screenshotAvailable = $derived.by(
    () =>
      !!(
        screenshotFeature &&
        screenshotFeature.status === "installed" &&
        screenshotFeature.enabled
      )
  );

  const handleGlobalChange = async (newHotkey: string) => {
    globalHotkey = newHotkey;
    try {
      await settingsStore.saveHotkey(newHotkey);
    } catch (err) {
      console.error("Failed to save global hotkey:", err);
    }
  };

  const handleFullChange = (newHotkey: string) => {
    screenshotHotkeyFull = newHotkey;
    settingsStore.updateScreenshotHotkeys(newHotkey, screenshotHotkeyRegion);
  };

  const handleRegionChange = (newHotkey: string) => {
    screenshotHotkeyRegion = newHotkey;
    settingsStore.updateScreenshotHotkeys(screenshotHotkeyFull, newHotkey);
  };

</script>

<section
  class="bg-surface rounded-2xl border border-border/60 p-3 space-y-4 relative"
>
  <div class="flex items-center gap-2">
    <Icon name="text" size={20} />
    <div>
      <h2 class="text-lg font-semibold">Hotkeys</h2>
      <p class="text-xs text-text-muted">
        Configure global shortcuts and helper actions.
      </p>
    </div>
  </div>

  <div class="space-y-6 mt-6">
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="size-10 rounded-xl bg-white/5 flex items-center justify-center">
          <Icon name="text" size={18} />
        </div>
        <div>
          <p class="text-sm font-semibold text-text">Show/Hide Window</p>
          <p class="text-xs text-text-muted">Global hotkey to toggle the app.</p>
        </div>
      </div>
      <HotkeyRecorder
        id="global-hotkey"
        value={globalHotkey}
        onChange={handleGlobalChange}
      />
    </div>

    {#if screenshotAvailable}
      <div
        class="flex items-center justify-between bg-surface-200/40 border border-border/60 rounded-2xl px-4 py-3"
      >
        <div class="flex items-center gap-3">
          <div
            class="h-10 w-10 rounded-xl bg-surface-100/50 border border-border/60 flex items-center justify-center"
          >
            <Icon name="image" size={18} />
          </div>
          <div>
            <p class="text-sm font-semibold text-text">
              Screenshot (full screen)
            </p>
            <p class="text-xs text-text-muted">
              Runs the helper for full-screen capture.
            </p>
          </div>
        </div>
        <HotkeyRecorder
          id="screenshot-full"
          value={screenshotHotkeyFull}
          onChange={handleFullChange}
          disabled={!screenshotAvailable}
        />
      </div>

      <div
        class="flex items-center justify-between bg-surface-200/40 border border-border/60 rounded-2xl px-4 py-3"
      >
        <div class="flex items-center gap-3">
          <div
            class="h-10 w-10 rounded-xl bg-surface-100/50 border border-border/60 flex items-center justify-center"
          >
            <Icon name="image" size={18} />
          </div>
          <div>
            <p class="text-sm font-semibold text-text">Screenshot (region)</p>
            <p class="text-xs text-text-muted">
              Runs the helper for region capture.
            </p>
          </div>
        </div>
        <HotkeyRecorder
          id="screenshot-region"
          value={screenshotHotkeyRegion}
          onChange={handleRegionChange}
          disabled={!screenshotAvailable}
        />
      </div>
    {/if}

    {#if !screenshotAvailable}
      <div class="flex items-center gap-2 text-xs text-text-muted px-1">
        <Icon name="info" size={14} />
        <span
          >Install and enable Screenshot in Laboratory to configure its hotkeys.</span
        >
      </div>
    {/if}

    {#if hotkeyFeedback}
      <div
        class="mt-3 px-3 py-2 rounded-lg text-sm flex items-center gap-2"
        class:bg-primary-20={hotkeyFeedback.type === "success"}
        class:text-primary={hotkeyFeedback.type === "success"}
        class:bg-danger-20={hotkeyFeedback.type === "error"}
        class:text-danger={hotkeyFeedback.type === "error"}
      >
        {#if hotkeyFeedback.type === "success"}
          <Icon name="check" size={16} />
        {:else}
          <Icon name="x" size={16} />
        {/if}
        <span>{hotkeyFeedback.message}</span>
      </div>
    {/if}
  </div>
</section>
