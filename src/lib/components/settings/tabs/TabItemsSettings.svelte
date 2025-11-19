<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";

  interface Props {
    maxItemsEnabled: boolean;
    maxLocalItems: number;
    retentionEnabled: boolean;
    retentionDays: number;
    onToggleMaxItems?: () => void;
    onUpdateMaxItems?: (value: number) => void;
    onToggleRetention?: () => void;
    onUpdateRetentionDays?: (days: number) => void;
  }

  let {
    maxItemsEnabled,
    maxLocalItems,
    retentionEnabled,
    retentionDays,
    onToggleMaxItems = () => {},
    onUpdateMaxItems = () => {},
    onToggleRetention = () => {},
    onUpdateRetentionDays = () => {},
  }: Props = $props();

  const retentionOptions = [7, 30, 90, 180, 365];
</script>

<section class=" relative bg-surface rounded-2xl border border-border/60 p-6 space-y-6">
  <h2 class="text-lg font-semibold flex items-center gap-2">
    <Icon name="trash" size={20} />
    Item Management
  </h2>

  <div class="flex items-center justify-between">
    <div>
      <p class="block text-sm font-medium text-text mb-1">
        Limitar items guardados
      </p>
      <p class="text-xs text-text-muted">
        Auto-eliminar items antiguos cuando se supera el límite
      </p>
    </div>
    <button
      onclick={onToggleMaxItems}
      class={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${maxItemsEnabled ? "bg-primary" : "bg-border"}`}
      role="switch"
      aria-label="Toggle saved items limit"
      aria-checked={maxItemsEnabled}
    >
      <span
        class={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
          maxItemsEnabled ? "translate-x-6" : "translate-x-1"
        }`}
      ></span>
    </button>
  </div>

  {#if maxItemsEnabled}
    <div>
      <label
        for="max-items-range"
        class="block text-sm font-medium text-text mb-2"
      >
        Máximo: {maxLocalItems} items
      </label>
      <input
        id="max-items-range"
        type="range"
        min="100"
        max="5000"
        step="100"
        value={maxLocalItems}
        oninput={(e) => onUpdateMaxItems(parseInt(e.currentTarget.value, 10))}
        class="w-full"
      />
      <div class="flex justify-between text-xs text-text-muted mt-1">
        <span>100</span>
        <span>1000</span>
        <span>5000</span>
      </div>
    </div>
  {/if}

  <div class="flex items-center justify-between">
    <div>
      <p class="block text-sm font-medium text-text mb-1">
        Auto-eliminar items antiguos
      </p>
      <p class="text-xs text-text-muted">
        Eliminar items después de cierto tiempo
      </p>
    </div>
    <button
      onclick={onToggleRetention}
      class={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
        retentionEnabled ? "bg-primary" : "bg-border"
      }`}
      role="switch"
      aria-label="Toggle retention policy"
      aria-checked={retentionEnabled}
    >
      <span
        class={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
          retentionEnabled ? "translate-x-6" : "translate-x-1"
        }`}
      ></span>
    </button>
  </div>

  {#if retentionEnabled}
    <div class="grid grid-cols-3 gap-2">
      {#each retentionOptions as days}
        <button
          onclick={() => onUpdateRetentionDays(days)}
          class={`px-3 py-2 text-sm rounded transition-colors ${
            retentionDays === days
              ? "bg-primary text-white"
              : "bg-background text-text-muted hover:bg-surface-hover"
          }`}
        >
          {days === 7
            ? "7 días"
            : days === 30
              ? "1 mes"
              : days === 90
                ? "3 meses"
                : days === 180
                  ? "6 meses"
                  : "1 año"}
        </button>
      {/each}
    </div>
  {/if}
</section>
