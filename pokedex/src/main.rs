use std::fs::File;
use std::io;
use std::io::{BufReader, Error};
use std::io::prelude::*;

fn main() {
    let file = File::open("./src/data.txt")
        .expect("Could not open file!");

    let mut buff_reader = BufReader::new(file);
    
    // Read the header
    let mut first_line = String::new();
    buff_reader.read_line(&mut first_line)
        .expect("Could not read header line!");

    let pokemons: Vec<Pokemon> = buff_reader.lines()
        .map(map_line_to_pokemon)
        .collect();

    loop {
        println!("Please input the Pokemon name you want to search for:");

        let mut search = String::new();

        io::stdin().read_line(&mut search)
                .expect("Failed to read line");

        let search = search.trim();

        let filtered: Vec<&Pokemon>;

        match search.to_lowercase().as_str() {
            "quit" | "exit" => break,
            search => filtered = filter_pokemons(&pokemons, search),
        }

        let length = filtered.len();
        if length == 0 {
            println!("Found none!");
        } else {
            println!("Found {} pokemon(s)", length);
            for pokemon in filtered {
                println!("{:?}", pokemon);
            }
        }
    }
}

fn filter_pokemons<'a>(poks: &'a Vec<Pokemon>, name: &str) -> Vec<&'a Pokemon> {
    poks.iter()
        .filter(|&p| p.name.to_lowercase().contains(name))
        .collect()
}

fn map_line_to_pokemon(line: Result<String, Error>) -> Pokemon {
    let line = line.expect("Could not read line!");

    let line_split: Vec<&str> = line.split(',')
        .filter(|s| s.len() > 0)
        .map(|s| s.trim())
        .collect();

    Pokemon {
        number: line_split[0].to_string(),
        name: line_split[1].to_string(),
        hp: line_split[2].to_string(),
        attack: line_split[3].to_string(),
        defense: line_split[4].to_string(),
        sp_attack: line_split[5].to_string(),
        sp_defense: line_split[6].to_string(),
        speed: line_split[7].to_string(),
        total: line_split[8].to_string(),
        average: line_split[9].to_string(),
    }
}

#[derive(Debug)]
struct Pokemon {
    number: String,
    name: String,
    hp: String,
    attack: String,
    defense: String,
    sp_attack: String,
    sp_defense: String,
    speed: String,
    total: String,
    average: String,
}
