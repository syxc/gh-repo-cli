//! Terminal output helpers: headers, separators, directory trees and tables,
//! byte-compatible with the Go build's formatting (rune-based cell padding).

use crate::color;
use crate::fsutil::{FileItem, Kind};

pub fn print_header(text: &str) {
    println!("{}", color::bold(text));
}

pub fn print_error(message: &str) {
    println!("{}", color::red(&format!("❌ Error: {message}")));
}

pub fn print_separator(width: usize) {
    println!("{}", color::gray(&"─".repeat(width)));
}

pub fn display_tree(items: &[FileItem], prefix: &str) {
    for line in tree_lines(items, prefix) {
        println!("{line}");
    }
}

fn tree_lines(items: &[FileItem], prefix: &str) -> Vec<String> {
    let mut lines = Vec::new();
    for (index, item) in items.iter().enumerate() {
        let is_last = index == items.len() - 1;
        let connector = if is_last { "└─ " } else { "├─ " };
        let icon = match item.kind {
            Kind::Directory => "📁",
            Kind::File => "📄",
        };

        lines.push(format!(
            "{prefix}{connector}{icon} {}",
            color::escape_controls(&item.name)
        ));

        let Some(children) = item
            .children
            .as_ref()
            .filter(|children| !children.is_empty())
        else {
            continue;
        };
        let continuation = if is_last { "   " } else { "│  " };
        lines.extend(tree_lines(children, &format!("{prefix}{continuation}")));
    }

    lines
}

/// Renders a bordered table; header cells are cyan, cells are padded or
/// truncated to `widths` counted in characters.
pub fn print_table(headers: &[&str], rows: &[Vec<String>], widths: &[usize]) {
    for line in render_table(headers, rows, widths) {
        println!("{line}");
    }
}

fn render_table(headers: &[&str], rows: &[Vec<String>], widths: &[usize]) -> Vec<String> {
    let columns = headers.len();
    if columns == 0 {
        return Vec::new();
    }

    let horizontal = |left: &str, middle: &str, right: &str| {
        let segments: Vec<String> = widths[..columns]
            .iter()
            .map(|width| "─".repeat(width + 2))
            .collect();
        format!("{left}{}{right}", segments.join(middle))
    };
    let row_line = |cells: &[String]| {
        let segments: Vec<String> = (0..columns)
            .map(|index| {
                let cell = cells.get(index).map_or("", String::as_str);
                format!(" {} ", pad_cell(cell, widths[index]))
            })
            .collect();
        format!("│{}│", segments.join("│"))
    };

    let mut lines = vec![horizontal("┌", "┬", "┐")];

    let header_cells: Vec<String> = headers
        .iter()
        .enumerate()
        .map(|(index, header)| format!(" {} ", color::cyan(&pad_cell(header, widths[index]))))
        .collect();
    lines.push(format!("│{}│", header_cells.join("│")));
    lines.push(horizontal("├", "┼", "┤"));

    for row in rows {
        lines.push(row_line(row));
    }
    lines.push(horizontal("└", "┴", "┘"));

    lines
}

fn pad_cell(cell: &str, width: usize) -> String {
    let cell = color::escape_controls(cell);
    let length = cell.chars().count();
    if length >= width {
        return cell.chars().take(width).collect();
    }

    format!("{cell}{}", " ".repeat(width - length))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plain(line: &str) -> String {
        line.to_owned()
    }

    #[test]
    fn pad_cell_pads_and_truncates_by_characters() {
        assert_eq!(pad_cell("go", 5), "go   ");
        assert_eq!(pad_cell("go", 2), "go");
        assert_eq!(pad_cell("gopher", 2), "go");
        assert_eq!(pad_cell("中文abc", 4), "中文ab");
    }

    #[test]
    fn render_table_draws_borders_with_two_space_padding() {
        let lines = render_table(
            &["Language", "Files"],
            &[
                vec!["Go".to_owned(), "3".to_owned()],
                vec!["Rust".to_owned(), "12".to_owned()],
            ],
            &[8, 5],
        );

        let expected: Vec<String> = vec![
            plain("┌──────────┬───────┐"),
            plain("│ Language │ Files │"),
            plain("├──────────┼───────┤"),
            plain("│ Go       │ 3     │"),
            plain("│ Rust     │ 12    │"),
            plain("└──────────┴───────┘"),
        ];

        assert_eq!(lines, expected);
    }

    #[test]
    fn render_table_truncates_cells_to_width() {
        let lines = render_table(
            &["File"],
            &[vec!["a-very-long-file-name.rs".to_owned()]],
            &[6],
        );

        assert_eq!(lines[3], "│ a-very │");
    }

    #[test]
    fn render_table_without_headers_is_empty() {
        assert_eq!(render_table(&[], &[], &[]), Vec::<String>::new());
    }

    #[test]
    fn regression_tree_escapes_terminal_controls_in_file_names() {
        let tree = vec![FileItem {
            name: "\u{1b}]52;c;U1lOVEhFVElD\u{7}\nfile.txt".to_owned(),
            path: "unused".to_owned(),
            kind: Kind::File,
            children: None,
        }];

        assert_eq!(
            tree_lines(&tree, ""),
            vec!["└─ 📄 \\u{1b}]52;c;U1lOVEhFVElD\\u{7}\\nfile.txt"]
        );
    }

    #[test]
    fn regression_table_escapes_controls_before_padding_or_truncation() {
        assert_eq!(pad_cell("a\nb", 5), "a\\nb ");
        assert_eq!(pad_cell("\u{1b}[2J", 30), "\\u{1b}[2J                     ");
        assert_eq!(pad_cell("中文abc", 4), "中文ab");
    }

    #[test]
    fn display_tree_draws_connectors_and_continuation_lines() {
        let tree = vec![
            FileItem {
                name: "src".to_owned(),
                path: "src".to_owned(),
                kind: Kind::Directory,
                children: Some(vec![FileItem {
                    name: "main.rs".to_owned(),
                    path: "src/main.rs".to_owned(),
                    kind: Kind::File,
                    children: None,
                }]),
            },
            FileItem {
                name: "README.md".to_owned(),
                path: "README.md".to_owned(),
                kind: Kind::File,
                children: None,
            },
            FileItem {
                name: "empty-dir".to_owned(),
                path: "empty-dir".to_owned(),
                kind: Kind::Directory,
                children: Some(Vec::new()),
            },
        ];

        assert_eq!(
            tree_lines(&tree, ""),
            vec![
                "├─ 📁 src",
                "│  └─ 📄 main.rs",
                "├─ 📄 README.md",
                "└─ 📁 empty-dir",
            ]
        );
    }
}
