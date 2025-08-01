pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        Vec::new()
    }else{
    let rows = garden.len();
    let cols = garden[0].len();
    let directions = [
        (-1, -1),(-1, 0),(-1, 1),
        (0, -1),(0, 1),
        (1, -1),(1, 0), (1, 1)
    ];

    (0..rows)
        .map(|i| {
            (0..cols)
                .map(|j| {
                    let c = garden[i].as_bytes()[j] as char; 
                    if c != '*' {
                        let n = directions.iter().filter(|&&(dx, dy)| {
                            let ni = i as isize + dx;
                            let nj = j as isize + dy;
                            ni >= 0
                                && ni < rows as isize
                                && nj >= 0
                                && nj < cols as isize
                                && garden[ni as usize].as_bytes()[nj as usize] as char == '*'
                        }).count();
                        if n > 0 {
                            std::char::from_digit(n as u32, 10).unwrap()
                        } else {
                            ' '
                        }
                    } else {
                        c
                    }
                })
                .collect::<String>()
        })
        .collect()
    }
}

