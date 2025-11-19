<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";

</script>

<section class="relative bg-surface rounded-2xl border border-border/60 p-6 space-y-3">
  <h2 class="text-lg font-semibold mb-4 flex items-center gap-2">
    <Icon name="info" size={20} />
    About
  </h2>
  <div class="flex justify-between">
    <span class="text-sm text-text-muted">Version</span>
    <span class="text-sm text-text">1.0.0</span>
  </div>
  <div class="flex justify-between">
    <span class="text-sm text-text-muted">Platform</span>
    <span class="text-sm text-text">Tauri v2</span>
  </div>
</section>
