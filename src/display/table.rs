use crate::{
    claudebar_types::display::HeaderInfo,   
    common::colors::*,
};

/// The TableCreator handles rendering table data
pub struct TableCreator {
    headers: Vec<HeaderInfo>,
    rows: Vec<Vec<String>>,
    has_warnings: bool,
}

impl TableCreator {
    pub fn new(headers: Vec<HeaderInfo>) -> Self {
        Self { 
            headers,
            rows: Vec::new(),
            has_warnings: false,
        }
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
                        let suffix: String = value
                            .chars()
                            .rev()
                            .take(header.width - 1)
                            .collect::<String>()
                            .chars()
                            .rev()
                            .collect();
                        format!("{:>width$}", format!(".{}", suffix), width = header.width)
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

    fn create_header(&self) {
         // Top border
        print!("{BOLD}┌");
        for (i, h) in self.headers.iter().enumerate() {
            print!("{:─<width$}", "─", width = h.width);
            if i < self.headers.len() - 1 {
                print!("┬");
            } else {
                print!("┐{RESET}\n");
            }
        }

        // Header row
        print!("{BOLD}│");
        for h in &self.headers {
            print!("{:<width$}│", h.label, width = h.width);
        }
        print!("{RESET}\n");

        // Middle separator
        print!("{BOLD}├");
        for (i, h) in self.headers.iter().enumerate() {
            print!("{:─<width$}", "─", width = h.width);
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