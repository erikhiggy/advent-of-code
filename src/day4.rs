use regex::Regex;

fn get_input() -> Vec<&'static str> {
    include_str!("./inputs/day4.txt").split("\n\n").collect()
}

fn parse_input(input: Vec<&str>) -> Vec<Vec<&str>> {
    input.iter().map(|line| line.split_whitespace().collect()).collect()
}

/// go through entries and determine if the passport is valid
fn validate_passport_pt_1(passport: &Vec<&str>) -> bool {
    let mut field_counter = 0;
    for key in passport.iter() {
        match &key[..3] {
            "byr" => field_counter += 1,
            "iyr" => field_counter += 1,
            "eyr" => field_counter += 1,
            "hgt" => field_counter += 1,
            "hcl" => field_counter += 1,
            "ecl" => field_counter += 1,
            "pid" => field_counter += 1,
            _ => ()
        }
    }

    field_counter == 7
}

/// go through entries and determine if the passport is valid
fn validate_passport_pt_2(pid_pattern: &Regex, ecl_pattern: &Regex, cm_pattern: &Regex, in_pattern: &Regex, number_pattern: &Regex, color_pattern: &Regex, passport: &Vec<&str>) -> bool {
    let mut field_counter = 0;
    for key in passport.iter() {
        let value = &key[4..];
        match &key[..3] {
            "byr" => {
                if value.len() == 4 {
                    let num: i32 = value.parse().unwrap();
                    if num >= 1920 && num <= 2002 {
                        field_counter += 1;
                    }
                }
            },
            "iyr" => {
                if value.len() == 4 {
                    let num: i32 = value.parse().unwrap();
                    if num >= 2010 && num <= 2020 {
                        field_counter += 1;
                    }
                }
            },
            "eyr" => {
                if value.len() == 4 {
                    let num: i32 = value.parse().unwrap();
                    if num >= 2020 && num <= 2030 {
                        field_counter += 1;
                    }
                }
            },
            "hgt" => {
                let num_vec: Vec<&str> = number_pattern.split(value).collect();
                // if the value is not a number, quit out
                if num_vec[0].parse::<i32>().is_err() {
                    continue;
                }

                let num_from_val: i32 = num_vec[0].parse().unwrap();

                if cm_pattern.is_match(value) {
                    if num_from_val >= 150 && num_from_val <= 193 {
                        field_counter += 1;
                    }
                } else if in_pattern.is_match(value) {
                    if num_from_val >= 59 && num_from_val <= 76 {
                        field_counter += 1;
                    }
                }
            },
            "hcl" => {
                if color_pattern.is_match(value) {
                    field_counter += 1;
                }
            },
            "ecl" => {
                if ecl_pattern.is_match(value) {
                    field_counter += 1;
                }
            },
            "pid" => {
                if pid_pattern.is_match(value) {
                    field_counter += 1;
                }
            },
            _ => ()
        }
    }

    field_counter == 7
}

pub fn get_solution_pt_1() -> i32 {
    let parsed_input = parse_input(get_input());
    let mut valid_passports = 0;
    println!("Parsed Input: {:?}", parsed_input);
    for passport in parsed_input.iter() {
        if validate_passport_pt_1(passport) {
            valid_passports += 1;
        }
    }

    valid_passports
}

pub fn get_solution_pt_2() -> i32 {
    let parsed_input = parse_input(get_input());
    let mut valid_passports = 0;
    let cm_matcher: Regex = Regex::new(r"cm").unwrap();
    let in_matcher: Regex = Regex::new(r"in").unwrap();
    let num_matcher: Regex = Regex::new(r"[a-zA-Z]+").unwrap();
    let color_matcher: Regex = Regex::new(r"[#]([a-zA-Z]|[0-9]){6}").unwrap();
    let pid_matcher: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    let ecl_matcher: Regex = Regex::new(r"^amb$|^blu$|^brn$|^gry$|^grn$|^hzl$|^oth$").unwrap();
    for passport in parsed_input.iter() {
        println!("Passport: {:?}", passport);
        if validate_passport_pt_2(&pid_matcher, &ecl_matcher, &cm_matcher, &in_matcher, &num_matcher, &color_matcher, passport) {
            valid_passports += 1;
        }
    }

    valid_passports
}
