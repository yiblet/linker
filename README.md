# Mdlinker

mdlinker is a tool designed for enhancing Markdown documentation by automatically creating hyperlinks between documents based on keywords and slugs defined in their front matter. It parses a directory of Markdown files, identifies keywords within their front matter, and then generates a new set of Markdown files with added links for cross-references, facilitating easier navigation and discovery of related topics within a documentation set.

## Features

- **Keyword and Slug Parsing**: Extracts `keywords` and `slug` from the YAML front matter of each Markdown file.
- **Automatic Link Generation**: Creates hyperlinks in the documents that reference each other based on the extracted keywords, making the documentation more navigable.
- **Support for N-gram Keywords**: Handles multi-word keywords efficiently, allowing for more natural cross-referencing.
- **Customizable Output**: Generates a new set of Markdown files with added links, leaving the original files unmodified.

## Getting Started

To use mdlinker, follow these steps:

### Prerequisites

Ensure you have Rust installed on your system. You can download Rust and find installation instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Installation

1. Clone this repository to your local machine.
2. Navigate to the cloned directory.
3. Build the project using Cargo, Rust's package manager and build system:
   ```sh
   cargo build --release
   ```

### Usage

After building the project, you can run mdlinker as follows:

```sh
cargo run -- --glob "<path-to-markdown-files>/*.md" --output <path-to-output-directory>
```

Replace `<path-to-markdown-files>` with the path to your directory containing the Markdown files you wish to process. Replace `<path-to-output-directory>` with the path where you want the processed files to be saved.

*Note*: The glob pattern must be enclosed in quotes to prevent shell expansion.

## How It Works

mdlinker works in several steps:

1. **Indexing**: Parses each Markdown file's front matter to extract keywords and slugs, building an index of keywords to document slugs.
2. **Updating**: For each document, it looks up other documents' slugs associated with its keywords and inserts hyperlinks to these documents at suitable locations within the text.
3. **Output Generation**: Saves the updated Markdown files with added links to the specified output directory.

## Contributing

Contributions to mdlinker are welcome! Whether it's submitting a bug report, proposing a feature, or submitting a pull request, all contributions are appreciated.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
