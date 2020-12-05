use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
enum InvalidPassport {
    MissingField(&'static str),
    InvalidField(&'static str),
}

impl Error for InvalidPassport {}

impl Display for InvalidPassport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let msg = match self {
            InvalidPassport::MissingField(field) => format!("Missing field `{}`", field),
            InvalidPassport::InvalidField(field) => format!("Invalid field `{}`", field),
        };
        write!(f, "InvalidPassport: {}", msg)
    }
}

#[derive(Debug, Default)]
struct Passport {
    // (Birth Year)
    byr: String,
    // (Issue Year)
    iyr: String,
    // (Expiration Year)
    eyr: String,
    // (Height)
    hgt: String,
    // (Hair Color)
    hcl: String,
    // (Eye Color)
    ecl: String,
    // (Passport ID)
    pid: String,
    // (Country ID)
    cid: Option<String>,
}

impl Passport {
    pub fn validate(&self) -> Result<(), InvalidPassport> {
        self.check_is_in_range(self.byr.as_str(), 1920, 2002, "byr")?;
        self.check_is_in_range(self.iyr.as_str(), 2010, 2020, "iry")?;
        self.check_is_in_range(self.eyr.as_str(), 2020, 2030, "eyr")?;
        self.validate_height()?;
        self.validate_hair_color()?;
        self.validate_eye_color()?;
        self.validate_passport_id()?;

        Ok(())
    }

    fn check_is_in_range(
        &self,
        value: &str,
        min: u32,
        max: u32,
        field: &'static str,
    ) -> Result<(), InvalidPassport> {
        let value: u32 = value
            .parse()
            .map_err(|_| InvalidPassport::InvalidField(field))?;
        if value < min || value > max {
            return Err(InvalidPassport::InvalidField(field));
        }
        Ok(())
    }

    fn validate_height(&self) -> Result<(), InvalidPassport> {
        let len = self.hgt.len();
        let num_chars = &self.hgt[0..len - 2];
        if !num_chars.chars().all(char::is_numeric) {
            return Err(InvalidPassport::InvalidField("hgt"));
        }
        if self.hgt.ends_with("cm") {
            self.check_is_in_range(num_chars, 150, 193, "hgt")?;
        } else if self.hgt.ends_with("in") {
            self.check_is_in_range(num_chars, 59, 76, "hgt")?;
        } else {
            return Err(InvalidPassport::InvalidField("hgt"));
        }
        Ok(())
    }

    fn validate_hair_color(&self) -> Result<(), InvalidPassport> {
        if !self.hcl.starts_with('#') {
            return Err(InvalidPassport::InvalidField("hcl"));
        }

        if !self.hcl.len() == 7 {
            return Err(InvalidPassport::InvalidField("hcl"));
        }

        if self
            .hcl
            .chars()
            .any(|c| (c < '0' && c > '9') || (c < 'a' && c > 'f'))
        {
            return Err(InvalidPassport::InvalidField("hcl"));
        }

        Ok(())
    }

    fn validate_eye_color(&self) -> Result<(), InvalidPassport> {
        let result = self.ecl == "amb"
            || self.ecl == "blu"
            || self.ecl == "brn"
            || self.ecl == "gry"
            || self.ecl == "grn"
            || self.ecl == "hzl"
            || self.ecl == "oth";
        if result {
            Ok(())
        } else {
            Err(InvalidPassport::InvalidField("ecl"))
        }
    }

    fn validate_passport_id(&self) -> Result<(), InvalidPassport> {
        let result = self.pid.len() == 9 && self.pid.chars().all(char::is_numeric);
        if result {
            Ok(())
        } else {
            Err(InvalidPassport::InvalidField("pid"))
        }
    }
}

impl TryFrom<&str> for Passport {
    type Error = InvalidPassport;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts: HashMap<String, String> = value
            .split_whitespace()
            .map(|entry| {
                let mut parts = entry.split(':');
                let key = String::from(parts.next().unwrap());
                let value = String::from(parts.next().unwrap());
                (key, value)
            })
            .collect();

        let byr = parts
            .remove("byr")
            .ok_or(InvalidPassport::MissingField("byr"))?;

        let iyr = parts
            .remove("iyr")
            .ok_or(InvalidPassport::MissingField("iyr"))?;

        let eyr = parts
            .remove("eyr")
            .ok_or(InvalidPassport::MissingField("eyr"))?;

        let hgt = parts
            .remove("hgt")
            .ok_or(InvalidPassport::MissingField("hgt"))?;

        let hcl = parts
            .remove("hcl")
            .ok_or(InvalidPassport::MissingField("hcl"))?;

        let ecl = parts
            .remove("ecl")
            .ok_or(InvalidPassport::MissingField("ecl"))?;

        let pid = parts
            .remove("pid")
            .ok_or(InvalidPassport::MissingField("pid"))?;

        let cid = parts.remove("cid");

        Ok(Self {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
            cid,
        })
    }
}

impl TryFrom<String> for Passport {
    type Error = InvalidPassport;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

fn get_passport_entries(input: &str) -> Vec<String> {
    let mut entries = vec![String::new()];
    input.lines().for_each(|line| {
        if line.is_empty() {
            entries.push(String::new());
        } else {
            let last_i = entries.len() - 1;
            entries[last_i].push_str(line);
            entries[last_i].push(' ');
        }
    });
    entries
}

fn count_passports_with_required_fields(input: &str) -> usize {
    get_passport_entries(input)
        .into_iter()
        .map(Passport::try_from)
        .filter(Result::is_ok)
        .count()
}

fn count_passports_with_valid_fields(input: &str) -> usize {
    get_passport_entries(input)
        .into_iter()
        .map(Passport::try_from)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .map(|passport| passport.validate())
        .filter(Result::is_ok)
        .count()
}

fn main() {
    let input = include_str!("../input.txt");

    let part1 = count_passports_with_required_fields(input);
    println!("Part 1 solution: {}", part1);
    let part2 = count_passports_with_valid_fields(input);
    println!("Part 2 solution: {}", part2);
}

#[test]
fn part1_works() {
    let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
    let result = count_passports_with_required_fields(input);
    assert_eq!(2, result);
}

#[test]
fn part2_works1() {
    let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
    let result = count_passports_with_valid_fields(input);
    assert_eq!(0, result);
}

#[test]
fn part2_works2() {
    let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
    let result = count_passports_with_valid_fields(input);
    assert_eq!(4, result);
}
