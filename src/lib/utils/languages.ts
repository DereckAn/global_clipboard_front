export interface LanguageInfo {
  name: string;
  color: string;
  svgPath: string;
}

export function getLanguageInfo(language: string | null): LanguageInfo | null {
  if (!language) return null;

  // En desarrollo usa /src/assets, en producción usa la ruta del asset protocol
  const isDev = import.meta.env.DEV;
  const assetPath = (name: string) =>
    isDev ? `/src/assets/svg/${name}.svg` : `asset://localhost/svg/${name}.svg`;

  const languages: Record<string, LanguageInfo> = {
    javascript: {
      name: "JavaScript",
      color: "#F7DF1E",
      svgPath: assetPath("js"),
    },
    typescript: {
      name: "TypeScript",
      color: "#3178C6",
      svgPath: assetPath("ts"),
    },
    python: {
      name: "Python",
      color: "#3776AB",
      svgPath: assetPath("python"),
    },
    rust: {
      name: "Rust",
      color: "#CE412B",
      svgPath: assetPath("rust"),
    },
    go: {
      name: "Go",
      color: "#00ADD8",
      svgPath: assetPath("golang"),
    },
    java: {
      name: "Java",
      color: "#007396",
      svgPath: assetPath("java"),
    },
    cpp: {
      name: "C++",
      color: "#00599C",
      svgPath: assetPath("cpp"),
    },
    html: {
      name: "HTML",
      color: "#E34F26",
      svgPath: assetPath("html"),
    },
    css: {
      name: "CSS",
      color: "#1572B6",
      svgPath: assetPath("css"),
    },
    svelte: {
      name: "Svelte",
      color: "#FF3E00",
      svgPath: assetPath("svelte"),
    },
    vue: {
      name: "Vue",
      color: "#4FC08D",
      svgPath: assetPath("vue"),
    },
    react: {
      name: "React",
      color: "#61DAFB",
      svgPath: assetPath("react"),
    },
    json: {
      name: "JSON",
      color: "#000000",
      svgPath: assetPath("json"),
    },
    markdown: {
      name: "Markdown",
      color: "#083FA1",
      svgPath: assetPath("md"),
    },
    sql: {
      name: "SQL",
      color: "#4479A1",
      svgPath: assetPath("sql"),
    },
    php: {
      name: "PHP",
      color: "#777BB4",
      svgPath: assetPath("php"),
    },
    ruby: {
      name: "Ruby",
      color: "#CC342D",
      svgPath: assetPath("ruby"),
    },
    bash: {
      name: "Bash",
      color: "#4EAA25",
      svgPath: assetPath("bash"),
    },
    swift: {
      name: "Swift",
      color: "#F05138",
      svgPath: assetPath("swift"),
    },
    dart: {
      name: "Dart",
      color: "#0175C2",
      svgPath: assetPath("dart"),
    },
    docker: {
      name: "Docker",
      color: "#2496ED",
      svgPath: assetPath("docker"),
    },
    kotlin: {
      name: "Kotlin",
      color: "#7F52FF",
      svgPath: assetPath("kotlin"),
    },
    csharp: {
      name: "C#",
      color: "#239120",
      svgPath: assetPath("csharp"),
    },
  };

  return (
    languages[language.toLowerCase()] || {
      name: language.toUpperCase(),
      color: "#6B7280",
      svgPath: assetPath("code"),
    }
  );
}
