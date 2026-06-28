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
 * Normalizes an SVG so it scales to its container instead of rendering at its
 * intrinsic size. Ensures a viewBox exists (so the vector keeps its aspect
 * ratio) and removes the fixed width/height attributes (so CSS controls the
 * rendered size). Returns the input unchanged if it can't be parsed.
 */
export function normalizeSvgSize(svgContent: string): string {
  if (typeof window === "undefined") return svgContent;

  try {
    const doc = new DOMParser().parseFromString(svgContent, "image/svg+xml");
    const svg = doc.querySelector("svg");
    if (!svg || doc.querySelector("parsererror")) return svgContent;

    // Derive a viewBox from width/height when one is missing, so the vector
    // can scale while keeping its aspect ratio.
    if (!svg.getAttribute("viewBox")) {
      const rawWidth = svg.getAttribute("width") ?? "";
      const rawHeight = svg.getAttribute("height") ?? "";
      const width = parseFloat(rawWidth);
      const height = parseFloat(rawHeight);
      const usable =
        !rawWidth.includes("%") &&
        !rawHeight.includes("%") &&
        width > 0 &&
        height > 0;
      if (usable) {
        svg.setAttribute("viewBox", `0 0 ${width} ${height}`);
      }
    }

    // Let CSS decide the rendered size.
    svg.removeAttribute("width");
    svg.removeAttribute("height");

    return svg.outerHTML;
  } catch {
    return svgContent;
  }
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
