use crate::model::{Node, Presentation};

pub fn render_html(presentation: &Presentation) -> String {
    let mut slides_html = String::new();

    for slide in &presentation.slides {
        // collect the HTML for all nodes in the slide
        let nodes_html = slide
            .nodes
            .iter()
            .map(render_node) // render each node to HTML
            .collect::<Vec<_>>()
            .join("\n");

        // wrap the nodes in a slide section
        slides_html.push_str(&format!(
            r#"
            <section class="slide" id="{}">
            {}
            </section>
            "#,
            escape_html(&slide.id),
            nodes_html
        ));
    }

    // wrap the slides in a complete HTML document
    format!(
        r#"
        <!doctype html>
        <html lang="ja">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>Prescene Presentation</title>

                <style>
                    body {{
                        margin: 0;
                        padding: 32px;
                        background: #dddddd;
                        font-family: sans-serif;
                        }}

                    .slide {{
                        position: relative;
                        width: {}px;
                        height: {}px;
                        margin: 0 auto 32px;
                        overflow: hidden;
                        background: white;
                        box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
                    }}

                    .node {{
                        position: absolute;
                        box-sizing: border-box;
                    }}

                    .text-node {{
                        white-space: pre-wrap;
                    }}

                    .rect-node {{
                        border: 2px solid black;
                    }}

                    .image-node {{
                        object-fit: contain;
                    }}
                </style>
            </head>

            <body>
            {}
            </body>
        </html>
    "#,
        presentation.presentation.width, presentation.presentation.height, slides_html
    )
}

fn render_node(node: &Node) -> String {
    match node {
        Node::Text {
            id,
            x,
            y,
            width,
            height,
            content,
        } => {
            format!(
                r#"<div id="{}" class="node text-node" style="left:{}px; top:{}px; width:{}px; height:{}px;">{}</div>"#,
                escape_html(id),
                x,
                y,
                width,
                height,
                escape_html(content)
            )
        }

        Node::Rect {
            id,
            x,
            y,
            width,
            height,
        } => {
            format!(
                r#"<div id="{}" class="node rect-node" style="left:{}px; top:{}px; width:{}px; height:{}px;"></div>"#,
                escape_html(id),
                x,
                y,
                width,
                height
            )
        }

        Node::Image {
            id,
            x,
            y,
            width,
            height,
            src,
        } => {
            format!(
                r#"<img id="{}" class="node image-node" src="{}" alt="" style="left:{}px; top:{}px; width:{}px; height:{}px;">"#,
                escape_html(id),
                escape_html(src),
                x,
                y,
                width,
                height
            )
        }
    }
}

// Escape HTML special characters in a string to prevent XSS attacks and ensure proper rendering in HTML.
fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
