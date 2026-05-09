const LARGE_DOCUMENT_SECTIONS: usize = 600;
const LARGE_DOCUMENT_TABLE_INTERVAL: usize = 25;
const CLEAN_DOCUMENT_SECTIONS: usize = 400;
const LINK_DOCUMENT_ENTRIES: usize = 500;
const INLINE_CODE_DOCUMENT_LINES: usize = 600;
const REFERENCE_DOCUMENT_ENTRIES: usize = 500;
const TABLE_DOCUMENT_ROWS: usize = 300;
const PARSER_DOCUMENT_SECTIONS: usize = 300;
const SMALL_DOCUMENT_COUNT: usize = 250;

pub(super) struct BenchDocuments {
    pub(super) large: String,
    pub(super) clean_large: String,
    pub(super) link_heavy: String,
    pub(super) inline_code_heavy: String,
    pub(super) reference_heavy: String,
    pub(super) table_heavy: String,
    pub(super) parser_heavy: String,
    pub(super) many_small: Vec<String>,
}

impl BenchDocuments {
    pub(super) fn generate() -> Self {
        Self {
            large: generate_large_document(),
            clean_large: generate_clean_large_document(),
            link_heavy: generate_link_heavy_document(),
            inline_code_heavy: generate_inline_code_heavy_document(),
            reference_heavy: generate_reference_heavy_document(),
            table_heavy: generate_table_heavy_document(),
            parser_heavy: generate_parser_heavy_document(),
            many_small: generate_many_small_documents(),
        }
    }
}

fn generate_large_document() -> String {
    let mut content = String::new();
    for index in 0..LARGE_DOCUMENT_SECTIONS {
        content.push_str("#Heading\n");
        content.push_str("Paragraph with bare URL https://example.com and trailing spaces.  \n");
        content.push_str(">  blockquote with too many spaces\n");
        content.push_str("+ list item\n");
        content.push_str("```rust\nfn main() {}\n```\n\n");
        if index % LARGE_DOCUMENT_TABLE_INTERVAL == 0 {
            content.push_str("| a | b |\n|---|---|\n| 1 | 2 |\n\n");
        }
    }
    content
}

fn generate_clean_large_document() -> String {
    let mut content = String::from("# Title\n\n");
    for index in 0..CLEAN_DOCUMENT_SECTIONS {
        content.push_str(&format!("## Section {index}\n\n"));
        content.push_str("Paragraph text stays short and plain.\n\n");
        content.push_str("- first item\n");
        content.push_str("- second item\n\n");
        content.push_str("```rust\nfn main() {}\n```\n\n");
    }
    content
}

fn generate_link_heavy_document() -> String {
    let mut content = String::from("# Links\n\n");
    for index in 0..LINK_DOCUMENT_ENTRIES {
        content.push_str(&format!(
            "See [nested [{index}]](https://example.com/{index}?q=1 \"title\") and <https://example.org/{index}>.\n"
        ));
        content.push_str(&format!(
            "Image ![alt {index}][image-{index}] and `[ignored](https://example.invalid/{index})`.\n\n"
        ));
        content.push_str(&format!(
            "[image-{index}]: <https://example.org/image-{index}.png> \"Image\"\n"
        ));
    }
    content
}

fn generate_inline_code_heavy_document() -> String {
    let mut content = String::from("# Inline Code\n\n");
    for index in 0..INLINE_CODE_DOCUMENT_LINES {
        content.push_str(&format!(
            "`https://example.com/{index}` and ``[link {index}](https://example.org/{index})`` stay literal.\n"
        ));
    }
    content
}

fn generate_reference_heavy_document() -> String {
    let mut content = String::from("# References\n\n");
    for index in 0..REFERENCE_DOCUMENT_ENTRIES {
        content.push_str(&format!(
            "[Reference {index}][ref-{index}] and ![Image {index}][image-{index}]\n"
        ));
    }
    content.push('\n');
    for index in 0..REFERENCE_DOCUMENT_ENTRIES {
        content.push_str(&format!("[ref-{index}]: https://example.com/{index}\n"));
        content.push_str(&format!(
            "[image-{index}]: <https://example.org/image-{index}.png>\n"
        ));
    }
    content
}

fn generate_table_heavy_document() -> String {
    let mut content = String::from("# Tables\n\n");
    for index in 0..TABLE_DOCUMENT_ROWS {
        content.push_str(&format!("| Key {index} | Value {index} |\n"));
        content.push_str("|---|---|\n");
        content.push_str(&format!(
            "| link | [text {index}](https://example.com/{index}) |\n"
        ));
        content.push_str(&format!("| code | `https://example.invalid/{index}` |\n\n"));
    }
    content
}

fn generate_parser_heavy_document() -> String {
    let mut content = String::from("# Parser Heavy\r\n\r\n");
    for index in 0..PARSER_DOCUMENT_SECTIONS {
        content.push_str(&format!(
            "##Section {index}\r\n[ link {index} ](https://example.com/{index}) and ` code {index} `.\r\n"
        ));
        content.push_str(&format!(
            "``[literal {index}](https://example.invalid/{index})`` and <span>{index}</span>.\r\n"
        ));
        content.push_str(&format!(
            "* spaced {index} * and __strong {index}__.\r\n\r\n"
        ));
    }
    content
}

fn generate_many_small_documents() -> Vec<String> {
    (0..SMALL_DOCUMENT_COUNT)
        .map(|index| {
            format!("# Doc {index}\n\nParagraph with https://example.com/{index}\n\n+ item\n\n")
        })
        .collect()
}
