use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

extern crate regex;

use regex::Regex;

fn main() {
    let re_date = Regex::new(r"^Données de réalisation du (\d{2}/\d{2}/\d{4})$").unwrap();
    let re_entry = Regex::new(r"^\d").unwrap();
    let f = File::open("DonnéesRTE2017.csv").unwrap();

    let file = BufReader::new(&f);
    let mut last_date = None;
    println!("Date	Heures	Biomasse	Gaz	Charbon	Fioul	Hydraulique STEP	Hydraulique fil de l'eau / éclusée	Hydraulique lacs	Nucléaire	Solaire	Déchets	Éolien terrestre	Total");
    for line in file.lines() {
        let mut l = line.unwrap();
        update_last_date(&mut last_date, &l, &re_date);
        update_entry_with_date(&last_date, &l, &re_entry);
    }
}

fn update_last_date(last_date : &mut Option<String>, line : &str, re: &Regex){
    let maybe_caps = re.captures(&line);
    match maybe_caps {
        None => {
            // do nothing
        },
        Some(captures) => *last_date = Some((&captures[1]).to_owned()),
    }
}

fn update_entry_with_date(last_date : &Option<String>, line : &str, re: &Regex){
    match *last_date {
        Some(ref date) => {
            if re.is_match(line) {
                println!("{}	{}",date, line);
            }
        }
        None => panic!()
    }
}
