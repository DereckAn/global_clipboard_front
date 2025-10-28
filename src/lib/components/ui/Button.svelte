<script lang="ts">
  import { cn } from '$lib/utils/cn'
  
  interface Props {
    variant?: 'default' | 'destructive' | 'outline' | 'ghost'
    size?: 'default' | 'sm' | 'lg' | 'icon'
    class?: string
    children?: any
    onclick?: (e: MouseEvent) => void
    disabled?: boolean
    type?: 'button' | 'submit' | 'reset'
  }
  
  let {
    variant = 'default',
    size = 'default',
    class: className,
    children,
    onclick,
    disabled = false,
    type = 'button',
    ...restProps
  }: Props = $props()
</script>

<button
  {type}
  {disabled}
  {onclick}
  class={cn(
    // Base
    'inline-flex items-center justify-center rounded-md font-medium transition-colors',
    'focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring',
    'disabled:pointer-events-none disabled:opacity-50',
    
    // Variants
    variant === 'default' && 'bg-primary text-primary-foreground hover:bg-primary/90',
    variant === 'destructive' && 'bg-destructive text-destructive-foreground hover:bg-destructive/90',
    variant === 'outline' && 'border border-input bg-background hover:bg-accent hover:text-accent-foreground',
    variant === 'ghost' && 'hover:bg-accent hover:text-accent-foreground',
    
    // Sizes
    size === 'default' && 'h-10 px-4 py-2',
    size === 'sm' && 'h-9 rounded-md px-3',
    size === 'lg' && 'h-11 rounded-md px-8',
    size === 'icon' && 'h-10 w-10',
    
    className
  )}
  {...restProps}
>
  {@render children?.()}
</button>