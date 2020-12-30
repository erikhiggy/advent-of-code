use regex::Regex;
use std::error::Error;

const NUMBER_MATCHER: Regex = Regex::new(r"/\d+/").unwrap();
const CM_MATCHER: Regex = Regex::new(r"/\d+cm/gm").unwrap();
const IN_MATCHER: Regex = Regex::new(r"/\d+cm/gm").unwrap();

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
fn validate_passport_pt_2(passport: &Vec<&str>) -> bool {
    let mut field_counter = 0;
    for key in passport.iter() {
        let value = &key[3..];
        match &key[..3] {
            "byr" => {
                if value.len() == 4
                    && (value.parse().unwrap() >= 1920
                    && value.parse().unwrap() <= 2002) {
                    field_counter += 1;
                }
            },
            "iyr" => {
                if value.len() == 4
                    && (value.parse().unwrap() >= 2010
                    && value.parse().unwrap() <= 2020) {
                    field_counter += 1;
                }
            },
            "eyr" => {
                if value.len() == 4
                    && (value.parse().unwrap() >= 2020
                    && value.parse().unwrap() <= 2030) {
                    field_counter += 1;
                }
            },
            "hgt" => {
                if cm_matcher.is_match(value) {

                }
            },
            "hcl" => field_counter += 1,
            "ecl" => field_counter += 1,
            "pid" => field_counter += 1,
            _ => ()
        }
    }

    field_counter == 7
}

pub fn get_solution_pt_1() -> i32 {
    let parsed_input = parse_input(get_input());
    let mut valid_passports = 0;
    for passport in parsed_input.iter() {
        if validate_passport_pt_1(passport) {
            valid_passports += 1;
        }
    }

    valid_passports
}
