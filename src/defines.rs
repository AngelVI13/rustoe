pub const ROWS: usize = 3;
pub const BOARD_SIZE: usize = ROWS*ROWS;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Mark {
    X = 1,
    O = -1,
    NoPlayer = 0,
}

// Game result scores
pub const LOSS: f32 = 0.0;
pub const DRAW: f32 = 0.5;
pub const WIN: f32 = 1.0;

pub fn get_result_lines() -> Vec<Vec<Vec<usize>>> {
    let col_vec = get_column_vector();
    let row_vec = get_row_vector(&col_vec);
    let diagonal_vec = get_diagonals(&row_vec);

    let result_lines = vec![col_vec, row_vec, diagonal_vec];
    result_lines
}

fn get_row_vector(col_vec: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut row_vec: Vec<Vec<usize>> = vec![vec![0; ROWS]; ROWS];

    for i in 0..ROWS {
        for j in 0..ROWS {
            // performs the equivalent of zip on a list of lists in python
            // [[1, 2], [3, 4]] -> [[1, 3], [2, 4]]
            row_vec[i][j] = col_vec[j][i];
        }
    }

    row_vec
}

fn get_column_vector() -> Vec<Vec<usize>> {
    let mut col_vec: Vec<Vec<usize>> = vec![vec![0; ROWS]; ROWS];

    // take every nth element from board starting from each of the first row columns
    // i.e. 0-3-6, 1-4-7 etc.
    for i in 0..ROWS {
        let v_temp = (i..BOARD_SIZE).
            step_by(ROWS).
            collect::<Vec<usize>>();

        col_vec[i] = v_temp;
    }

    col_vec
}


fn get_diagonals(row_vec: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut left_diagonal: Vec<usize> = vec![0; ROWS];
    let mut right_diagonal: Vec<usize> = vec![0; ROWS];

    for i in 0..ROWS {
        left_diagonal[i] = row_vec[i][i]; // left diagonal -> 0,0| 1,1| 2,2
        right_diagonal[i] = row_vec[i][ROWS-i-1] // right diagonal 0,2 | 1,1| 2,0 .
        // Additional -1 is needed to convert from size to index
    }

    let diagonals = vec![left_diagonal, right_diagonal];
    diagonals
}
