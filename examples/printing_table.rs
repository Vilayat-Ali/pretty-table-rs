use pretty_table::prelude::*;

fn main() {
    // define your table as 2-D vectors where all vectors must have `EQUAL` lengths
    let table_data = vec![
        vec!["Name", "Age", "Salary"], // header
        vec!["Altmann", "45", "11.0k"],
        vec!["Bezos", "32", "99.34k"],
        vec!["Pichai", "56", "9.9m"],
        vec!["Cook", "43", "8.2m"],
    ];

    // print to terminal/standard output
    print_table!(table_data.clone());

    // write to file
    write_table_to_file("table.txt", table_data);
}
