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

struct Passport {
    fields: HashMap<PassportField, String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.fields.len() == 8
            || self.fields.len() == 7 && self.fields.get(&PassportField::CountryId).is_none()
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
    passports.iter().filter(|p| p.is_valid()).count()
}

fn main() {
    println!("part 1: {}", part1());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(part1(), 206)
    }
}
