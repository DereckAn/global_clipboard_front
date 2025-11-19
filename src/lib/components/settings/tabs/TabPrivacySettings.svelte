<script lang="ts">
  import Icon from "$lib/components/icons/Icon.svelte";

  interface Props {
    enableAnalytics: boolean;
  }

  let { enableAnalytics = $bindable<boolean>() }: Props = $props();
</script>

<section class="relative bg-surface rounded-2xl border border-border/60 p-6">
  <h2 class="text-lg font-semibold mb-4 flex items-center gap-2">
    <Icon name="lock" size={20} />
    Privacy
  </h2>
  <div class="flex items-center justify-between">
    <div>
      <p class="block text-sm font-medium text-text mb-1">Enable analytics</p>
      <p class="text-xs text-text-muted">
        Help improve the app by sharing anonymous usage data
      </p>
    </div>
    <button
      onclick={() => (enableAnalytics = !enableAnalytics)}
      class={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${enableAnalytics ? "bg-primary" : "bg-border"}`}
      role="switch"
      aria-label="Toggle analytics"
      aria-checked={enableAnalytics}
    >
      <span
        class={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
          enableAnalytics ? "translate-x-6" : "translate-x-1"
        }`}
      ></span>
    </button>
  </div>
</section>
