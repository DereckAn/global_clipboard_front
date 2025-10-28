<script lang="ts">
    import { cn } from '$lib/utils/cn';
    import { iconPaths } from './icons.svelte';

    interface Props {
      name: keyof typeof iconPaths
      size?: number | string
      class?: string
      strokeWidth?: number
      fill?: string
    }

    let {
      name,
      size = 24,
      class: className,
      strokeWidth = 2,
      fill = 'none'
    }: Props = $props()

    const sizeValue = typeof size === 'number' ? `${size}px` : size
  </script>

  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={sizeValue}
    height={sizeValue}
    viewBox="0 0 24 24"
    fill={fill}
    stroke="currentColor"
    stroke-width={strokeWidth}
    stroke-linecap="round"
    stroke-linejoin="round"
    class={cn('inline-block', className)}
  >
    {#each iconPaths[name].split(' M') as pathPart, i}
      {#if i === 0}
        <path d={pathPart} />
      {:else}
        <path d={'M' + pathPart} />
      {/if}
    {/each}
  </svg>