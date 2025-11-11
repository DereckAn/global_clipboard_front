use crate::clipboard::{file_handler, image_handler};

pub fn delete_file_url(content_type: &str, file_url: &str, metadata_json: &str) {
    let result = if content_type == "file" {
        file_handler::delete_file_assets(file_url, metadata_json)
    } else {
        image_handler::delete_image_from_disk(file_url)
    };

    if let Err(err) = result {
        eprintln!(
            "Failed to delete {} asset ({}): {}",
            content_type, file_url, err
        );
    }
}
