<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";
  import HotkeyRecorder from "../HotkeyRecorder.svelte";

  interface Props {
    globalHotkey: string;
    hotkeyFeedback: { type: "success" | "error"; message: string } | null;
  }

  let { globalHotkey = $bindable(""), hotkeyFeedback }: Props = $props();

</script>

<section class="bg-surface rounded-2xl border border-border/60 p-6 space-y-6 relative">
  <h2 class="text-lg font-semibold flex items-center gap-2">
    <Icon name="text" size={20} />
    Clipboard
  </h2>
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
