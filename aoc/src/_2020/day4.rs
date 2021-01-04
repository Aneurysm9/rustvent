extern crate regex;

use regex::Regex;

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        return self
            .input
            .split("\n\n")
            .map(|l| l.split_whitespace().collect())
            .filter(|p| has_fields(p))
            .count()
            .to_string();
    }

    fn run_b(&self) -> String {
        return self
            .input
            .split("\n\n")
            .map(|l| l.split_whitespace().collect())
            .filter(|e| has_fields(e))
            .filter(|e| validate(e))
            .count()
            .to_string();
    }
}

#[allow(clippy::ptr_arg)]
fn has_fields(passport: &Vec<&str>) -> bool {
    if passport.len() == 8
        || passport.len() == 7 && passport.iter().filter(|s| s.contains("cid:")).count() == 0
    {
        return true;
    }
    false
}

#[allow(clippy::ptr_arg)]
fn validate(passport: &Vec<&str>) -> bool {
    lazy_static! {
        static ref HCL_RE: Regex = Regex::new(r"^\#[\da-f]{6}$").unwrap();
        static ref HGT_RE: Regex = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
        static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    for field in passport.iter() {
        let mut i = field.split(':');
        let key = i.next().unwrap();
        let val = i.next().unwrap();
        match key {
            "byr" => {
                let byr: u32 = val.trim().parse().expect("Cannot parse byr");
                if !(1920..=2002).contains(&byr) {
                    return false;
                }
            }
            "iyr" => {
                let iyr: u32 = val.trim().parse().expect("Cannot parse byr");
                if !(2010..=2020).contains(&iyr) {
                    return false;
                }
            }
            "eyr" => {
                let eyr: u32 = val.trim().parse().expect("Cannot parse byr");
                if !(2020..=2030).contains(&eyr) {
                    return false;
                }
            }
            "hgt" => {
                if !HGT_RE.is_match(val) {
                    return false;
                }
                for cap in HGT_RE.captures_iter(val) {
                    let hgt: u32 = cap[1].trim().parse().unwrap();
                    let unit = &cap[2];
                    match unit {
                        "cm" => {
                            if !(150..=193).contains(&hgt) {
                                return false;
                            }
                        }
                        "in" => {
                            if !(59..=76).contains(&hgt) {
                                return false;
                            }
                        }
                        _ => return false,
                    }
                }
            }
            "hcl" => {
                if !HCL_RE.is_match(val) {
                    return false;
                }
            }
            "ecl" => {
                if !ECL_RE.is_match(val) {
                    return false;
                }
            }
            "pid" => {
                if !PID_RE.is_match(val) {
                    return false;
                }
            }
            "cid" => {}
            _ => return false,
        }
    }
    true
}
