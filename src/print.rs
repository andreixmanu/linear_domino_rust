use crate::game::MAX_ROWS;
use crate::Piece;
use std::io;

#[allow(dead_code)]
pub unsafe fn print_table_debug(table: &Vec<Vec<Piece>>){
    for i in 0..MAX_ROWS {
        for j in 0..15 {
            print!("[{}|{}] ", table[i][j].left_box.value, table[i][j].right_box.value);
        }
        println!();
    }
}

pub unsafe fn print_table(table: &Vec<Vec<Piece>>){
    for i in 0..MAX_ROWS {
        for j in 0..15 {
            if table[i][j].left_box.value != -1 && table[i][j].right_box.value != -1 {
                print!("[{}|{}] ", table[i][j].left_box.value, table[i][j].right_box.value);
            } else if table[i][j].left_box.value != -1 && table[i][j].right_box.value == -1{
                print!("[ {} ] ", table[i][j].left_box.value);
            } else if table[i][j].left_box.value == -1 && table[i][j].right_box.value != -1{
                print!("[ {} ] ", table[i][j].right_box.value);
            } else {
                print!("      ");
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

pub fn read_char(prompt: &str) -> char {
    println!("{}", prompt);

    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input){
            Ok(_) => {
                if let Some(ch) = input.chars().next(){
                    return ch;
                } else {
                    println!("Error");
                    continue;
                }
            }
            Err(_error) =>{
                println!("Some error occurred, please repeat");
                continue;
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
