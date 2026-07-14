use crate::model::{Node, Presentation};
use pulldown_cmark::{Options, Parser, html};

const DEFAULT_CSS: &str = include_str!("default.css");

// Convert the presentation model into a self-contained HTML document.
pub fn render_html(presentation: &Presentation, custom_css: &str) -> String {
    let mut slides_html = String::new();

    for slide in &presentation.slides {
        // Render nodes in their source order and combine them into one slide.
        let nodes_html = slide
            .nodes
            .iter()
            .map(render_node)
            .collect::<Vec<_>>()
            .join("\n");

        // Wrap the nodes in a section that provides their positioning context.
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

    // Use the presentation dimensions as the coordinate system for each slide.
    format!(
        r#"
        <!doctype html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>Prescene Presentation</title>

                <style>
                {default_css}

                .slide {{
                    width: {width}px;
                    height: {height}px;
                }}

                {custom_css}

                </style>
            </head>

            <body>
            {slides_html}
            </body>
        </html>
    "#,
        default_css = DEFAULT_CSS,
        width = presentation.presentation.width,
        height = presentation.presentation.height,
        custom_css = custom_css,
        slides_html = slides_html,
    )
}

fn render_markdown(source: &str) -> String {
    let mut options = Options::empty();

    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_FOOTNOTES);

    let parser = Parser::new_ext(source, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

fn render_node(node: &Node) -> String {
    // Match every node variant and emit the corresponding HTML element.
    match node {
        Node::Markdown { base, content } => {
            let classes = render_classes(&["node", "text-node"], &base.classes);

            format!(
                r#"<div id="{}" class="{}" style="left:{}px; top:{}px; width:{}px; height:{}px;">{}</div>"#,
                escape_html(&base.id),
                classes,
                base.transform.x,
                base.transform.y,
                base.transform.width,
                base.transform.height,
                render_markdown(content)
            )
        }

        Node::Rect { base } => {
            let classes = render_classes(&["node", "rect-node"], &base.classes);

            format!(
                r#"<div id="{}" class="{}" style="left:{}px; top:{}px; width:{}px; height:{}px;"></div>"#,
                escape_html(&base.id),
                classes,
                base.transform.x,
                base.transform.y,
                base.transform.width,
                base.transform.height
            )
        }

        Node::Image { base, src, alt } => {
            let classes = render_classes(&["node", "image-node"], &base.classes);
            let src = src.to_string_lossy();
            let alt = alt.as_deref().unwrap_or("");

            format!(
                r#"<img id="{}" class="{}" src="{}" alt="{}" style="left:{}px; top:{}px; width:{}px; height:{}px;">"#,
                escape_html(&base.id),
                classes,
                escape_html(src.as_ref()),
                escape_html(alt),
                base.transform.x,
                base.transform.y,
                base.transform.width,
                base.transform.height
            )
        }
    }
}

// Combine renderer-provided classes with user-defined classes from the input.
fn render_classes(base: &[&str], custom: &[String]) -> String {
    base.iter()
        .copied()
        .chain(custom.iter().map(String::as_str))
        .map(escape_html)
        .collect::<Vec<_>>()
        .join(" ")
}

// Escape text and attribute values before inserting them into generated HTML.
fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
