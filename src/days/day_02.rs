pub(crate) fn a(input: &str) -> i32 {
    let input_vec: Vec<&str> = input.split("\n").collect();
    let mut valid_password_count = 0;

    for line in input_vec {
        if line.len() == 0 {
            continue;
        }
        let min: i32 = line.split(" ").next().unwrap().split("-").next().unwrap().parse().unwrap();
        let max: i32 = line.split(" ").next().unwrap().split("-").last().unwrap().parse().unwrap();
        let char: char = line.split(" ").collect::<Vec<&str>>()[1].chars().nth(0).unwrap();
        let pw: &str = line.split(" ").last().unwrap();

        if pw.matches(&char.to_string()).count() as i32 >= min && pw.matches(&char.to_string()).count() as i32 <= max {
            valid_password_count += 1;
        }

    }
    return valid_password_count;
}

pub(crate) fn b(input: &str) -> i32 {
    let input_vec: Vec<&str> = input.split("\n").collect();
    let mut valid_password_count = 0;

    for line in input_vec {
        if line.len() == 0 {
            continue;
        }
        let first: i32 = line.split(" ").next().unwrap().split("-").next().unwrap().parse().unwrap();
        let second: i32 = line.split(" ").next().unwrap().split("-").last().unwrap().parse().unwrap();
        let char: char = line.split(" ").collect::<Vec<&str>>()[1].chars().nth(0).unwrap();
        let pw: &str = line.split(" ").last().unwrap();

        if (pw.chars().nth((first-1) as usize).unwrap() == char) != (pw.chars().nth((second-1) as usize).unwrap() == char) {
            valid_password_count += 1;
        }
    }

    return valid_password_count;
}
