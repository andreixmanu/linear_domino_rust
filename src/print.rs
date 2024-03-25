use crate::{Piece};
use std::io;

pub fn print_table_debug(table: &Vec<Vec<Piece>>){
    for i in 0..1 {
        for j in 0..15 {
            print!("[{}|{}] ", table[i][j].left_box.value, table[i][j].right_box.value);
        }
        println!();
    }
}

//not working
pub fn print_table(table: &Vec<Vec<Piece>>){
    for i in 0..3 {
        for j in 0..15 {
            if table[i][j].left_box.value != -1 && table[i][j].right_box.value != -1 {
                print!("[{}|{}] ", table[i][j].left_box.value, table[i][j].right_box.value);
            }
        }
        println!();
    }
}

pub fn print_player_debug(player: &Vec<Piece>){
    for i in 0..player.len() {
        println!("{}: [{}|{}] ", i+1, player[i].left_box.value, player[i].right_box.value);
    }
    println!();
}

pub fn print_rules(){
    println!("Rules");
}

pub fn read_string(prompt: &str) -> String {
    loop {
        println!("{}", prompt);

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let cleaned_input = input.trim().to_string();
                if !cleaned_input.is_empty() {
                    return cleaned_input;
                } else {
                    println!("Input non valido. Riprova.");
                }
            }
            Err(err) => {
                eprintln!("Errore durante la lettura dell'input: {}", err);
            }
        }
    }
}

pub fn read_int(prompt: &str) -> usize {
    loop {
        println!("{}", prompt);

        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let cleaned_input : usize = input.trim().parse().expect("Puzza");
                    return cleaned_input;
            }
            Err(err) => {
                eprintln!("Errore durante la lettura dell'input: {}", err);
            }
        }
    }
}
