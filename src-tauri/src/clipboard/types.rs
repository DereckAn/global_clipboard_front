/// Detecta el tipo de contenido basado en el texto
pub fn detect_content_type(text: &str) -> String {
    // URL detection
    if text.starts_with("http://") || text.starts_with("https://") {
        return "link".to_string();
    }

    // Color detection (hex, rgb, rgba, hsl)
    if is_color(text) {
        return "color".to_string();
    }

    // Code detection (simple heuristic)
    if is_code(text) {
        return "code".to_string();
    }

    // Default to text
    "text".to_string()
}

/// Detecta el lenguaje de programación basado en el contenido
pub fn detect_code_language(text: &str) -> Option<String> {
    // HTML/XML
    if text.contains("<!DOCTYPE")
        || text.contains("<html")
        || text.contains("</") && text.contains("/>")
    {
        return Some("html".to_string());
    }

    // Svelte (component with script tags)
    if text.contains("<script")
        && text.contains("</script>")
        && (text.contains("$:") || text.contains("export let"))
    {
        return Some("svelte".to_string());
    }

    // Vue
    if text.contains("<template>") || text.contains("<script setup>") {
        return Some("vue".to_string());
    }

    // React/JSX
    if (text.contains("import React")
        || text.contains("from 'react'")
        || text.contains("from \"react\""))
        || (text.contains("className=") && text.contains("=>"))
    {
        return Some("react".to_string());
    }

    // TypeScript
    if text.contains("interface ")
        || text.contains("type ") && text.contains(":")
        || text.contains(": string")
        || text.contains(": number")
    {
        return Some("typescript".to_string());
    }

    // JavaScript
    if text.contains("function ")
        || text.contains("const ")
        || text.contains("let ")
        || text.contains("var ")
        || text.contains("=>")
    {
        return Some("javascript".to_string());
    }

    // Python
    if text.contains("def ")
        || text.contains("import ") && !text.contains("from \"")
        || text.contains("print(")
        || text.contains("self.")
    {
        return Some("python".to_string());
    }

    // Rust
    if text.contains("fn ")
        || text.contains("impl ")
        || text.contains("struct ")
        || text.contains("pub ")
        || text.contains("let mut")
    {
        return Some("rust".to_string());
    }

    // Go
    if text.contains("package ")
        || text.contains("func ") && text.contains("{")
        || text.contains("import (")
    {
        return Some("go".to_string());
    }

    // Java
    if text.contains("public class ")
        || text.contains(
            "public 
  static void main",
        )
        || text.contains("System.out.println")
    {
        return Some("java".to_string());
    }

    // C/C++
    if text.contains("#include") || text.contains("int main(") || text.contains("std::") {
        return Some("cpp".to_string());
    }

    // CSS
    if text.contains("{")
        && text.contains("}")
        && (text.contains(":") && text.contains(";"))
        && !text.contains("function")
        && !text.contains("const")
    {
        return Some("css".to_string());
    }

    // JSON
    if (text.trim().starts_with("{") && text.trim().ends_with("}"))
        || (text.trim().starts_with("[") && text.trim().ends_with("]"))
    {
        if text.contains("\":") || text.contains("\": ") {
            return Some("json".to_string());
        }
    }

    // Markdown
    if text.contains("# ") || text.contains("## ") || text.contains("```") {
        return Some("markdown".to_string());
    }

    // SQL
    if text.to_uppercase().contains("SELECT ")
        || text.to_uppercase().contains("INSERT INTO")
        || text.to_uppercase().contains("CREATE TABLE")
    {
        return Some("sql".to_string());
    }

    // PHP
    if text.contains("<?php") || text.contains("$_") {
        return Some("php".to_string());
    }

    // Ruby
    if text.contains("def ") && text.contains("end") || text.contains("puts ") {
        return Some("ruby".to_string());
    }

    // Shell/Bash
    if text.starts_with("#!/bin/bash")
        || text.starts_with("#!/bin/sh")
        || text.contains("echo ")
        || text.contains("export ")
    {
        return Some("bash".to_string());
    }

    None
}

fn is_color(text: &str) -> bool {
    let trimmed = text.trim();

    // Hex color
    if trimmed.starts_with('#') && (trimmed.len() == 7 || trimmed.len() == 9) {
        return trimmed[1..].chars().all(|c| c.is_ascii_hexdigit());
    }

    // RGB/RGBA
    if trimmed.starts_with("rgb(") || trimmed.starts_with("rgba(") {
        return true;
    }

    // HSL/HSLA
    if trimmed.starts_with("hsl(") || trimmed.starts_with("hsla(") {
        return true;
    }

    // OKLCH
    if trimmed.starts_with("oklch(") {
        return true;
    }

    false
}

fn is_code(text: &str) -> bool {
    // Simple heuristics for code detection
    let code_indicators = [
        "function ",
        "const ",
        "let ",
        "var ",
        "class ",
        "import ",
        "export ",
        "def ",
        "print(",
        "console.log",
        "public ",
        "private ",
        "protected ",
        "<?php",
        "#!/",
        "<script",
        "</",
        "/>",
        "=>",
        "fn ",
        "impl ",
        "struct ",
        "{",
        "}",
        ";",
        "package ",
        "interface ",
    ];

    let indicator_count = code_indicators
        .iter()
        .filter(|indicator| text.contains(*indicator))
        .count();

    // If has 2+ code indicators, likely code
    indicator_count >= 2
}

/// Get source app name (placeholder - requires platform-specific implementation)
pub fn get_source_app() -> Option<String> {
    // TODO: Implement platform-specific source app detection
    // For now, return None
    // On macOS: use NSWorkspace
    // On Windows: use GetForegroundWindow + GetWindowText
    // On Linux: use X11/Wayland APIs
    None
}
