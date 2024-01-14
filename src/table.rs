use crate::TableOptions;

const THIN_HR_LINE_CHAR: char = '-';
const THINK_VR_LINE_CHAR: char = '|';

pub(crate) fn generate_table_string_vec<S>(
    rows: Vec<Vec<S>>,
    options: Option<TableOptions>,
) -> Vec<String>
where
    S: Into<String>,
{
    let mut table_string_vec: Vec<String> = Vec::new();
    let options = options.unwrap_or_default();
    let mut size_arr: Vec<usize> = Vec::new();

    let rows = rows
        .into_iter()
        .map(|row| {
            row.into_iter()
                .enumerate()
                .map(|(cell_no, cell)| {
                    let cell_string: String = cell.into();

                    if cell_string.len() > size_arr[cell_no] {
                        size_arr[cell_no] = cell_string.len();
                    }

                    cell_string
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let draw_hr_line = || {};

    table_string_vec
}
