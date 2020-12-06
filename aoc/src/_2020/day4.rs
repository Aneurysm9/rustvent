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
            .collect::<Vec<Vec<_>>>()
            .len()
            .to_string();
    }

    fn run_b(&self) -> String {
        return self
            .input
            .split("\n\n")
            .map(|l| l.split_whitespace().collect())
            .filter(|e| has_fields(e))
            .filter(|e| validate(e))
            .collect::<Vec<Vec<_>>>()
            .len()
            .to_string();
    }
}

fn has_fields(passport: &Vec<&str>) -> bool {
    if passport.len() == 8 {
        return true;
    } else if passport.len() == 7
        && passport
            .iter()
            .filter(|s| s.contains("cid:"))
            .collect::<Vec<_>>()
            .len()
            == 0
    {
        return true;
    }
    return false;
}

fn validate(passport: &Vec<&str>) -> bool {
    lazy_static! {
        static ref HCL_RE: Regex = Regex::new(r"^\#[\da-f]{6}$").unwrap();
        static ref HGT_RE: Regex = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
        static ref ECL_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
    }
    for field in passport.iter() {
        let mut i = field.split(":");
        let key = i.next().unwrap();
        let val = i.next().unwrap();
        match key {
            "byr" => {
                let byr: u32 = val.trim().parse().expect("Cannot parse byr");
                if byr < 1920 || byr > 2002 {
                    return false;
                }
            }
            "iyr" => {
                let iyr: u32 = val.trim().parse().expect("Cannot parse byr");
                if iyr < 2010 || iyr > 2020 {
                    return false;
                }
            }
            "eyr" => {
                let eyr: u32 = val.trim().parse().expect("Cannot parse byr");
                if eyr < 2020 || eyr > 2030 {
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
                            if hgt < 150 || hgt > 193 {
                                return false;
                            }
                        }
                        "in" => {
                            if hgt < 59 || hgt > 76 {
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
    return true;
}
