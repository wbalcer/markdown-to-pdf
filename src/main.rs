use printpdf::*;
use std::{fs, io::BufWriter};

fn main() {
    let input_path = "readme.md";
    let output_path = "output.pdf";

    let markdown_content = match fs::read_to_string(input_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading Markdown file: {}", err);
            return;
        }
    };

    match generate_pdf(&markdown_content, output_path) {
        Ok(_) => println!("PDF successfully generated at {}", output_path),
        Err(err) => eprintln!("Failed to generate PDF: {}", err),
    }
}

fn generate_pdf(content: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (doc, cover_page, cover_layer) = PdfDocument::new("Markdown to PDF", Mm(210.0), Mm(297.0), "Cover Page");
    let font_regular = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;
    let font_monospace = doc.add_builtin_font(BuiltinFont::Courier)?;

    let title = extract_title(content).unwrap_or("Untitled".to_string());
    let signature = extract_signature(content).unwrap_or("___________________".to_string());
    let footer_text = extract_footer(content).unwrap_or("Generated with Markdown to PDF by wbalcer".to_string());

    let cover = doc.get_page(cover_page).get_layer(cover_layer);
    cover.use_text(&title, 32.0, Mm(20.0), Mm(250.0), &font_bold);
    cover.use_text(&signature, 16.0, Mm(20.0), Mm(100.0), &font_regular);

    let (toc_page, toc_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Table of Contents");
    let toc = doc.get_page(toc_page).get_layer(toc_layer);
    toc.use_text("Table of Contents", 28.0, Mm(15.0), Mm(270.0), &font_bold);

    let mut toc_entries = Vec::new();
    let mut page_number = 3;
    let mut content_pages = Vec::new();
    let mut y_position = 270.0;
    let mut in_code_block = false;

    let (content_page, content_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Content Page");
    content_pages.push((content_page, content_layer));

    for line in content.lines() {
        if line.starts_with("Signature:") || line == footer_text {
            continue;
        }

        let (current_page, current_layer) = content_pages.last().unwrap();
        let layer = doc.get_page(*current_page).get_layer(*current_layer);

        if y_position < 20.0 {
            add_footer(&doc, *current_page, page_number, &font_regular, &footer_text);
            page_number += 1;

            let (new_page, new_layer) = doc.add_page(Mm(210.0), Mm(297.0), "Content Page");
            content_pages.push((new_page, new_layer));
            y_position = 270.0;
        }

        if line.trim() == "```" {
            in_code_block = !in_code_block;
            continue;
        }

        if in_code_block {
            layer.use_text(line, 10.0, Mm(15.0), Mm(y_position), &font_monospace);
            y_position -= 10.0;
        } else if line.starts_with("# ") {
            let chapter = line.trim_start_matches("# ").to_string();
            toc_entries.push((chapter.clone(), page_number, true));
            layer.use_text(&chapter, 24.0, Mm(15.0), Mm(y_position), &font_bold);
            y_position -= 24.0;
        } else if line.starts_with("## ") {
            let subchapter = line.trim_start_matches("## ").to_string();
            toc_entries.push((subchapter.clone(), page_number, false));
            layer.use_text(&subchapter, 18.0, Mm(25.0), Mm(y_position), &font_bold);
            y_position -= 18.0;
        } else {
            for wrapped_line in wrap_text(line, 80) {
                layer.use_text(wrapped_line, 12.0, Mm(15.0), Mm(y_position), &font_regular);
                y_position -= 12.0;
            }
        }
    }

    add_footer(&doc, content_pages.last().unwrap().0, page_number, &font_regular, &footer_text);

    let mut toc_y_position = 250.0;
    for (entry, page, is_chapter) in toc_entries {
        let indent = if is_chapter { 20.0 } else { 30.0 };
        toc.use_text(
            format!("{} .......................... {}", entry, page),
            14.0,
            Mm(indent),
            Mm(toc_y_position),
            &font_regular,
        );
        toc_y_position -= 14.0;
    }

    let mut file = BufWriter::new(fs::File::create(output_path)?);
    doc.save(&mut file)?;
    Ok(())
}

fn wrap_text(line: &str, max_width: usize) -> Vec<String> {
    let mut wrapped_lines = Vec::new();
    let mut current_line = String::new();

    for word in line.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_width {
            wrapped_lines.push(current_line);
            current_line = String::new();
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    if !current_line.is_empty() {
        wrapped_lines.push(current_line);
    }

    wrapped_lines
}

fn add_footer(doc: &PdfDocumentReference, page: PdfPageIndex, page_num: i32, font: &IndirectFontRef, footer_text: &str) {
    let footer_layer = doc.get_page(page).add_layer("Footer Layer");
    footer_layer.use_text(
        format!("Page {}  |  {}", page_num, footer_text),
        10.0,
        Mm(10.0),
        Mm(10.0),
        font,
    );
}

fn extract_title(content: &str) -> Option<String> {
    content.lines().find_map(|line| {
        if line.starts_with("# ") {
            Some(line.trim_start_matches("# ").to_string())
        } else {
            None
        }
    })
}

fn extract_signature(content: &str) -> Option<String> {
    content.lines().find_map(|line| {
        if line.starts_with("Signature:") {
            Some(line.replace("Signature:", "").trim().to_string())
        } else {
            None
        }
    })
}

fn extract_footer(content: &str) -> Option<String> {
    content.lines().last().map(|line| line.trim().to_string())
}
