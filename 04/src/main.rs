#![deny(warnings, clippy::all)]

use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum PassportField {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportId,
    CountryId,
}

impl FromStr for PassportField {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "byr" => Ok(Self::BirthYear),
            "iyr" => Ok(Self::IssueYear),
            "eyr" => Ok(Self::ExpirationYear),
            "hgt" => Ok(Self::Height),
            "hcl" => Ok(Self::HairColor),
            "ecl" => Ok(Self::EyeColor),
            "pid" => Ok(Self::PassportId),
            "cid" => Ok(Self::CountryId),
            _ => Err(format!("Unknown field: \"{}\"", s)),
        }
    }
}

impl PassportField {
    fn is_number_in_range(s: &str, lower: u32, upper: u32) -> bool {
        if let Ok(num) = s.parse::<u32>() {
            num >= lower && num <= upper
        } else {
            false
        }
    }
    fn is_valid_height(s: &str) -> bool {
        s.ends_with("cm") && Self::is_number_in_range(&s.replace("cm", ""), 150, 193)
            || s.ends_with("in") && Self::is_number_in_range(&s.replace("in", ""), 59, 76)
    }
    fn is_hex(s: &str) -> bool {
        s.chars().all(|c| c.is_ascii_hexdigit())
    }
    fn is_valid_hair_color(s: &str) -> bool {
        s.starts_with('#') && s[1..].len() == 6 && Self::is_hex(&s[1..])
    }
    fn is_valid_eye_color(s: &str) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s)
    }
    fn is_valid_passport_id(s: &str) -> bool {
        s.len() == 9 && s.chars().all(|c| c.is_ascii_digit())
    }
    fn is_valid(&self, s: &str) -> bool {
        match self {
            Self::BirthYear => Self::is_number_in_range(s, 1920, 2002),
            Self::IssueYear => Self::is_number_in_range(s, 2010, 2020),
            Self::ExpirationYear => Self::is_number_in_range(s, 2020, 2030),
            Self::Height => Self::is_valid_height(s),
            Self::HairColor => Self::is_valid_hair_color(s),
            Self::EyeColor => Self::is_valid_eye_color(s),
            Self::PassportId => Self::is_valid_passport_id(s),
            Self::CountryId => true,
        }
    }
}

struct Passport {
    fields: HashMap<PassportField, String>,
}

impl Passport {
    fn is_valid_part1(&self) -> bool {
        self.fields.len() == 8
            || self.fields.len() == 7 && self.fields.get(&PassportField::CountryId).is_none()
    }
    fn is_valid_part2(&self) -> bool {
        self.is_valid_part1() && self.fields.iter().all(|(k, v)| k.is_valid(v))
    }
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            fields: s
                .split(|c| c == ' ' || c == '\n')
                .map(|s| {
                    let (field, data) = s
                        .split(':')
                        .collect_tuple()
                        .ok_or(format!("Incorrect field key/value pair: \"{}\"", s))?;
                    Ok((PassportField::from_str(field)?, data.to_string()))
                })
                .collect::<Result<HashMap<_, _>, String>>()?,
        })
    }
}

fn parse_input() -> Vec<Passport> {
    let data = include_str!("input.txt");
    data.split_terminator("\n\n")
        .map(|s| Passport::from_str(s).unwrap())
        .collect()
}

fn part1() -> usize {
    let passports = parse_input();
    passports.iter().filter(|p| p.is_valid_part1()).count()
}

fn part2() -> usize {
    let passports = parse_input();
    passports.iter().filter(|p| p.is_valid_part2()).count()
}

fn main() {
    println!("part 1: {}", part1());
    println!("part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 206)
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(), 123)
    }
}
