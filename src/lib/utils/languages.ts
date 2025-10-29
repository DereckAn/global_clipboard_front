export interface LanguageInfo {
  name: string;
  color: string;
  iconName: keyof typeof import("$lib/components/icons/icons.svelte").iconPaths;
}

export function getLanguageInfo(language: string | null): LanguageInfo | null {
  if (!language) return null;

  const languages: Record<string, LanguageInfo> = {
    javascript: {
      name: "JavaScript",
      color: "#F7DF1E",
      iconName: "javascript",
    },
    typescript: {
      name: "TypeScript",
      color: "#3178C6",
      iconName: "typescript",
    },
    python: { name: "Python", color: "#3776AB", iconName: "python" },
    rust: { name: "Rust", color: "#CE412B", iconName: "rust" },
    go: { name: "Go", color: "#00ADD8", iconName: "golang" },
    java: { name: "Java", color: "#007396", iconName: "java" },
    cpp: { name: "C++", color: "#00599C", iconName: "cpp" },
    html: { name: "HTML", color: "#E34F26", iconName: "html" },
    css: { name: "CSS", color: "#1572B6", iconName: "css" },
    svelte: { name: "Svelte", color: "#FF3E00", iconName: "svelte" },
    vue: { name: "Vue", color: "#4FC08D", iconName: "vue" },
    react: { name: "React", color: "#61DAFB", iconName: "react" },
    json: { name: "JSON", color: "#000000", iconName: "json" },
    markdown: { name: "Markdown", color: "#083FA1", iconName: "markdown" },
    sql: { name: "SQL", color: "#4479A1", iconName: "sql" },
    php: { name: "PHP", color: "#777BB4", iconName: "php" },
    ruby: { name: "Ruby", color: "#CC342D", iconName: "ruby" },
    bash: { name: "Bash", color: "#4EAA25", iconName: "bash" },
  };

  return (
    languages[language.toLowerCase()] || {
      name: language.toUpperCase(),
      color: "#6B7280",
      iconName: "code", // Fallback icon
    }
  );
}
