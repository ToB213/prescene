use std::fs;
use std::path::{Path, PathBuf};

use crate::error::AppError;
use crate::model::{MarkdownFrontMatter, Node, Presentation, PresentationConfig, Slide};

// Read each user-provided stylesheet and combine them in CLI order.
pub fn load_css(paths: &[PathBuf]) -> Result<String, AppError> {
    let mut css = String::new();

    for path in paths {
        let content = fs::read_to_string(path).map_err(|source| AppError::ReadFile {
            path: path.clone(),
            source,
        })?;

        css.push_str(&content);
        css.push('\n');
    }

    Ok(css)
}

// Read the YAML input file and deserialize it into the presentation model.
fn load_yaml(path: &Path) -> Result<Presentation, AppError> {
    let source = read_text(path)?;
    let presentation: Presentation = serde_yaml::from_str(&source)?;

    Ok(presentation)
}

pub fn load_input(path: &Path) -> Result<(Presentation, Vec<PathBuf>), AppError> {
    // Keep the existing YAML input path while adding Markdown as a second
    // document format selected by the file extension.
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("md") => load_markdown(path),
        Some("yaml") | Some("yml") => Ok((load_yaml(path)?, Vec::new())),
        _ => Ok((load_yaml(path)?, Vec::new())),
    }
}

// Read Markdown front matter, convert each slide into a text node, and resolve
// CSS paths relative to the Markdown file.
fn load_markdown(path: &Path) -> Result<(Presentation, Vec<PathBuf>), AppError> {
    let source = read_text(path)?;
    // Normalize line endings so front matter parsing behaves consistently on
    // files created on different operating systems.
    let normalized = source.replace("\r\n", "\n");

    let (front_matter_source, body) = split_front_matter(&normalized)?;

    let front_matter: MarkdownFrontMatter = serde_yaml::from_str(&front_matter_source)?;

    // CSS paths in front matter are relative to the input Markdown file, not
    // to the process's current working directory.
    let base_dir = path.parent().unwrap_or_else(|| Path::new("."));

    let css_paths = front_matter
        .css
        .into_iter()
        .map(|css_path| {
            if css_path.is_absolute() {
                css_path
            } else {
                base_dir.join(css_path)
            }
        })
        .collect();

    // Each Markdown section becomes one full-slide text node. The existing
    // renderer can therefore be reused without introducing a new node type.
    let slide_sources = split_slides(body);

    let slides = slide_sources
        .into_iter()
        .enumerate()
        .map(|(index, content)| {
            let slide_number = index + 1;

            Slide {
                id: format!("slide-{}", slide_number),
                nodes: vec![Node::Text {
                    id: format!("slide-{}-content", slide_number),
                    x: 80.0,
                    y: 60.0,
                    width: front_matter.width as f32 - 160.0,
                    height: front_matter.height as f32 - 120.0,
                    content,
                }],
            }
        })
        .collect();

    let presentation = Presentation {
        presentation: PresentationConfig {
            width: front_matter.width,
            height: front_matter.height,
        },
        slides,
    };

    Ok((presentation, css_paths))
}

fn read_text(path: &Path) -> Result<String, AppError> {
    // Centralize file-read errors so every input file reports its path.
    fs::read_to_string(path).map_err(|source| AppError::ReadFile {
        path: path.to_path_buf(),
        source,
    })
}

// Separate the YAML front matter from the Markdown body. Both delimiters must
// occupy their own line and use three hyphens.
fn split_front_matter(source: &str) -> Result<(&str, &str), AppError> {
    let Some(source) = source.strip_prefix("---\n") else {
        return Err(AppError::InvalidMarkdown(
            "front matter must start with ---".to_string(),
        ));
    };

    let Some(end) = source.find("\n---\n") else {
        return Err(AppError::InvalidMarkdown(
            "front matter closing --- was not found".to_string(),
        ));
    };

    let front_matter = &source[..end];
    let body = &source[end + 5..];

    Ok((front_matter, body))
}

// Treat a line containing only `---` as a slide separator. Empty sections are
// ignored so accidental extra separators do not create blank slides.
fn split_slides(source: &str) -> Vec<String> {
    let mut slides = Vec::new();
    let mut current = String::new();

    for line in source.lines() {
        if line.trim() == "---" {
            if !current.trim().is_empty() {
                slides.push(current.trim().to_string());
            }

            current.clear();
        } else {
            current.push_str(line);
            current.push('\n');
        }
    }

    if !current.trim().is_empty() {
        slides.push(current.trim().to_string());
    }

    slides
}
