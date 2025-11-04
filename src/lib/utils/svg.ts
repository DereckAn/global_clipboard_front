import DOMPurify from "dompurify";

/**
 * Sanitizes SVG content to prevent XSS attacks
 * Removes script tags, event handlers, and other potentially dangerous content
 */
export function sanitizeSvg(svgContent: string): string {
  // Configure DOMPurify for SVG
  const clean = DOMPurify.sanitize(svgContent, {
    USE_PROFILES: { svg: true, svgFilters: true },
    ADD_TAGS: ["use"], // Allow <use> tag for SVG references
    ADD_ATTR: ["target"], // Allow target attribute for links
    FORBID_TAGS: ["script", "iframe", "object", "embed"], // Explicitly forbid dangerous tags
    FORBID_ATTR: ["onerror", "onload", "onclick", "onmouseover", "onmouseout"], // Forbid event handlers
  });

  return clean;
}

/**
 * Validates if a string is a valid SVG
 */
export function isValidSvg(content: string): boolean {
  const trimmed = content.trim();
  return (
    trimmed.startsWith("<svg") ||
    (trimmed.startsWith("<?xml") && trimmed.includes("<svg"))
  );
}
