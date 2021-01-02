use regex::Regex;
pub fn get_input() -> Vec<&'static str> {
    include_str!("./inputs/day6.txt").split(|line| (line == "\r\n\r\n") || (line == "\r\n")).collect()
}