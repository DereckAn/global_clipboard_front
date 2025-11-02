/**
 * Resalta las ocurrencias de un query en un texto
 * @param text - El texto donde buscar
 * @param query - La palabra o frase a resaltar
 * @returns Array de segmentos con flag de si están resaltados
 */
export function highlightText(
  text: string,
  query: string
): Array<{
  text: string;
  highlight: boolean;
}> {
  if (!query.trim() || !text) {
    return [{ text, highlight: false }];
  }

  const segments: Array<{ text: string; highlight: boolean }> = [];
  const regex = new RegExp(`(${escapeRegex(query)})`, "gi");
  const parts = text.split(regex);

  for (const part of parts) {
    if (part) {
      const isMatch = part.toLowerCase() === query.toLowerCase();
      segments.push({
        text: part,
        highlight: isMatch,
      });
    }
  }

  return segments;
}

/**
 * Escapa caracteres especiales de regex
 */
function escapeRegex(str: string): string {
  return str.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}