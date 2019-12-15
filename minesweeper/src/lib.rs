pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let list: Vec<Vec<char>> = minefield.iter().map(|x| x.chars().collect()).collect();
    let mut result: Vec<Vec<u8>> = vec![vec![0; list[0].len()]; list.len()];
    for (i, row) in list.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c.to_owned() == '*' {
                for m in (i as isize) - 1..=(i as isize) + 1 {
                    for n in (j as isize) - 1..=(j as isize) + 1 {
                        if m >= 0 && n >= 0 && m < list.len() as isize && n < row.len() as isize {
                            result[m as usize][n as usize] += 1;
                        }
                    }
                }
            }
        }
    }

    result.iter().enumerate().map(|(i, row)| {
        row.iter().enumerate().map(|(j, c)| {
            match list[i][j].to_owned() {
                '*' => "*".to_owned(),
                _ => if *c != 0 { c.to_string() } else { " ".to_owned() },
            }
        }).collect()
    }).collect()
}
