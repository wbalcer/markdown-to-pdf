# Markdown to PDF Generator

Signature: Wojciech Balcer

The Markdown to PDF Generator is a Rust-based tool designed to convert Markdown files into fully-formatted PDF documents. With support for titles, signatures, footers, code blocks, tables of contents, and more.

# Features

## Header and Footer 
         Automatically generates a header with the title and a footer with page numbers and customizable text.
## Table of Contents 
         Dynamically generates a Table of Contents based on the headers in your Markdown file.
## Code Block Styling
         Preserves and styles code blocks for a professional look in the PDF.
## Multi-page Support 
         Automatically paginates content and manages space efficiently.
## Custom Signature and Footer
         Allows embedding signatures and custom footer text.

# Installation

1. Install Rust and Cargo if not already installed:
   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Clone this repository:
   ```
   git clone https://github.com/wbalcer/markdown-to-pdf.git
   cd markdown-to-pdf
   ```
3. Build this project:
   ```
   cargo build --release
   ```
4. Run the binary:
   ```
   ./target/release/markdown-to-pdf
   ```

# Usage

## Signature
      To sign the file just change the line with "Signature:" for your own signature.

## Footer
      The last line of the markdown file will act as the footer

# Test
   To test:
   ```
   cargo run <input_file> <output_file>
   ```

PDF generated by wbalcer markdown-to-pdf
