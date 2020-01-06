pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let list: Vec<Vec<char>> = minefield.iter().map(|x| x.chars().collect()).collect();
    let mut result: Vec<Vec<u8>> = vec![vec![0; list[0].len()]; list.len()];

    for (i, row) in list.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c.to_owned() == '*' {
                count_mine(i, j, row.len(), list.len(), &mut result);
            }
        }
    }

    result.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().map(|(j, c)| {
            match (list[i][j].to_owned(), c) {
                ('*', _) => "*".to_owned(),
                (_, 0) => " ".to_owned(),
                (_, _) => c.to_string(),
            }
        }).collect()
    }).collect()
}

fn count_mine(i: usize, j: usize, row_len: usize, col_len: usize, board: &mut Vec<Vec<u8>>) {
    for m in (i as isize) - 1..=(i as isize) + 1 {
        for n in (j as isize) - 1..=(j as isize) + 1 {
            if m >= 0 && n >= 0 && m < col_len as isize && n < row_len as isize {
                board[m as usize][n as usize] += 1;
            }
        }
    }
}
