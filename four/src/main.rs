fn main() {
    let passports = parse(std::fs::read_to_string("input.txt").unwrap());
    println!("total: {}", count_valid(passports));
}

struct Passport {
    byr: Option<i64>,
    iyr: Option<i64>,
    eyr: Option<i64>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<i64>,
}

impl Passport {
    fn default() -> Self {
        Passport{
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn from_lines(lines: Vec<String>) -> Passport {
        let mut pp = Passport::default();
        for line in lines {
            let key_values : Vec<&str> = line.split(" ").collect();
            for key_value in key_values {
                let kv : Vec<&str> = key_value.split(":").collect();
                pp.set_value(kv[0], kv[1]);
            }
        }
        pp
    }

    fn is_valid(&self) -> bool {
        !(self.byr.is_none() ||
        self.iyr.is_none() ||
        self.eyr.is_none() ||
        self.hgt.is_none() ||
        self.hcl.is_none() ||
        self.ecl.is_none() ||
        self.pid.is_none())
    }

    fn set_value(&mut self, key: &str, value: &str) {
        match key {
            "byr" => self.byr = Some(value.parse().expect("byr not int")),
            "iyr" => self.iyr = Some(value.parse().expect("iyr not int")),
            "eyr" => self.eyr = Some(value.parse().expect("eyr not int")),
            "hgt" => self.hgt = Some(value.to_string()),
            "hcl" => self.hcl = Some(value.to_string()),
            "ecl" => self.ecl = Some(value.to_string()),
            "pid" => self.pid = Some(value.to_string()),
            "cid" => self.cid = Some(value.parse().expect("cid not int")),
            _ => panic!("not a property")
        }
    }
}

fn count_valid(passports: Vec<Passport>) -> usize {
    passports.iter().filter(|pp| pp.is_valid()).count()
}

fn parse(input: String) -> Vec<Passport> {
    let mut passports = Vec::new();
    let mut lines = Vec::new();
    for line in input.lines() {
        if line.len() == 0 {
            passports.push(Passport::from_lines(lines.clone()));
            lines = Vec::new();
        } else {
            lines.push(line.to_string())
        }
    }
    passports.push(Passport::from_lines(lines.clone()));
    passports
}

#[test]
fn test_input() {
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

    let passports = parse(input.to_string());
    assert_eq!(passports.len(), 4);
    assert_eq!(passports.get(0).unwrap().byr, Some(1937));
    assert_eq!(passports.get(1).unwrap().hcl, Some("#cfa07d".to_string()));
    assert_eq!(passports.get(2).unwrap().ecl, Some("brn".to_string()));
    assert_eq!(passports.get(3).unwrap().hgt, Some("59in".to_string()));

    assert_eq!(
        count_valid(passports),
        2);
}