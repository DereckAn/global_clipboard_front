use image::imageops::FilterType;
use image::{DynamicImage, ImageBuffer, Rgba};
use std::fs;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetObjectW, SelectObject, BITMAP,
    BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, HBITMAP, HDC,
};

pub fn generate_thumbnail(path: &Path, target_path: &Path) -> Result<bool, String> {
    if !path.exists() {
        return Ok(false);
    }

    let Some(parent) = target_path.parent() else {
        return Err("Invalid thumbnail target path".to_string());
    };

    fs::create_dir_all(parent).map_err(|e| format!("Failed to create thumbnail dir: {e}"))?;

    // Fast path: images via the bundled image crate.
    if let Ok(img) = image::open(path) {
        let thumb = img.resize(256, 256, FilterType::Lanczos3);
        thumb
            .save(target_path)
            .map_err(|e| format!("Failed to save thumbnail: {e}"))?;
        return Ok(true);
    }

    // Everything else (video, pdf, docx, …): ask the Windows Shell for the same
    // thumbnail Explorer shows.
    match shell_thumbnail(path, 256) {
        Ok(Some(img)) => {
            img.save(target_path)
                .map_err(|e| format!("Failed to save shell thumbnail: {e}"))?;
            Ok(true)
        }
        Ok(None) => Ok(false),
        Err(e) => {
            eprintln!("Shell thumbnail failed for {}: {e}", path.display());
            Ok(false)
        }
    }
}

/// Ask the Windows Shell for a file's thumbnail via `IShellItemImageFactory`
/// (the same provider Explorer uses — covers video, PDF, Office, etc.).
fn shell_thumbnail(path: &Path, size: i32) -> Result<Option<DynamicImage>, String> {
    use windows::core::PCWSTR;
    use windows::Win32::Foundation::SIZE;
    use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
    use windows::Win32::UI::Shell::{
        IShellItemImageFactory, SHCreateItemFromParsingName, SIIGBF_RESIZETOFIT,
    };

    // Null-terminated wide string for the path.
    let wide: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        // COM must be initialized per-thread; ignore "already initialized".
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        let factory: IShellItemImageFactory =
            SHCreateItemFromParsingName(PCWSTR(wide.as_ptr()), None)
                .map_err(|e| format!("SHCreateItemFromParsingName failed: {e}"))?;

        let hbitmap = factory
            .GetImage(SIZE { cx: size, cy: size }, SIIGBF_RESIZETOFIT)
            .map_err(|e| format!("GetImage failed: {e}"))?;

        let img = hbitmap_to_image(hbitmap)?;
        Ok(Some(img))
    }
}

fn hbitmap_to_image(hbitmap: HBITMAP) -> Result<DynamicImage, String> {
    unsafe {
        let mut bitmap = BITMAP::default();
        GetObjectW(
            hbitmap,
            std::mem::size_of::<BITMAP>() as i32,
            Some(&mut bitmap as *mut _ as _),
        );

        let width = bitmap.bmWidth;
        let height = bitmap.bmHeight;

        let mut info_header = BITMAPINFOHEADER::default();
        info_header.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        info_header.biWidth = width;
        info_header.biHeight = -height; // top-down
        info_header.biPlanes = 1;
        info_header.biBitCount = 32;
        info_header.biCompression = 0;

        let mut info = BITMAPINFO {
            bmiHeader: info_header,
            ..Default::default()
        };

        let mut pixels = vec![0u8; (width * height * 4) as usize];

        let hdc: HDC = CreateCompatibleDC(None);
        if hdc.0.is_null() {
            let _ = DeleteObject(hbitmap);
            return Err("Failed to create DC".into());
        }

        let old = SelectObject(hdc, hbitmap);
        let result = GetDIBits(
            hdc,
            hbitmap,
            0,
            height as u32,
            Some(pixels.as_mut_ptr() as _),
            &mut info,
            DIB_RGB_COLORS,
        );

        SelectObject(hdc, old);
        let _ = DeleteObject(hbitmap);
        let _ = DeleteDC(hdc);

        if result == 0 {
            return Err("GetDIBits failed".into());
        }

        // Windows gives BGRA; swap to RGBA for the image crate.
        for px in pixels.chunks_exact_mut(4) {
            px.swap(0, 2);
        }

        let image = ImageBuffer::<Rgba<u8>, _>::from_raw(width as u32, height as u32, pixels)
            .ok_or_else(|| "Failed to build image buffer".to_string())?;

        Ok(DynamicImage::ImageRgba8(image))
    }
}
