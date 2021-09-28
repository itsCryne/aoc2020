use std::collections::HashMap;

pub(crate) fn a(input: &str) -> i32 {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valid_passports = 0;
    for passport in input.split("\n\n") {
        if required_fields.iter().all(|field| passport.contains(field)) {
            valid_passports += 1;
        }
    }
    valid_passports
}

pub(crate) fn b(input: &str) -> i32 {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valid_passports = 0;
    for passport in input.split("\n\n") {
        if required_fields.iter().all(|field| passport.contains(field)) {
            let mut field_map = HashMap::new();
            for line in passport.lines() {
                for field in line.split(" ") {
                    field_map.insert(field.split(":").next().unwrap(), field.split(":").last().unwrap());
                }
            }
            let byr_valid = (1920..=2003).contains(&field_map.get("byr").unwrap().parse().unwrap_or(0));
            let iyr_valid = (2010..=2020).contains(&field_map.get("iyr").unwrap().parse().unwrap_or(0));
            let eyr_valid = (2020..=2030).contains(&field_map.get("eyr").unwrap().parse().unwrap_or(0));
            let pid_valid = field_map.get("pid").unwrap().len() == 9 && field_map.get("pid").unwrap().parse().unwrap_or(0) > 0;
            let ecl_valid = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(field_map.get("ecl").unwrap());

            let hgt = field_map.get("hgt").unwrap().replace("in", "").replace("cm", "").parse().unwrap_or(0);
            let hgt_cm = field_map.get("hgt").unwrap().ends_with("cm");
            let hgt_valid = if hgt_cm {(150..=193).contains(&hgt)} else {(59..=76).contains(&hgt)};
            let hcl_valid = field_map.get("hcl").unwrap().starts_with("#")
                && i64::from_str_radix(&*field_map.get("hcl").unwrap().replace("#", ""), 16).unwrap_or(0) != 0;

            if byr_valid && iyr_valid && eyr_valid && pid_valid && ecl_valid && hgt_valid && hcl_valid {
                valid_passports += 1;
            }
        }
    }
    valid_passports
}
