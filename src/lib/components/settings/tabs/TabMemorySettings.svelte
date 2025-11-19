<script>
  import Icon from "$lib/components/icons/Icon.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { settingsStore } from "$lib/stores/settings.svelte";
</script>

<section
  class="relative bg-surface rounded-2xl border border-border/60 p-6 space-y-4"
>
  <h2 class="text-lg font-semibold flex items-center gap-2">
    <Icon name="database" size={20} />
    Memory Usage
  </h2>
  <button
    onclick={async () => await settingsStore.loadDatabaseStats()}
    class="text-sm text-primary hover:underline"
  >
    Actualizar estadísticas
  </button>
  {#if settingsStore.dbStats}
    <div class="space-y-2">
      <div class="flex justify-between text-sm">
        <span class="text-text-muted">Total de items:</span>
        <span class="text-text font-medium">
          {settingsStore.dbStats.total_items}
        </span>
      </div>
      <div class="flex justify-between text-sm">
        <span class="text-text-muted">Favoritos:</span>
        <span class="text-text font-medium">
          {settingsStore.dbStats.favorites}
        </span>
      </div>
      <div class="flex justify-between text-sm">
        <span class="text-text-muted">Snippets:</span>
        <span class="text-text font-medium">
          {settingsStore.dbStats.snippets}
        </span>
      </div>
      <div class="flex justify-between text-sm">
        <span class="text-text-muted">Tamaño en disco:</span>
        <span class="text-text font-medium">
          {settingsStore.dbStats.database_size_mb.toFixed(2)} MB
        </span>
      </div>
    </div>
    <div class="flex gap-2">
      <Button
        size="sm"
        class="h-fit py-1"
        onclick={async () => {
          const deleted = await settingsStore.cleanupOldItems();
          alert(`${deleted} items eliminados`);
        }}
      >
        Limpiar items antiguos
      </Button>
      <Button
        size="sm"
         class="h-fit py-1"
        onclick={async () => {
          await settingsStore.optimizeDatabase();
          alert("Base de datos optimizada");
        }}
      >
        Optimizar DB
      </Button>
    </div>
  {/if}
</section>
