use crate::TableOptions;
use std::ops::Div;

const THIN_HR_LINE_CHAR: &'static str = "-";
const THICK_HR_LINE_CHAR: &'static str = "=";
const THIN_VR_LINE_CHAR: &'static str = "|";

pub(crate) fn generate_table_string_vec<S>(
    rows: Vec<Vec<S>>,
    options: Option<TableOptions>,
) -> Vec<String>
where
    S: Into<String>,
{
    let mut table_string_vec: Vec<String> = Vec::new();
    let _options = options.unwrap_or_default();
    let mut size_arr: Vec<usize> = vec![0; rows[0].len()];

    let rows = rows
        .into_iter()
        .map(|row| {
            row.into_iter()
                .enumerate()
                .map(|(cell_no, cell)| {
                    let cell_string: String = cell.into();

                    if (cell_string.len() + 2) > size_arr[cell_no] {
                        size_arr[cell_no] = cell_string.len() + 2;
                    }

                    cell_string
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    enum HrLineType {
        Header,
        Cell,
    }

    let draw_hr_line = |line_type: HrLineType| -> String {
        let mut line_string = String::from("+");

        for size in size_arr.iter() {
            match line_type {
                HrLineType::Header => line_string.push_str(&THICK_HR_LINE_CHAR.repeat(*size)),
                HrLineType::Cell => line_string.push_str(&THIN_HR_LINE_CHAR.repeat(*size)),
            }

            line_string.push('+');
        }

        line_string
    };

    let compute_spaces = |string_size: usize, reserved_size: usize| -> (usize, usize) {
        let partitioned_space = (reserved_size - string_size).div(2);

        match (reserved_size - string_size) % 2 == 0 {
            true => (partitioned_space, partitioned_space),
            false => (partitioned_space, partitioned_space + 1),
        }
    };

    table_string_vec.push(draw_hr_line(HrLineType::Header));
    table_string_vec.push({
        // processing headers to form the first row
        let mut header_row: String = String::from(THIN_VR_LINE_CHAR.to_string());

        for i in 0..rows[0].len() {
            let spaces = compute_spaces(rows[0][i].len(), size_arr[i]);
            header_row.push_str(&format!(
                "{}{}{}{THIN_VR_LINE_CHAR}",
                " ".repeat(spaces.0),
                rows[0][i],
                " ".repeat(spaces.1)
            ));
        }

        header_row
    });
    table_string_vec.push(draw_hr_line(HrLineType::Header));

    // processing all data rows in the table
    for row_no in 1..rows.len() {
        let mut row_string: String = String::from(THIN_VR_LINE_CHAR.to_string());
        for col_no in 0..rows[row_no].len() {
            let spaces = compute_spaces(rows[row_no][col_no].len(), size_arr[col_no]);
            row_string.push_str(&format!(
                "{}{}{}{THIN_VR_LINE_CHAR}",
                " ".repeat(spaces.0),
                rows[row_no][col_no],
                " ".repeat(spaces.1)
            ));
        }
        table_string_vec.push(row_string);
        table_string_vec.push(draw_hr_line(HrLineType::Cell));
    }

    table_string_vec
}
