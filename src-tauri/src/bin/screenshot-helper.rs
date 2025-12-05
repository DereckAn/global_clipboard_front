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
    use std::process::Stdio;

    let path = _path.to_string_lossy();
    let script = format!(
        r#"
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
$bounds = [System.Windows.Forms.Screen]::PrimaryScreen.Bounds
$bitmap = New-Object System.Drawing.Bitmap $bounds.Width, $bounds.Height
$graphics = [System.Drawing.Graphics]::FromImage($bitmap)
$graphics.CopyFromScreen($bounds.Location, [System.Drawing.Point]::Empty, $bounds.Size)
$bitmap.Save('{path}', [System.Drawing.Imaging.ImageFormat]::Png)
"#,
        path = path.replace("\\", "\\\\")
    );

    let status = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-Command")
        .arg(script)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if !status.success() {
        return Err("Windows PowerShell capture failed".into());
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn run_region(_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Stdio;

    let path = _path.to_string_lossy().replace("\\", "\\\\");
    let script = format!(
        r#"
Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing

function Save-ClipImage([string]$p, [int]$timeoutMs) {{
  $sw = [System.Diagnostics.Stopwatch]::StartNew()
  while ($sw.ElapsedMilliseconds -lt $timeoutMs) {{
    Start-Sleep -Milliseconds 200
    if ([Windows.Forms.Clipboard]::ContainsImage()) {{
      $img = [Windows.Forms.Clipboard]::GetImage()
      if ($img) {{
        $img.Save($p, [System.Drawing.Imaging.ImageFormat]::Png)
        return $true
      }}
    }}
  }}
  return $false
}}

$started = $false
try {{
  Start-Process -WindowStyle Hidden "explorer.exe" "ms-screenclip:"
  $started = $true
}} catch {{}}

if (-not $started) {{
  try {{
    Start-Process -WindowStyle Hidden "snippingtool.exe" "/clip"
    $started = $true
  }} catch {{}}
}}

if (-not $started) {{ exit 2 }}

if (-not (Save-ClipImage '{path}' 15000)) {{ exit 3 }}
"#,
        path = path
    );

    let status = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-STA")
        .arg("-NonInteractive")
        .arg("-Command")
        .arg(script)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    if !status.success() {
        return match status.code() {
            Some(2) => Err("No screen clipping tool found (ms-screenclip or snippingtool)".into()),
            Some(3) => Err("Timed out waiting for region capture".into()),
            _ => Err("Windows region capture failed".into()),
        };
    }

    Ok(())
}
