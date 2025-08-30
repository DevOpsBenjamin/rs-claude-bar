use crate::{
    table::HeaderInfo,   
    common::colors::*,
};

/// The TableCreator handles rendering table data
pub struct TableCreator {
    headers: Vec<HeaderInfo>,
    rows: Vec<Vec<String>>,
    has_warnings: bool,
}

impl TableCreator {
    pub fn new(mut headers: Vec<HeaderInfo>) -> Self {
        let has_warnings = Self::format_headers(&mut headers);
        Self { headers, rows: Vec::new(), has_warnings }
    }

    /// Adds a row, formatting and padding immediately. Tracks any formatting problems.
    pub fn add_row(&mut self, row: Vec<String>) {
        let mut bad_format = false;
        let mut processed: Vec<String> = Vec::new();

        for (i, header) in self.headers.iter().enumerate() {
            let cell_content = row.get(i);

            let formatted = match cell_content {
                Some(value) => {
                    let content_len = value.chars().count();
                    if content_len > header.width {
                        bad_format = true;
                        let truncated = Self::truncate_with_dot(value, header.width);
                        format!("{:>width$}", truncated, width = header.width)
                    } else {
                        format!("{:>width$}", value, width = header.width)
                    }
                }
                None => {
                    // Missing column, insert `_` padded
                    bad_format = true;
                    "_".repeat(header.width)
                }
            };

            processed.push(formatted);
        }

        if row.len() > self.headers.len() {
            bad_format = true; // too many values
        }

        if bad_format {
            self.has_warnings = true;
        }
        self.rows.push(processed);
    }
    
    pub fn display(&self, ignore_warning: bool) {
        self.create_header();
        for row in &self.rows {
            print!("│");
            for cell in row {
                print!(" {} │", cell);
            }
            println!();
        }
        self.create_bottom();

        if self.has_warnings && !ignore_warning {
            println!("⚠️  Warning: Some rows were auto-corrected (truncated or padded).");
        }
    }

    /// Format headers in place, return true if any were truncated
    fn format_headers(headers: &mut Vec<HeaderInfo>) -> bool {
        let mut has_warnings = false;
        for header in headers.iter_mut() {
            let label_len = header.label.chars().count();
            if label_len > header.width {
                has_warnings = true;
                header.label = Self::truncate_with_dot(&header.label, header.width);
            } else {
                header.label = format!("{:<width$}", &header.label, width = header.width)
            }
        }
        has_warnings
    }

    /// Truncate text to width with dot prefix (e.g., "very long text" -> ".ext")
    fn truncate_with_dot(text: &str, width: usize) -> String {
        let suffix: String = text
            .chars()
            .rev()
            .take(width - 1)
            .collect::<String>()
            .chars()
            .rev()
            .collect();
            
        format!(".{:<width$}", suffix, width = width-1)
    }

    fn create_header(&self) {
         // Top border
        print!("{BOLD}┌");
        for (i, h) in self.headers.iter().enumerate() {
            print!("{:─<width$}", "─", width = h.width + 2);
            if i < self.headers.len() - 1 {
                print!("┬");
            } else {
                print!("┐{RESET}\n");
            }
        }

        // Header row
        print!("{BOLD}│");
        for h in &self.headers {
            print!(" {} │", h.label);
        }
        print!("{RESET}\n");

        // Middle separator
        print!("{BOLD}├");
        for (i, h) in self.headers.iter().enumerate() {
            print!("{:─<width$}", "─", width = h.width + 2);
            if i < self.headers.len() - 1 {
                print!("┼");
            } else {
                print!("┤{RESET}\n");
            }
        }
    }

    fn create_bottom(&self) {        
        // Bottom border
        print!("{BOLD}└");
        for (i, h) in self.headers.iter().enumerate() {
            print!("{:─<width$}", "─", width = h.width + 2);
            if i < self.headers.len() - 1 {
                print!("┴");
            } else {
                print!("┘{RESET}\n");
            }
        }
    }
}