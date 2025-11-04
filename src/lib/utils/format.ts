import type { ContentType } from "$lib/types";

export function formatDate(date: Date): string {
  const now = new Date();
  const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);

  if (diffInSeconds < 60) {
    return "Just now";
  }

  const diffInMinutes = Math.floor(diffInSeconds / 60);
  if (diffInMinutes < 60) {
    return `${diffInMinutes}m ago`;
  }

  const diffInHours = Math.floor(diffInMinutes / 60);
  if (diffInHours < 24) {
    return `${diffInHours}h ago`;
  }

  const diffInDays = Math.floor(diffInHours / 24);
  if (diffInDays < 7) {
    return `${diffInDays}d ago`;
  }

  // Format as date
  return date.toLocaleDateString("en-US", {
    month: "short",
    day: "numeric",
    year: date.getFullYear() !== now.getFullYear() ? "numeric" : undefined,
  });
}

export function truncateText(text: string, maxLength: number = 100): string {
  if (text.length <= maxLength) {
    return text;
  }
  return text.slice(0, maxLength) + "...";
}

export function formatFileSize(bytes: number): string {
  if (bytes === 0) return "0 B";

  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} 
  ${sizes[i]}`;
}

// Retorna el nombre del icono en lugar de emoji
export function getContentTypeIcon(
  type: ContentType
): keyof typeof import("$lib/components/icons/icons.svelte").iconPaths {
  const iconMap: Record<
    ContentType,
    keyof typeof import("$lib/components/icons/icons.svelte").iconPaths
  > = {
    text: "text",
    code: "code",
    link: "link",
    image: "image",
    file: "file",
    color: "color",
    svg: "image", // Use image icon for SVG
  };

  return iconMap[type] || "text";
}

// Nueva función para detectar tipo de color
export function detectColorFormat(colorString: string): string {
  if (/^#[0-9A-Fa-f]{6}$/.test(colorString)) return "HEX";
  if (/^#[0-9A-Fa-f]{8}$/.test(colorString)) return "HEX (with alpha)";
  if (/^rgb\(/.test(colorString)) return "RGB";
  if (/^rgba\(/.test(colorString)) return "RGBA";
  if (/^hsl\(/.test(colorString)) return "HSL";
  if (/^hsla\(/.test(colorString)) return "HSLA";
  return "Unknown";
}

// Nueva función para contar caracteres
export function getCharacterCount(text: string | null): number {
  return text?.length || 0;
}

// Nueva función para contar palabras
export function getWordCount(text: string | null): number {
  if (!text) return 0;
  return text.trim().split(/\s+/).length;
}
