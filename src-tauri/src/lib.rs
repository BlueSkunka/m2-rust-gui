use std::cmp::{Ordering, PartialEq};
use std::error::Error;
use std::fs::File;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Ord, Eq, PartialEq)]
struct Pokemon {
    name: String,
    level: u8,
    pk_type: PkType,
    experience: i32,
    gender: Gender
}

impl Pokemon {
    fn println(&self) {
        println!("{:#?}", self);
    }

    // Gain exp to pokemon
    fn gain_experience(&mut self, experience: i32) {
        self.experience += experience;

        // Si le pokemon atteinds 100 points d'xp, il monte d'un niveau
        if self.experience >= 100 {
            self.experience -= 100;
            self.level += 1;
        }
    }

    // Vérifie si un pokemon peut se reproduire avec un autre
    fn may_reproduce(&self, other: &Pokemon) -> bool {
        if self.gender == other.gender
            || self.name == other.name
            || self.pk_type != other.pk_type
            || self.level < 10
            || other.level < 10
        {
           return false;
        }

        true
    }
    
    fn from_vec_string(values: Vec<String>) -> Pokemon {
        Pokemon {
            name: values[0].clone().trim().to_string(),
            level: values[1].parse::<u8>().unwrap(),
            pk_type: PkType::from_str(values[2].trim()).unwrap(),
            experience: values[3].parse::<i32>().unwrap(),
            gender: Gender::from_str(values[4].trim()).unwrap(),
        }
    }

    fn to_array_string(&self) -> [String; 5] {
        [
            self.name.clone(),
            self.level.to_string(),
            self.pk_type.to_str().to_string(),
            self.experience.to_string(),
            self.gender.to_str().to_string(),
        ]
    }
}

impl PartialOrd for Pokemon {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.level.cmp(&other.level))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Elevage {
    pokemons: Vec<Pokemon>,
}

impl Elevage {
    // Génère un nouveau pokemon issue de la reproduction
    fn pokemon_reproduce(pokemon_a: Pokemon, pokemon_b: Pokemon) -> Result<Pokemon, String> {
        let baby: Pokemon = Pokemon {
            name: "Mystère".to_string(),
            level: 1,
            pk_type: pokemon_a.pk_type.clone(),
            experience: 1,
            gender: Gender::FEMALE,
        };

        Ok(baby)
    }

    fn read_elevage() -> Result<Elevage, String> {
        if !Path::new(ELEVAGE_FILE).exists() {
            let _ = File::create(Path::new(ELEVAGE_FILE));
        }

        let mut reader = csv::Reader::from_path("elevage.csv").unwrap();
        let mut pokemons: Vec<Pokemon> = Vec::new();

        for result in reader.deserialize() {
            let values: Vec<String> = result.unwrap();

            let pokemon: Pokemon = Pokemon::from_vec_string(values);
            pokemons.push(pokemon);
        }

        let elevage = Elevage { pokemons };

        Ok(elevage)
    }

    fn write_elevage(elevage: Elevage) -> Result<(), String> {
        let mut wtr = csv::Writer::from_path(ELEVAGE_FILE).unwrap();
        
        wtr.write_record(&["Name", "Level", "PK Type", "Experience", "Gender"]).expect("Error while writing headerts");
        
        for pokemon in elevage.pokemons {
            wtr.write_record(&pokemon.to_array_string()).expect("Error while writing pokemon");
        }
        
        wtr.flush().unwrap();
        
        Ok(())
    }
}

const ELEVAGE_FILE: &str = "elevage.csv";

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord)]
enum PkType {
    Fire,
    Water,
    Leaf,
    Ground,
    Ice
}

impl PkType {
    fn to_str(&self) -> &str {
        match self {
            PkType::Fire => {"fire"}
            PkType::Water => {"water"}
            PkType::Leaf => {"leaf"}
            PkType::Ground => {"ground"}
            PkType::Ice => {"ice"}
        }
    }

    fn from_str(input: &str) -> Result<PkType, String> {
        match input {
            "fire" => Ok(PkType::Fire),
            "water" => Ok(PkType::Water),
            "leaf" => Ok(PkType::Leaf),
            "ground" => Ok(PkType::Ground),
            "ice" => Ok(PkType::Ice),
            _ => {Err(format!("Unknown PkType {}", input))}
        }
    }
}

impl PartialOrd for PkType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord)]
enum Gender {
    MALE,
    FEMALE
}

impl Gender {
    fn to_str(&self) -> &str {
        match self {
            Gender::MALE => {"male"}
            Gender::FEMALE => {"female"}
        }
    }

