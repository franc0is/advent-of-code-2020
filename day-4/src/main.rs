use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let passports: Vec<&str> = input.split("\n\n").collect();
    let field_re = Regex::new(r"(\D{3}):([#0-9a-z]+)").unwrap();
    let pic_re = Regex::new(r"^\d{9}$").unwrap();
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let hgt_re = Regex::new(r"(\d+)").unwrap();
    let mut result = 0;
    for passport in passports {
        //println!("{:?}", passport);
        //let num_fields = field_re.captures_iter(passport).count();
        ////println!("num fields: {}", num_fields);
        //if num_fields == 8 {
        //    //println!("All 8 fields");
        //    result = result + 1;
        //    continue;
        //}
        //if num_fields == 7 && !passport.contains("cid") {
        //    //println!("All 7 fields, no CID");
        //    result = result + 1;
        //    continue
        //}
        let mut num_valid_fields = 0;
        for field in field_re.captures_iter(passport) {
            match &field[1] {
                "byr" => {
                    let year: u32 = field[2].parse().unwrap_or(0);
                    if year >= 1920 && year <= 2002 {
                        num_valid_fields = num_valid_fields + 1;
                    }
                    //println!("valid byr");
                },
                "iyr" => {
                    let year: u32 = field[2].parse().unwrap_or(0);
                    if year >= 2010 && year <= 2020 {
                        num_valid_fields = num_valid_fields + 1;
                    }
                    //println!("valid iyr");
                },
                "eyr" => {
                    let year: u32 = field[2].parse().unwrap_or(0);
                    if year >= 2020 && year <= 2030 {
                        num_valid_fields = num_valid_fields + 1;
                    }
                    //println!("valid eyr");
                },
                "pid" => {
                    if pic_re.is_match(&field[2]) {
                        num_valid_fields = num_valid_fields + 1;
                    }
                    //println!("valid pic");
                },
                "ecl" => {
                    match &field[2] {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
                            num_valid_fields = num_valid_fields + 1;
                        }
                        _ => {}
                    }
                    //println!("valid ecl");
                },
                "hcl" => {
                    if hcl_re.is_match(&field[2]) {
                        num_valid_fields = num_valid_fields + 1;
                    }
                    //println!("valid hcl");
                },
                "hgt" => {
                    let hgt_string = hgt_re.captures(&field[2]).unwrap().get(1).unwrap().as_str();
                    let height: u32 = hgt_string.parse().unwrap();
                    if field[2].contains("in") && height >= 59 && height <= 76 {
                        num_valid_fields = num_valid_fields + 1;
                    } else if field[2].contains("cm") && height >= 150 && height <= 193 {
                        num_valid_fields = num_valid_fields + 1;
                    }
                    //println!("valid hgt");
                }
                _ => {

                }
            }
        }
        if num_valid_fields == 7 {
            //println!("Valid Passport !!!!");
            result = result + 1;
        }
    }

    println!("Result is {}", result);
}
