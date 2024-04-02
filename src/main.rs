mod print;
mod game;

#[derive(PartialEq, Clone, Copy)]
pub struct Box {
    value: i32,
}

#[derive(PartialEq, Clone, Copy)]
pub struct Piece {
    left_box: Box,
    right_box: Box,
}

const EMPTY_BOX: Box = Box { value: -1 };
const EMPTY_PIECE: Piece = Piece {
    left_box: EMPTY_BOX,
    right_box: EMPTY_BOX,
};

fn main() {

    let mut table: Vec<Vec<Piece>> = Vec::with_capacity(15);
    let player: Vec<Piece> = vec![];

    // Inizializzazione della matrice con valori di esempio
    for _ in 0..15 {
        let mut row: Vec<Piece> = Vec::with_capacity(15);
        for _ in 0..15 {
            let piece = EMPTY_PIECE;
            row.push(piece);
        }
        table.push(row);
    }

    println!("Welcome to Linear Domino!\
    \n\nMenu:\
    \n\t1. Start game\
    \n\t2. View Rules\
    \n\t3. Exit");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read line");
    //convertiamo la stringa in un numero
    let choice: u32 = choice.trim().parse().expect("Please type a number!");

    loop{
        match choice {
            1 => unsafe {
                game::main_game(table, player);
                break;
            },
            2 => {
                print::print_rules();
                break;
            },
            3 => {
                println!("Exiting...");
                break;
            },
            _ => {
                println!("Please select a valid option!");
                break;
            },
        }
    }


}