    fn from_str(input: &str) -> Result<Gender, String> {
        match input {
            "male" => Ok(Gender::MALE),
            "female" => Ok(Gender::FEMALE),
            _ => {Err(format!("Unknown Gender {}", input))}
        }
    }
}

impl PartialEq for Gender {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Self::FEMALE, Self::FEMALE) | (Self::MALE, Self::MALE),
        )
    }
}

impl PartialOrd for Gender {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    published_year: String,
}

impl Book {
    fn print(&self) {
        println!("Title: {}, Author: {}, ISBN: {}, PUBLISHED_YEAR: {}", self.title, self.author, self.isbn, self.published_year);
    }

    fn from_vec_string(values: Vec<String>) -> Book {
        Book {
            title: values[0].clone().trim().to_string(),
            author: values[1].clone().trim().to_string(),
            isbn: values[2].clone().trim().to_string(),
            published_year: values[3].clone().trim().to_string(),
        }
    }

    fn to_array_string(&self) -> [String; 4] {
        [
            self.title.clone().trim().to_string(),
            self.author.clone().trim().to_string(),
            self.isbn.clone().trim().to_string(),
            self.published_year.clone().trim().to_string()
        ]
    }

    fn get_book_headers() -> [String; 4] {
        ["Title".to_string(), "Author".to_string(), "ISBN".to_string(), "Published Year".to_string()]
    }
}

// Récupération des livres
fn fetch_books() -> Result<Vec<Book>, String> {
    if !Path::new("./books.csv").exists() {
        let _ = File::create(Path::new("./books.csv"));
    }

    let mut reader = csv::Reader::from_path("./books.csv").unwrap();
    let mut books: Vec<Book> = Vec::new();

    for record in reader.deserialize() {
        let values: Vec<String> = record.unwrap();

        let book: Book = Book::from_vec_string(values);
        books.push(book);
    }

    println!("{:?}", books);

    Ok(books)
}

// Ecriture des livres
fn write_books(books: &Vec<Book>) -> Result<(), Box<dyn Error>>{
    let mut wtr = csv::Writer::from_path("./books.csv").unwrap();

    // Ajout des headers
    wtr.write_record(&Book::get_book_headers())?;

    // Ecriture des livres
    for book in books {
        wtr.write_record(&book.to_array_string()).expect("Error writing CSV record");
    }

    wtr.flush().unwrap();

    Ok(())
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn post_pokemon(name: String, pk_type: String, gender: String) -> Result<Pokemon, String> {
    let pokemon: Pokemon = Pokemon {
        name: name,
        level: 1,
        pk_type: PkType::from_str(&*pk_type).unwrap(),
        experience: 0,
        gender: Gender::from_str(&*gender).unwrap()
    };
    
    let mut elevage = Elevage::read_elevage().unwrap();

    elevage.pokemons.push(pokemon.clone());

    Elevage::write_elevage(elevage).expect("Error while writing csv file");

    Ok(pokemon)
}

#[tauri::command]
fn elevage() -> Result<Elevage, String> {
    Elevage::read_elevage()
}

#[tauri::command]
fn get_pokemon_command() -> Result<Pokemon, String> {
    get_pokemon()
}

fn get_pokemon() -> Result<Pokemon, String> {
    let pokemon: Pokemon = Pokemon {
        name: "Lucario".to_string(),
        level: 50,
        pk_type: PkType::Ground,
        experience: 50 * 100,
        gender: Gender::MALE,
    };

    Ok(pokemon)
}

#[tauri::command]
fn training_command() -> Result<Elevage, String> {
    let mut elevage: Elevage = Elevage::read_elevage().unwrap();

    for pokemon in elevage.pokemons.iter_mut() {
        pokemon.gain_experience(25);
    }

    Elevage::write_elevage(elevage.clone()).expect("Error while writing CSV record");

    Ok(elevage)
}

#[tauri::command]
fn sort_level_command() -> Result<Elevage, String> {
    let mut elevage: Elevage = Elevage::read_elevage()?;

    elevage.pokemons.sort();
    elevage.pokemons.reverse();

    Ok(elevage)
}

#[tauri::command]
fn pokemon_reproduce_command(pokemon_a: Pokemon, pokemon_b: Pokemon) -> Result<Elevage, String> {
    if (pokemon_a.may_reproduce(&pokemon_b)) {
        let pokemon = Elevage::pokemon_reproduce(pokemon_a.clone(), pokemon_b.clone())?;

        let mut elevage: Elevage = Elevage::read_elevage()?;

        elevage.pokemons.push(pokemon);

        Elevage::write_elevage(elevage.clone()).expect("Error while writing CSV record");

        return Ok(elevage)
    }

    Err("Pokemon can't be reproduced together.".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, elevage, post_pokemon, training_command, sort_level_command, pokemon_reproduce_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
