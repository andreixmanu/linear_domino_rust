use rand::Rng;
use std::io;
use std::process::exit;
use crate::{EMPTY_PIECE, Piece, print};
use crate::autocomplete::autocomplete;

use crate::Box;
use crate::print::{print_player_debug, print_table, print_table_debug, read_string, read_int};

const LEFT_SIDE: usize = 1;
const RIGHT_SIDE: usize = 2;

fn assign_pieces(player: &mut Vec<Piece>, n_pieces: i32) {
    let possible_pieces: Vec<Vec<i32>> = vec![
        vec![1, 1], vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6],
        vec![2, 2], vec![2, 3], vec![2, 4], vec![2, 5], vec![2, 6],
        vec![3, 3], vec![3, 4], vec![3, 5], vec![3, 6],
        vec![4, 4], vec![4, 5], vec![4, 6],
        vec![5, 5], vec![5, 6],
        vec![6, 6],
    ];

    for _ in player.len()..n_pieces as usize {
        let mut piece = Piece {
            left_box: Box { value: 0 },
            right_box: Box { value: 0 },
        };
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..possible_pieces.len());
        piece.left_box.value = possible_pieces[random_index][0];
        piece.right_box.value = possible_pieces[random_index][1];
        player.push(piece);
    }
}

fn first_valid_index(table: &Vec<Vec<Piece>>) -> isize {
    if let Some(first_row) = table.get(0) {
        for (i, piece) in first_row.iter().enumerate() {
            if piece != &EMPTY_PIECE { return i as isize; }
        }
    }
    -1
}

fn last_valid_index(table: &Vec<Vec<Piece>>) -> i8 {
    if let Some(first_row) = table.get(0) {
        for (i, piece) in first_row.iter().enumerate().rev() {
            if piece != &EMPTY_PIECE { return i as i8; }
        }
    }
    -1
}

fn check_move(used_piece: Piece, table_piece: Piece, side: usize) -> bool{

    if side == LEFT_SIDE{
        if used_piece.right_box.value == table_piece.left_box.value{
            return true
        }
    } else {
        if used_piece.left_box.value == table_piece.right_box.value{
            return true
        }
    }
    false
}

fn use_piece(mut table: &mut Vec<Vec<Piece>>, mut player: &mut Vec<Piece>, choice: usize, side: usize) -> bool {

    let selected_piece: Piece = player[choice];

    if side == LEFT_SIDE {

        let first_index : isize = first_valid_index(&table);
        //todo check index validity
        if check_move(table[0][first_index as usize], selected_piece, LEFT_SIDE){
            table[0][(first_index-1) as usize] = selected_piece;
            player.remove(choice);
            println!("DEBUG: Removed piece at index {}", choice-1);
            return true;
        }

    } else {

        let last_index : i8 = last_valid_index(&table);
        if check_move(table[0][last_index as usize], selected_piece, RIGHT_SIDE){
            table[0][(last_index+1) as usize] = selected_piece;
            player.remove(choice);
            println!("DEBUG: Removed piece at index {}", choice-1);
            return true;
        }
    }
    false
}

fn singleplayer(table: &mut Vec<Vec<Piece>>, mut player: &mut Vec<Piece>) {
    let pieces = player.len();

    let selected_piece_index: usize = read_int("Select the first piece you want to play");
    let selected_piece: Piece = player[selected_piece_index-1].clone();

    table[0][8] = selected_piece; //insert first piece
    player.remove(selected_piece_index - 1);
    println!("Printing table");
    print_table_debug(&table);

    loop {

        print_player_debug(&player);
        let choice = read_int("Select a piece to play: \
        Press 0 to quit the game");

        if choice == 0 {
            exit(0);
        } else if choice > pieces {
            println!("Please select a valid option!");
            continue;
        } else if choice < 0 {
            println!("Please select a valid option!");
            continue;
        } else if choice > player.len(){
            println!("Please write a lower number (1 - {})", player.len());
        }

        let side = read_int("Press 1 to place it on left, and 2 to place it on right:");

        if side == 1 {
            use_piece(table, &mut player, choice - 1, LEFT_SIDE);
        } else if side == 2 {
            use_piece(table, &mut player, choice - 1, RIGHT_SIDE);
        }
        print_table_debug(table);
    }
}

pub fn main_game(mut table: Vec<Vec<Piece>>, mut player: Vec<Piece>) {

    let n_pieces = read_string("How many pieces do you want to play with?");
    let n_pieces: i32 = n_pieces.trim().parse().expect("Please type a number!");

    if n_pieces > 20 {
        println!("You can't play with more than 20 pieces!");
    } else if n_pieces < 1 {
        println!("You can't play with less than 1 piece!");
    } else {
        assign_pieces(&mut player, n_pieces)
    }

    println!("Your pieces: ");
    print_player_debug(&player);

    loop {
        let choice = read_int("Play it yourself or make computer play?");

        if choice == LEFT_SIDE {
            singleplayer(&mut table, &mut player);
            break;
        } else if choice == RIGHT_SIDE {
            autocomplete(table, &mut player);
            break;
        } else {
            println!("Please select a valid option!");
        }
    }
}