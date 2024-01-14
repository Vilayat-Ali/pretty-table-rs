//! # pretty-table
//!
//! `pretty-table` is a Rust crate that provides a simple and convenient way to pretty print tables
//! and write them to a file using data from 2-D vectors.
//!
//! ## Features
//!
//! - Create beautiful tables from vectors of data.
//! - Customize table formatting, alignment, and styling.
//! - Print tables to the console for easy visualization.
//!
//! ## Usage
//!
//! To use `pretty-table`, add it as a dependency in your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! pretty-table = "0.1.0"
//! ```
//!
//! ```rust
//! use pretty_table::prelude::*;
//!
//! fn main() {
//!     // define your table as 2-D vectors where all vectors must have `EQUAL` lengths
//!     let table_data = vec![
//!         vec!["Name", "Age", "Salary"], // header
//!         vec!["Altmann", "45", "11.0k"],
//!         vec!["Bezos", "32", "99.34k"],
//!         vec!["Pichai", "56", "9.9m"],
//!         vec!["Cook", "43", "8.2m"],
//!     ];
//!
//!     // print to terminal/standard output
//!     print_table!(table_data.clone());
//!
//!     // write to file
//!     write_table_to_file("table.txt", table_data);
//! }
//! ```
//!
//! ## Contributing
//!
//! Contributions are welcome! If you have any suggestions, bug reports, or want to contribute code,
//! please open an issue or submit a pull request on the [GitHub repository](https://github.com/vilayat-ali/pretty-table).
//!
//! ## License
//!
//! This crate is distributed under the terms of the MIT License. See the [LICENSE](https://github.com/Vilayat-Ali/pretty-table-rs/blob/main/LICENSE) file for details.

pub mod error;
pub mod prelude;
pub mod table;

pub struct TableOptions {
    pub max_rows: Option<usize>,
    pub max_cols: Option<usize>,
    pub only_display_cols: Option<Vec<usize>>,
}

impl Default for TableOptions {
    fn default() -> Self {
        TableOptions {
            max_rows: None,
            max_cols: None,
            only_display_cols: None,
        }
    }
}
