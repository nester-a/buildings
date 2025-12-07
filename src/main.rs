use chrono::{Datelike, Local};
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::{metadata, read_dir};
use std::io::ErrorKind;

fn main() {
    let mut f_name = String::new();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        f_name = args[1].to_string();
    }
    else{
        let dir = env::current_dir().unwrap().as_path().to_str().unwrap().to_string();
        let pos_fn = find_first(&dir, ".csv");
        match pos_fn {
            Some(f) => f_name = f,
            None => {
                println!("Файл с данными о зданиях не найден");
                return;
            }
        }
    }

    if let Err(e) = get_f_with_data(&f_name){
        println!("Ошибка при поиске файла {:?}", e);
    }

    match get_data_from_f(&f_name){
        Ok(data) => {
            let oldest = get_oldest(&data);
            view_vec(&oldest);
            println!("Работа программы завершена")
        }
        Err(e) => {
            println!("Ошибка при загрузке данных из файла {:?}", e);
        }
    }

}

#[derive(Debug)]
enum FileError {
    FileNotFound,
    FileIsEmpty,
}

#[derive(Debug)]
enum AppError {
    NotValidValue(String, String),
    NotUniqueValue(Building),
}

fn find_first(dir: &str, ext: &str) -> Option<String> {
    let entries = read_dir(dir);

    if let Err(_) = entries {
        return None;
    }

    let mut f_name = String::new();
    let ent = entries.unwrap();
    for entry in ent {
        let entry = entry.unwrap();
        let md = entry.metadata().unwrap();

        if md.is_file() && entry.file_name().to_str().unwrap().contains(ext) {
            f_name = entry.file_name().to_str().unwrap().to_string();
            break;
        }
    }
    Some(f_name)
}
fn get_f_with_data(f_name: &str) -> Result<(), FileError> {
    match metadata(f_name) {
        Ok(m) => {
            if m.len() == 0 {
                return Err(FileError::FileIsEmpty);
            }
            Ok(())
        }
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                Err(FileError::FileNotFound)
            } else {
                panic!("Непредвиденная ошибка {e}")
            }
        }
    }
}

fn get_data_from_f(f_name: &str) -> Result<Vec<Building>, AppError> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_path(f_name)
        .unwrap();

    let mut res: Vec<Building> = Vec::new();

    for row in rdr.records() {
        let record = row.unwrap();

        let values: Vec<&str> = record.iter().collect();

        let dist = String::from(values[0]);
        let is_dist_correct = rus_only(&dist);
        if !is_dist_correct {
            return Err(AppError::NotValidValue("dist".to_string(), dist));
        }

        let street = String::from(values[1]);
        let is_street_correct = rus_only(&street);
        if !is_street_correct {
            return Err(AppError::NotValidValue("street".to_string(), street));
        }

        let num: u8 = values[2].parse().unwrap();
        let is_num_correct = num_between(num, 1, u8::max_value());
        if !is_num_correct {
            return Err(AppError::NotValidValue("num".to_string(), num.to_string()));
        }

        let year: u8 = values[3].parse().unwrap();
        let is_year_correct = num_between(year, 1, Local::now().year() as u8);
        if !is_year_correct {
            return Err(AppError::NotValidValue(
                "year".to_string(),
                year.to_string(),
            ));
        }

        let bld = Building::new(dist, street, num, year);
        let is_bld_unique = is_unique(&bld, &res);
        if !is_bld_unique {
            return Err(AppError::NotUniqueValue(bld));
        }

        res.push(bld);
    }

    Ok(res)
}

fn get_oldest(blds: &Vec<Building>) -> Vec<&Building> {

    let mut hm: HashMap<String, &Building> = HashMap::new();

    for bld in blds{
        
        let pos_key = bld.add.dist.clone();
        match hm.get(&pos_key){
            Some(existed) => {
                if existed.build_year > bld.build_year{
                    hm.insert(pos_key, bld);
                }
            }
            None => {
                hm.insert(pos_key, bld);
            }
        }
    }

    let mut res: Vec<&Building> = Vec::new();
    for (_, bld) in hm {
        res.push(bld);
    }
    res
}

fn view_vec(vec: &Vec<&Building>){
    for bld in vec {
        println!("{}|{}|{}|{}", bld.add.dist, bld.add.str, bld.add.num, bld.build_year);
    }
}

#[derive(Debug)]
struct Address {
    dist: String,
    str: String,
    num: u8,
}

impl Address {
    fn new(dist: String, str: String, num: u8) -> Self {
        Address { dist, str, num }
    }
    fn is_same(&self, add: &Address) -> bool {
        self.dist == add.dist && self.str == add.str && self.num == add.num
    }
}

#[derive(Debug)]
struct Building {
    add: Address,
    build_year: u8,
}

impl Building {
    fn new(dist: String, str: String, num: u8, build_year: u8) -> Self {
        Building {
            add: Address::new(dist, str, num),
            build_year,
        }
    }
}

fn num_between(input: u8, min: u8, max: u8) -> bool {
    input >= min && input <= max
}

fn rus_only(input: &str) -> bool {
    let re = Regex::new(r"^[A-Za-z0-9\s!?']+$").unwrap();

    !re.is_match(input)
}

fn is_unique(b: &Building, vec: &Vec<Building>) -> bool {
    for cur in vec {
        if cur.add.is_same(&b.add) {
            return false;
        }
    }
    true
}
