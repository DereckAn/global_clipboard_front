<script lang="ts">
  import { cn } from "$lib/utils/cn";

  interface Props {
    variant?: "default" | "destructive" | "outline" | "ghost" | "other";
    size?: "default" | "sm" | "lg" | "icon" | "other";
    class?: string;
    onclick?: (event: MouseEvent) => void;
    disabled?: boolean;
    type?: "button" | "submit" | "reset";
    children?: import("svelte").Snippet;
    onblur?: (event: FocusEvent) => void;
    id?: string;
  }

  let {
    variant = "default",
    size = "default",
    class: className,
    onclick,
    disabled = false,
    type = "button",
    children,
    onblur,
    id,
  }: Props = $props();

  // Base estilo “píldora de vidrio”
  const baseStyles = [
    "inline-flex items-center justify-center gap-2",
    "rounded-full text-[13px] font-medium select-none",
    "border",
    "transition-colors transition-shadow duration-150",
    "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-1 focus-visible:ring-sky-500",
    "disabled:opacity-60 disabled:cursor-default",
    // Liquid Glass vibes
    "bg-clip-padding backdrop-blur-md",
  ].join(" ");

  const variants: Record<NonNullable<Props["variant"]>, string> = {
    // Botón principal (tinte azul, líquido)
    default: ["bg-white/10 text-white rounded-md hover:bg-white/20"].join(" "),

    // Botón “vidrio neutro”, ideal para secundarios
    outline: [
      "text-white/70 rounded-xl",
      "border-white/30",
      "shadow-[0_0_0_1px_rgba(255,255,255,0.5)_inset,0_8px_20px_rgba(15,23,42,0.4)]",
      "hover:bg-white/65 dark:hover:bg-slate-900/65",
      "active:bg-white/75 dark:active:bg-slate-900/75",
    ].join(" "),

    // Botón casi invisible, solo sugerido
    ghost: ["h-fit text-sm rounded-md z-10 hover:bg-white/5"].join(" "),

    // Botón de peligro con tinte rojo líquido
    destructive: [
      "bg-red-500/30 text-white rounded-xl hover:bg-red-500/50 text-sm flex items-center justify-center gap-2 text-nowrap",
    ].join(" "),

    other: "",
  };

  const sizes: Record<NonNullable<Props["size"]>, string> = {
    default: "p-1 px-4 text-[14px]",
    sm: "px-3 text-[12px]",
    lg: "h-10 px-5 text-[14px]",
    icon: "h-8 w-8",
    other: "py-1 px-2 text-[12px]",
  };
</script>

<button
  {id}
  {type}
  {disabled}
  {onclick}
  class={cn(variants[variant], sizes[size], className)}
  {onblur}
>
  {@render children?.()}
</button>
