<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface Props {
    value?: string;
    id?: string;
    onChange?: (hotkey: string) => void;
    disabled?: boolean;
  }

  let { value = "", onChange, disabled = false }: Props = $props();
  let isRecording = $state(false);
  let recordedKeys = $state<string[]>([]);
  let currentOS = $state<string>("macos");

  // Símbolos según el sistema operativo
  const SYMBOLS_BY_OS: Record<string, Record<string, string>> = {
    macos: {
      Command: "⌘",
      Control: "⌃",
      Option: "⌥",
      Shift: "⇧",
    },
    windows: {
      Command: "Win",
      Control: "Ctrl",
      Alt: "Alt",
      Shift: "⇧",
    },
    linux: {
      Command: "Super",
      Control: "Ctrl",
      Alt: "Alt",
      Shift: "⇧",
    },
  };

  const MODIFIER_ORDER = ["Command", "Control", "Alt", "Option", "Shift"];

  onMount(async () => {
    try {
      const platform = await invoke<string>("get_platform");
      currentOS = platform;
      console.log("Platform detected:", platform);
    } catch (err) {
      console.error("Failed to detect platform:", err);
      currentOS = "macos"; // fallback
    }
  });

  const getSymbols = () => {
    return SYMBOLS_BY_OS[currentOS] || SYMBOLS_BY_OS.macos;
  };

  const formatHotkey = (hotkey: string, keys: string[]) => {
    const parts = keys.length
      ? keys
      : hotkey
          .split("+")
          .map((k) => k.trim())
          .filter(Boolean);

    if (!parts.length) return "Click to record hotkey";

    const symbols = getSymbols();
    return parts.map((part) => symbols[part] ?? part.toUpperCase()).join(" ");
  };

  const isModifier = (key: string) =>
    key === "Command" ||
    key === "Control" ||
    key === "Alt" ||
    key === "Option" ||
    key === "Shift";

  const displayText = $derived.by(() =>
    isRecording
      ? recordedKeys.length > 0
        ? formatHotkey("", recordedKeys)
        : "Press keys…"
      : formatHotkey(value, [])
  );

  const startRecording = () => {
    if (disabled) return;
    isRecording = true;
    recordedKeys = [];
  };

  const cancelRecording = () => {
    isRecording = false;
    recordedKeys = [];
  };

  const finishRecording = () => {
    isRecording = false;
    const hasMain = recordedKeys.some((k) => !isModifier(k));
    const hasModifierKey = recordedKeys.some(isModifier);

    if (recordedKeys.length > 0 && hasMain && hasModifierKey && onChange) {
      const hotkey = recordedKeys.join("+");
      onChange(hotkey);
    }
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    if (!isRecording || disabled) return;

    if (e.key === "Escape") {
      cancelRecording();
      return;
    }

    e.preventDefault();
    e.stopPropagation();

    const keys: string[] = [];

    // Recolectar modificadores
    if (e.metaKey) keys.push("Command");
    if (e.ctrlKey) keys.push("Control");
    if (e.altKey) keys.push("Alt");
    if (e.shiftKey) keys.push("Shift");

    if (
      e.key !== "Meta" &&
      e.key !== "Control" &&
      e.key !== "Alt" &&
      e.key !== "Shift"
    ) {
      keys.push(e.key.toUpperCase());
    }

    // Ordenar modificadores consistentemente
    const mods = MODIFIER_ORDER.filter((m) => keys.includes(m));
    const main = keys.filter((k) => !isModifier(k));
    recordedKeys = [...mods, ...main];
  };

  const handleKeyUp = () => {
    if (!isRecording || disabled) return;
    const hasMain = recordedKeys.some((k) => !isModifier(k));
    const hasModifierKey = recordedKeys.some(isModifier);

    if (recordedKeys.length > 0 && hasMain && hasModifierKey) {
      finishRecording();
    }
  };

  const handleBlur = () => {
    if (isRecording) {
      finishRecording();
    }
  };
</script>

<svelte:window onkeydown={handleKeyDown} onkeyup={handleKeyUp} />

<div class="flex items-center gap-2">
  <span
    class="px-3 py-1 rounded-md border border-border/60 text-sm text-no-wrap min-w-[100px] text-center"
  >
    {displayText}
  </span>
  <button
    class={`px-3 py-1 rounded-md bg-white/10 text-sm ${
      isRecording
        ? "border-primary text-primary bg-primary/10 animate-pulse"
        : "border-border/60 text-text bg-surface-100/40 hover:bg-white/20"
    }`}
    onclick={startRecording}
    onblur={handleBlur}
    id="edit-hotkey-button"
    {disabled}
  >
    {isRecording ? "Recording…" : "Edit"}
  </button>
</div>

<style>
  @keyframes pulse {
    0%,
    100% {
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
