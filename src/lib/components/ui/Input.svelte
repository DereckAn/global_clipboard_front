<script lang="ts">
    import { cn } from '$lib/utils/cn'

    interface Props {
      type?: string
      placeholder?: string
      class?: string
      disabled?: boolean
      value?: string
    }

    let {
      type = 'text',
      placeholder = '',
      class: className,
      disabled = false,
      value = $bindable('')
    }: Props = $props()

    const baseStyles = `flex h-10 w-full rounded-md border border-[var(--color-border)] bg-[var(--color-surface)] px-3 py-2
        text-sm text-[var(--color-text)] placeholder:text-[var(--color-text-muted)] focus-visible:outline-none
        focus-visible:ring-2 focus-visible:ring-[var(--color-primary)] focus-visible:ring-offset-2 disabled:cursor-not-allowed
        disabled:opacity-50 transition-colors`
  </script>

  <input
    {type}
    {placeholder}
    {disabled}
    bind:value
    class={cn(baseStyles, className)}
  />