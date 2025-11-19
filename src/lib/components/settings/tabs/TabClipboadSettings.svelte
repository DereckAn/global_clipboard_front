<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import HotkeyRecorder from "../HotkeyRecorder.svelte";

  interface Props {
    maxLocalItems: string;
    autoSaveClipboard: boolean;
    globalHotkey: string;
    hotkeyFeedback: { type: "success" | "error"; message: string } | null;
  }

  let {
    maxLocalItems = $bindable(""),
    autoSaveClipboard = $bindable(false),
    globalHotkey = $bindable(""),
    hotkeyFeedback,
  }: Props = $props();

</script>

<section class="bg-surface rounded-2xl border border-border/60 p-6 space-y-6 relative">
  <h2 class="text-lg font-semibold flex items-center gap-2">
    <Icon name="text" size={20} />
    Clipboard
  </h2>
  <div>
    <label
      for="max-items-input"
      class="block text-sm font-medium text-text mb-2"
    >
      Maximum local items
    </label>
    <Input
      type="text"
      bind:value={maxLocalItems}
      placeholder="1000"
      class="w-32"
    />
    <p class="text-xs text-text-muted mt-1">
      Maximum number of items to store locally
    </p>
  </div>

  <div class="flex items-center justify-between">
    <div>
      <p class="block text-sm font-medium text-text mb-1">
        Auto-save clipboard
      </p>
      <p class="text-xs text-text-muted">
        Automatically save copied content to history
      </p>
    </div>
    <button
      onclick={() => (autoSaveClipboard = !autoSaveClipboard)}
      class={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${autoSaveClipboard ? "bg-primary" : "bg-border"}`}
      role="switch"
      aria-checked={autoSaveClipboard}
      aria-label="Toggle auto-save clipboard"
    >
      <span
        class={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
          autoSaveClipboard ? "translate-x-6" : "translate-x-1"
        }`}
      ></span>
    </button>
  </div>

  <div>
    <label for="global-hotkey" class="block text-sm font-medium text-text mb-2">
      Global window hotkey
    </label>
    <HotkeyRecorder
      id="global-hotkey"
      value={globalHotkey}
      onChange={(newHotkey) => (globalHotkey = newHotkey)}
    />

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
