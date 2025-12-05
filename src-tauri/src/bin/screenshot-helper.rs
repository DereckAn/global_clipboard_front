use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = pico_args::Arguments::from_env();
    let mode: String = args.value_from_str("--mode")?;
    let out: String = args.value_from_str("--out")?;
    let out_path = PathBuf::from(out);

    match mode.as_str() {
        "full" => run_full(&out_path)?,
        "region" => run_region(&out_path)?,
        _ => return Err("unknown mode".into()),
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn run_full(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("screencapture").arg("-x").arg(path).status()?;
    if !status.success() {
        return Err("screencapture failed".into());
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn run_region(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("screencapture")
        .arg("-i")
        .arg("-x")
        .arg(path)
        .status()?;
    if !status.success() {
        return Err("screencapture failed".into());
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn run_full(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Prefer grim; fallback to gnome-screenshot
    if Command::new("grim").arg(path).status().map(|s| s.success()).unwrap_or(false) {
        return Ok(());
    }
    let status = Command::new("gnome-screenshot")
        .arg("-f")
        .arg(path)
        .status()?;
    if !status.success() {
        return Err("No supported screenshot tool found (install grim or gnome-screenshot)".into());
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn run_region(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(selection) = Command::new("slurp").output() {
        if selection.status.success() {
            let geometry = String::from_utf8_lossy(&selection.stdout).trim().to_string();
            if !geometry.is_empty() {
                if Command::new("grim")
                    .arg("-g")
                    .arg(geometry)
                    .arg(path)
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false)
                {
                    return Ok(());
                }
            }
        }
    }

    let status = Command::new("gnome-screenshot")
        .arg("-a")
        .arg("-f")
        .arg(path)
        .status()?;
    if !status.success() {
        return Err(
            "No supported region capture tool found (install grim+slurp or gnome-screenshot)".into(),
        );
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn run_full(_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    Err("Full-screen capture not implemented on Windows helper".into())
}

#[cfg(target_os = "windows")]
fn run_region(_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    Err("Region capture not implemented on Windows helper".into())
}
