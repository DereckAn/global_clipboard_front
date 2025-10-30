 <script lang="ts">
    interface Props {
      value?: string;
      id?: string;
      onChange?: (hotkey: string) => void;
    }

    let { value = "", onChange }: Props = $props();

    let isRecording = $state(false);
    let recordedKeys = $state<string[]>([]);

    const displayText = $derived(
      isRecording
        ? recordedKeys.length > 0
          ? recordedKeys.join(" + ")
          : "Press your keys..."
        : value || "Click to record hotkey"
    );

    const handleKeyDown = (e: KeyboardEvent) => {
      if (!isRecording) return;

      e.preventDefault();
      e.stopPropagation();

      const keys: string[] = [];

      // Add modifiers in order
      if (e.metaKey || e.ctrlKey) keys.push("CommandOrControl");
      if (e.altKey) keys.push("Alt");
      if (e.shiftKey) keys.push("Shift");

      // Add main key (not a modifier)
      if (
        e.key !== "Meta" &&
        e.key !== "Control" &&
        e.key !== "Alt" &&
        e.key !== "Shift"
      ) {
        keys.push(e.key.toUpperCase());
      }

      recordedKeys = keys;
    };

    const handleKeyUp = (e: KeyboardEvent) => {
      if (!isRecording) return;

      // Only finish if we have at least a modifier + key
      if (recordedKeys.length > 1) {
        finishRecording();
      }
    };

    const startRecording = () => {
      isRecording = true;
      recordedKeys = [];
    };

    const finishRecording = () => {
      isRecording = false;
      if (recordedKeys.length > 1 && onChange) {
        const hotkey = recordedKeys.join("+");
        onChange(hotkey);
      }
    };

    const handleBlur = () => {
      if (isRecording) {
        finishRecording();
      }
    };
  </script>

  <svelte:window onkeydown={handleKeyDown} onkeyup={handleKeyUp} />

  <div class="space-y-2">
    <button
      onclick={startRecording}
      onblur={handleBlur}
      class="w-full px-4 py-2.5 bg-surface border-2 rounded-lg text-left transition-all font-mono text-sm flex items-center justify-between"
      class:border-primary={isRecording}
      class:border-border={!isRecording}
      class:bg-primary-5={isRecording}
    >
      <span class:text-text={!isRecording} class:text-primary={isRecording}>
        {displayText}
      </span>

      {#if isRecording}
        <span class="text-xs text-primary font-semibold animate-pulse">
          Recording...
        </span>
      {:else}
        <span class="text-xs text-text-muted">
          Click to change
        </span>
      {/if}
    </button>

    {#if isRecording}
      <p class="text-xs text-primary font-medium flex items-center gap-1">
        <span class="inline-block w-2 h-2 rounded-full bg-primary animate-pulse"></span>
        Press your key combination (e.g., Cmd + Shift + V)
      </p>
    {:else}
      <p class="text-xs text-text-muted">
        Click the box above and press your desired key combination
      </p>
    {/if}
  </div>

  <style>
    @keyframes pulse {
      0%, 100% {
        opacity: 1;
      }
      50% {
        opacity: 0.5;
      }
    }

    .animate-pulse {
      animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
    }
  </style>