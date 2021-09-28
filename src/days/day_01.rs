pub(crate) fn a(input: &str) -> i32 {
    let mut input_vec: Vec<&str> = input.split("\n").collect();
    input_vec.retain(|s| s.parse::<i32>().is_ok());
    let number_vec: Vec<i32> = input_vec.into_iter().map(|s| {
        s.parse().expect("Tried to parse invalid number")
    }).collect();

    for first_entry in &number_vec {
        for second_entry in &number_vec {
            if first_entry + second_entry == 2020 {
                return first_entry * second_entry
            }
        }
    }
    return -1;
}

pub(crate) fn b(input: &str) -> i32 {
    let mut input_vec: Vec<&str> = input.split("\n").collect();
    input_vec.retain(|s| s.parse::<i32>().is_ok());
    let number_vec: Vec<i32> = input_vec.into_iter().map(|s| {
        s.parse().expect("Tried to parse invalid number")
    }).collect();

    for first_entry in &number_vec {
        for second_entry in &number_vec {
            for third_entry in &number_vec {
                if first_entry + second_entry + third_entry == 2020 {
                    return first_entry * second_entry * third_entry
                }
            }
        }
    }
    return -1;
}

