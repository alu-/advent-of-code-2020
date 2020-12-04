use itertools::Itertools;
use raster::Color;
use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;

fn get_attribute(attributes: &HashMap<&str, &str>, key: &str) -> Option<String> {
    attributes
        .get(&key)
        .and_then(|x| -> Option<String> { Some(x.to_string()) })
}

#[derive(Default, Debug)]
struct Passport {
    // Birth Year
    byr: Option<String>,
    // Issue Year
    iyr: Option<String>,
    // Expiration Year
    eyr: Option<String>,
    // Height
    hgt: Option<String>,
    // Hair Color
    hcl: Option<String>,
    // Eye Color
    ecl: Option<String>,
    // Passport ID
    pid: Option<String>,
    // Country ID
    cid: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        return self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some();
    }

    fn is_extra_valid(&self) -> bool {
        let byr: usize = self.byr.as_ref().unwrap().parse::<usize>().unwrap();
        if byr.to_string().len() != 4 {
            return false;
        }
        if byr < 1920 || byr > 2002 {
            return false;
        }

        let iyr: usize = self.iyr.as_ref().unwrap().parse::<usize>().unwrap();
        if iyr.to_string().len() != 4 {
            return false;
        }
        if iyr < 2010 || iyr > 2020 {
            return false;
        }

        let eyr: usize = self.eyr.as_ref().unwrap().parse::<usize>().unwrap();
        if eyr.to_string().len() != 4 {
            return false;
        }
        if eyr < 2020 || eyr > 2030 {
            return false;
        }

        let hgt: &String = self.hgt.as_ref().unwrap();
        let height_type: String = hgt
            .chars()
            .rev()
            .take(2)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<String>();
        if height_type != "cm" && height_type != "in" {
            return false;
        }

        let height = hgt[..hgt.len() - 2].parse::<usize>().expect("Parse height");
        if height_type == "cm" && (height < 150 || height > 193) {
            return false;
        } else if height_type == "in" && (height < 59 || height > 76) {
            return false;
        }

        let hcl: &String = self.hcl.as_ref().unwrap();
        if hcl.len() != 7 {
            return false;
        }
        let hair_color = Color::hex(hcl);
        if hair_color.is_err() {
            return false;
        }

        let ecl: &String = self.ecl.as_ref().unwrap();
        match ecl.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
            _ => return false,
        };

        let pid: &String = self.pid.as_ref().unwrap();
        if pid.len() != 9 {
            return false;
        }
        let is_numeric_pin = pid.chars().all(char::is_numeric);
        if !is_numeric_pin {
            return false;
        }

        return true;
    }
}

impl FromStr for Passport {
    type Err = Infallible;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        let attributes: HashMap<&str, &str> = data
            .split_terminator(' ')
            .map(|x| {
                let tuple: (&str, &str) = x.split(":").next_tuple().expect("Parsing key value");
                tuple
            })
            .collect();

        Ok(Passport {
            byr: get_attribute(&attributes, &"byr"),
            iyr: get_attribute(&attributes, &"iyr"),
            eyr: get_attribute(&attributes, &"eyr"),
            hgt: get_attribute(&attributes, &"hgt"),
            hcl: get_attribute(&attributes, &"hcl"),
            ecl: get_attribute(&attributes, &"ecl"),
            pid: get_attribute(&attributes, &"pid"),
            cid: get_attribute(&attributes, &"cid"),
        })
    }
}

pub fn part1(input: &str) -> Result<usize, String> {
    Ok(input
        .split("\n\n")
        .map(|x| x.replace("\n", " "))
        .map(|p| p.parse::<Passport>().expect("Parse password"))
        .filter(|x| x.is_valid())
        .count())
}

pub fn part2(input: &str) -> Result<usize, String> {
    // 120 too low
    Ok(input
        .split("\n\n")
        .map(|x| x.replace("\n", " "))
        .map(|p| p.parse::<Passport>().expect("Parse password"))
        .filter(|x| x.is_valid())
        .filter(|x| x.is_extra_valid())
        .count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in\n";
        assert_eq!(part1(INPUT).unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        const INVALID: &str = "eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\niyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946\n\nhcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\nhgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007\n";
        const VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f\n\neyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\nhcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022\n\niyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719\n";
        assert_eq!(part2(INVALID).unwrap(), 0);
        assert_eq!(part2(VALID).unwrap(), 4);
    }
}
