pub mod error;
pub mod prelude;
pub mod table;

use error::Error;
use std::fmt::{Debug, Display};
// use table::generate_table_string_vec;

pub trait Tabluate<T>
where
    T: Display + Debug,
{
    fn print_from_vec_to_stdout(&self);

    fn write_to_file_from_vec<S>(&self, filename: S) -> Result<(), Error>
    where
        S: Into<String>;

    fn print_from_csv_to_stdout(&self);

    fn write_to_file_from_csv<S>(&self, filename: S) -> Result<(), Error>
    where
        S: Into<String>;
}

pub enum Alignment {
    LEFT,
    RIGHT,
    CENTER,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::LEFT
    }
}

pub struct TableOptions {
    pub max_rows: Option<usize>,
    pub max_cols: Option<usize>,
    pub only_display_cols: Option<Vec<usize>>,
    pub alignment: Alignment,
}

impl Default for TableOptions {
    fn default() -> Self {
        TableOptions {
            max_rows: None,
            max_cols: None,
            only_display_cols: None,
            alignment: Alignment::CENTER,
        }
    }
}

pub struct Table;

impl<T> Tabluate<T> for Table
where
    T: Display + Debug,
{
    fn print_from_vec_to_stdout(&self) {
        todo!()
    }

    fn write_to_file_from_vec<S>(&self, filename: S) -> Result<(), Error>
    where
        S: Into<String>,
    {
        todo!()
    }

    fn print_from_csv_to_stdout(&self) {
        todo!()
    }

    fn write_to_file_from_csv<S>(&self, filename: S) -> Result<(), Error>
    where
        S: Into<String>,
    {
        todo!()
    }
}
