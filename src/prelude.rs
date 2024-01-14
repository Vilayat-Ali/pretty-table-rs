pub use crate::error::Error;
pub use crate::print_table;
use crate::table::generate_table_string_vec;
pub use crate::TableOptions;
use std::{fs::File, io::Write};

/// This is a simple macro that takes 1 or more number of Vectors of any type that's coersable into `String` type, and print them as
/// pretty-formatted table in the standard output.
///
/// ## Example
///
/// ```rust
/// use pretty_table::prelude::*;
///
/// fn main() {
///     // usage with individual rows
///     print_table!(
///         vec!["Name", "Age", "Salary"], // header
///         vec!["Altmann", "45", "11.0k"],
///         vec!["Bezos", "32", "99.34k"],
///         vec!["Pichai", "56", "9.9m"],
///         vec!["Cook", "43", "8.2m"]
///     );
///
///     // usage with table data
///     print_table!(
///         vec![
///             vec!["Name", "Age", "Salary"], // header
///             vec!["Altmann", "45", "11.0k"],
///             vec!["Bezos", "32", "99.34k"],
///             vec!["Pichai", "56", "9.9m"],
///             vec!["Cook", "43", "8.2m"]
///         ]
///     );
/// }
/// ```
#[macro_export]
macro_rules! print_table {
    ($rows:expr) => {
        {
            use pretty_table::table::generate_table_string_vec;

            let table_rows: Vec<Vec<String>> = $rows.into_iter().map(|row| {
                row.into_iter().map(|data| data.into()).collect::<Vec<String>>()
            }).collect::<Vec<Vec<String>>>();

            for row in generate_table_string_vec(table_rows, None).into_iter() {
                println!("{}", row);
            }
        }
    };
    ($($row:expr), +) => {
        {
            use pretty_table::table::generate_table_string_vec;


            let table_rows: Vec<Vec<String>> = vec![
                $($row.into_iter().map(String::from).collect()),+
            ];

            for row in generate_table_string_vec(table_rows, None).into_iter() {
                println!("{}", row);
            }
        }
    }
}

/// This function is used to write the table into a file with specified filename. The function takes filename and table data as
/// 2-D Vector of any type that can be coerse into a `String` type.
///
/// ## Example
///
/// ```rust
/// use pretty_table::prelude::*;
///
/// fn main() {
///     let table_data = vec![
///         vec!["Name", "Age", "Salary"], // header
///         vec!["Altmann", "45", "11.0k"],
///         vec!["Bezos", "32", "99.34k"],
///         vec!["Pichai", "56", "9.9m"],
///         vec!["Cook", "43", "8.2m"],
///     ];
///
///     write_table_to_file("table.txt", table_data).unwrap();
/// }
/// ```
pub fn write_table_to_file<S>(filename: S, table_rows: Vec<Vec<S>>) -> Result<(), Error>
where
    S: Into<String>,
{
    let filename = filename.into();
    let rows = table_rows
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|field| field.into())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let prettified_table_rows_vec = generate_table_string_vec(rows, None);

    let mut file = File::create(&filename)?;

    for row in prettified_table_rows_vec.into_iter() {
        writeln!(file, "{}", row)?;
    }

    Ok(())
}
