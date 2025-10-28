<script lang="ts">
  import { cn } from '$lib/utils/cn'
  
  interface Props {
    class?: string
    size?: 'sm' | 'md' | 'lg'
  }
  
  let { class: className, size = 'md' }: Props = $props()
</script>

<div
  class={cn(
    'animate-spin rounded-full border-2 border-current border-t-transparent',
    size === 'sm' && 'h-4 w-4',
    size === 'md' && 'h-8 w-8',
    size === 'lg' && 'h-12 w-12',
    className
  )}
  role="status"
  aria-label="Loading"
>
  <span class="sr-only">Loading...</span>
</div>