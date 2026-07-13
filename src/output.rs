use std::fs;
use std::path::Path;

use crate::error::AppError;

pub fn write_html(path: &Path, html: &str) -> Result<(), AppError> {
    // Create the output directory before writing the generated HTML file.
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| AppError::WriteFile {
            path: parent.to_path_buf(),
            source,
        })?;
    }

    // Write the final HTML and retain the output path for a useful error.
    fs::write(&path, html).map_err(|source| AppError::WriteFile {
        path: path.to_path_buf(),
        source,
    })?;

    println!("generated {}", path.display());

    Ok(())
}
