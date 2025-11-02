<script lang="ts">
  import { highlightText } from "$lib/utils/highlight";

  interface Props {
    text: string;
    query: string;
    class?: string;
  }

  let { text, query, class: className }: Props = $props();

  const segments = $derived(highlightText(text, query));
</script>

{#each segments as segment}
  {#if segment.highlight}
    <mark class="bg-yellow-300/30 text-yellow-100 rounded px-0.5">
      {segment.text}
    </mark>
  {:else}
    <span class={className}>{segment.text}</span>
  {/if}
{/each}
