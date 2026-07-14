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

// The loaded document contains the presentation model and any custom CSS paths specified in the input file.
#[derive(Debug, Deserialize)]
pub struct LoadedDocument {
    pub presentation: Presentation, // The loaded presentation model
    pub css_paths: Vec<PathBuf>,    // List of custom CSS file paths
}

#[derive(Debug, Deserialize)]
pub struct NodeBase {
    pub id: String,           // Unique identifier for the node
    pub transform: Transform, // Transformation properties for the node

    #[serde(default)]
    pub classes: Vec<String>, // Optional CSS classes for the node
}

#[derive(Debug, Deserialize)]
pub struct Transform {
    pub x: f32,      // X position of the node
    pub y: f32,      // Y position of the node
    pub width: f32,  // Width of the node
    pub height: f32, // Height of the node
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
// Serde reads the YAML `type` field and uses it to select the appropriate
// variant. Each variant then stores its type-specific properties.
pub enum Node {
    Markdown {
        #[serde(flatten)]
        base: NodeBase, // Common properties for the node
        content: String, // Content of the text node
    },

    Rect {
        #[serde(flatten)]
        base: NodeBase, // Common properties for the node
    },

    Image {
        #[serde(flatten)]
        base: NodeBase, // Common properties for the node
        src: PathBuf,        // Source path of the image file
        alt: Option<String>, // Optional alt text for the image
    },
}
