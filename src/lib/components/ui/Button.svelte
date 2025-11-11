<script lang="ts">
    import { cn } from '$lib/utils/cn'

    interface Props {
      variant?: 'default' | 'destructive' | 'outline' | 'ghost'
      size?: 'default' | 'sm' | 'lg' | 'icon'
      class?: string
      onclick?: (event: MouseEvent) => void
      disabled?: boolean
      type?: 'button' | 'submit' | 'reset'
      children?: import('svelte').Snippet
    }

    let {
      variant = 'default',
      size = 'default',
      class: className,
      onclick,
      disabled = false,
      type = 'button',
      children
    }: Props = $props()

    // Base styles
    const baseStyles = `inline-flex items-center justify-center rounded-md font-medium transition-colors focus-visible:outline-none 
    focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50`

    // Variant styles
    const variants = {
      default: 'text-white bg-gray-700 rounded-lg text-sm hover:bg-gray-800 transition-colors flex items-center justify-center gap-2',
      destructive: 'bg-[var(--color-danger)] text-white hover:bg-red-600',
      outline: 'ring ring-border text-sm bg-transparent hover:bg-surface-hover',
      ghost: 'hover:bg-[var(--color-surface-hover)] bg-transparent',
    }

    // Size styles
    const sizes = {
      default: 'px-4 py-1',
      sm: 'h-8 px-3 text-sm',
      lg: 'h-12 px-8',
      icon: 'h-9 w-9'
    }
  </script>

  <button
    {type}
    {disabled}
    {onclick}
    class={cn(baseStyles, variants[variant], sizes[size], className)}
  >
    {@render children?.()}
  </button>