pub(crate) fn a(input: &str) -> usize {
    let mut tree_count = 0;
    for (line_num, line) in input.lines().enumerate() {
        let pos = (3*line_num) % line.len();
        if line.chars().nth(pos).unwrap() == '#' {
            tree_count += 1;
        }
    }
    tree_count
}

pub(crate) fn b(input: &str) -> usize {
    let slopes = vec![1.0, 3.0, 5.0, 7.0, 0.5];
    let mut tree_count_mul = 1;
    for slope in slopes {
        let mut tree_count = 0;
        for (line_num, line) in input.lines().enumerate() {
            if slope == 0.5 && line_num % 2 != 0 {
                continue;
            }
            let pos = (slope*(line_num as f32)).round() as usize % line.len();
            if line.chars().nth(pos).or(Some('.')) == Some('#') {
                tree_count += 1;
            }
        }
        tree_count_mul *= tree_count;
    }
    tree_count_mul
}
