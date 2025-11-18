use image::{DynamicImage, ImageBuffer, Rgba};
use std::path::Path;
use widestring::U16CString;
use windows::core::PCWSTR;
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetObjectW, SelectObject, BITMAP,
    BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, HBITMAP, HDC,
};
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};
use windows::Win32::UI::Shell::{IShellItemImageFactory, SHCreateItemFromParsingName, SIIGBF};

pub fn generate_thumbnail(path: &Path, target_path: &Path) -> Result<bool, String> {
    if !path.exists() {
        return Ok(false);
    }

    if target_path.parent().is_none() {
        return Err("Invalid thumbnail target path".to_string());
    }

    if target_path.exists() {
        return Ok(true);
    }

    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)
            .map_err(|e| format!("Failed to init COM: {e}"))?;

        // Ensure COM is uninitialized even if we early-return
        let result = generate_with_shell(path, target_path);

        CoUninitialize();
        result
    }
}

unsafe fn generate_with_shell(path: &Path, target_path: &Path) -> Result<bool, String> {
    let wide = U16CString::from_os_str(path).map_err(|_| "Invalid UTF-16 path".to_string())?;
    let shell_item =
        SHCreateItemFromParsingName::<IShellItemImageFactory>(PCWSTR(wide.as_ptr()), None)
            .map_err(|e| format!("Failed to bind shell item: {e}"))?;

    let size = windows::Win32::UI::Shell::SIZE { cx: 256, cy: 256 };
    let hbitmap = shell_item
        .GetImage(size, SIIGBF::empty())
        .map_err(|e| format!("Failed to get thumbnail: {e}"))?;

    let bitmap = hbitmap_to_image(hbitmap)?;

    bitmap
        .save(target_path)
        .map_err(|e| format!("Failed to save PNG: {e}"))?;

    Ok(true)
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
        if hdc.0 == 0 {
            DeleteObject(hbitmap);
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
        DeleteObject(hbitmap);
        DeleteDC(hdc);

        if result == 0 {
            return Err("GetDIBits failed".into());
        }

        let image = ImageBuffer::<Rgba<u8>, _>::from_raw(width as u32, height as u32, pixels)
            .ok_or_else(|| "Failed to build image buffer".to_string())?;

        Ok(DynamicImage::ImageRgba8(image))
    }
}
