use pretty_table::prelude::write_table_to_file;

fn main() {
    write_table_to_file(
        "Data.txt",
        vec![
            vec!["Ali", "Mohd"],
            vec!["Ali", "Mohd"],
            vec!["Ali", "Mohd"],
        ],
    )
    .unwrap();
}
