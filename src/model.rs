use serde::Deserialize;
use std::path::PathBuf;

// Data loaded from the root of a presentation YAML document.
#[derive(Debug, Deserialize)]
pub struct Presentation {
    pub presentation: PresentationConfig, // Presentation configuration
    pub slides: Vec<Slide>,               // List of slides
}

// Dimensions shared by all slides in the presentation.
#[derive(Debug, Deserialize)]
pub struct PresentationConfig {
    pub width: u32,  // Width of the presentation
    pub height: u32, // Height of the presentation
}

// A slide has an identifier and an ordered list of visual nodes.
#[derive(Debug, Deserialize)]
pub struct Slide {
    pub id: String,       // Unique identifier for the slide
    pub nodes: Vec<Node>, // List of nodes in the slide
}

// Front matter is a YAML block at the top of a Markdown file that contains metadata about the document. In this case, it specifies the presentation dimensions and optional custom CSS files.
#[derive(Debug, Deserialize)]
pub struct MarkdownFrontMatter {
    pub width: u32,  // Width of the presentation
    pub height: u32, // Height of the presentation

    #[serde(default)]
    pub css: Vec<PathBuf>, // List of custom CSS file paths
}

pub struct LoadedDocument {
    pub presentation: Presentation, // The loaded presentation model
    pub css_paths: Vec<PathBuf>,    // List of custom CSS file paths
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
// Serde reads the YAML `type` field and uses it to select the appropriate
// variant. Each variant then stores its type-specific properties.
//
// Example YAML:
// type: text
// id: title
// x: 100
// y: 80
// width: 800
// height: 100
// content: Hello
pub enum Node {
    Text {
        id: String,      // Unique identifier for the text node
        x: f32,          // X position of the text node
        y: f32,          // Y position of the text node
        width: f32,      // Width of the text node
        height: f32,     // Height of the text node
        content: String, // Content of the text node
    },

    Rect {
        id: String,  // Unique identifier for the rectangle node
        x: f32,      // X position of the rectangle node
        y: f32,      // Y position of the rectangle node
        width: f32,  // Width of the rectangle node
        height: f32, // Height of the rectangle node
    },

    Image {
        id: String,  // Unique identifier for the image node
        x: f32,      // X position of the image node
        y: f32,      // Y position of the image node
        width: f32,  // Width of the image node
        height: f32, // Height of the image node
        src: String, // Source URL or path of the image
    },
}
