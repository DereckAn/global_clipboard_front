<script lang="ts">
  import { cn } from '$lib/utils/cn'
  
  interface Props {
    isOpen: boolean
    onClose: () => void
    title?: string
    class?: string
    children?: any
  }
  
  let {
    isOpen,
    onClose,
    title,
    class: className,
    children
  }: Props = $props()
  
  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape' && isOpen) {
      onClose()
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
  <div class="fixed inset-0 z-50 flex items-center justify-center">
    <!-- Backdrop -->
    <div 
      class="fixed inset-0 bg-black/50 backdrop-blur-sm"
      onclick={onClose}
      role="button"
      tabindex="-1"
    />
    
    <!-- Modal -->
    <div 
      class={cn(
        'relative z-50 w-full max-w-lg rounded-lg bg-card p-6 shadow-lg',
        'border border-border',
        className
      )}
    >
      {#if title}
        <div class="mb-4 flex items-center justify-between">
          <h2 class="text-lg font-semibold">{title}</h2>
          <button
            onclick={onClose}
            class="rounded-sm opacity-70 hover:opacity-100"
          >
            ✕
          </button>
        </div>
      {/if}
      
      {@render children?.()}
    </div>
  </div>
{/if}