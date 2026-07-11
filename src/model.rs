use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Presentation {
    pub presentation: PresentationConfig, // Presentation configuration
    pub slides: Vec<Slide>,               // List of slides
}

#[derive(Debug, Deserialize)]
pub struct PresentationConfig {
    pub width: u32,  // Width of the presentation
    pub height: u32, // Height of the presentation
}

#[derive(Debug, Deserialize)]
pub struct Slide {
    pub id: String,       // Unique identifier for the slide
    pub nodes: Vec<Node>, // List of nodes in the slide
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
// this enum represents different types of nodes that can be present in a slide.
// Each variant corresponds to a specific type of node, such as text, rectangle, or image.
// The fields within each variant define the properties of that node type.
//
// example of a slide node in the YAML format:
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
