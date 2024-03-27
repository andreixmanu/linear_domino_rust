use rand::Rng;
use std::process::exit;
use crate::{EMPTY_PIECE, Piece};
use crate::autocomplete::autocomplete;

use crate::Box;
use crate::print::{print_player_debug, print_table, print_table_debug, read_string, read_int, read_char};

pub static mut MAX_ROWS: usize = 1;

const VERTICAL: usize = 2;
const HORIZONTAL: usize = 1;
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

fn first_valid_index(table: &Vec<Vec<Piece>>, row: usize) -> usize {
    if let Some(query_row) = table.get(row) {
        for (i, piece) in query_row.iter().enumerate() {
            if piece != &EMPTY_PIECE { return i; }
        }
    }
    0
}

fn last_valid_index(table: &Vec<Vec<Piece>>, row: usize) -> usize {
    if let Some(query_row) = table.get(row) {
        for (i, piece) in query_row.iter().enumerate().rev() {
            if piece != &EMPTY_PIECE { return i; }
        }
    }
    0
}

unsafe fn calculate_score(table: &Vec<Vec<Piece>>) -> usize {
    let mut score: i32 = 0;
    for i in 0..MAX_ROWS {
        for j in 0..15 {
            if table[i][j].right_box.value != -1 && table[i][j].left_box.value != -1 {
                score += table[i][j].right_box.value;
                score += table[i][j].left_box.value;
            }
        }
    }
    return score as usize;
}

fn switch_pieces(player: &mut Vec<Piece>, n: usize) {
    let temp = player[n].right_box.value;
    player[n].right_box.value = player[n].left_box.value;
    player[n].left_box.value = temp;
}

fn check_move(used_piece: Piece, table_piece: Piece, side: usize) -> bool {

    if table_piece == EMPTY_PIECE {
        println!("Table piece is empty");
        return false;
    }

    // println!("DEBUG: Checking player piece {}|{} with table piece {}|{}",
    //used_piece.left_box.value, used_piece.right_box.value,
    //table_piece.left_box.value, table_piece.right_box.value);

    if side == LEFT_SIDE {

        if table_piece.right_box.value != -1 && table_piece.left_box.value == -1{
            if used_piece.right_box.value == table_piece.right_box.value{
                return true;
            }
        }

       if used_piece.right_box.value == table_piece.left_box.value {
            return true;
        }
    } else { // right side

        if table_piece.left_box.value != -1 && table_piece.right_box.value == -1{
            if used_piece.left_box.value == table_piece.left_box.value{
                return true;
            }
        }

       if used_piece.left_box.value == table_piece.right_box.value {
            return true;
        }
    }
    false
}

unsafe fn use_piece(mut table: &mut Vec<Vec<Piece>>, mut player: &mut Vec<Piece>, choice: usize, side: usize, row: usize, orientation: usize) -> bool {
    let selected_piece: Piece = player[choice].clone();

    if orientation == HORIZONTAL {

        if side == LEFT_SIDE {
            let first_index: usize = first_valid_index(&table, row);
            if check_move(selected_piece, table[row][first_index], LEFT_SIDE) {
                table[row][first_index - 1] = selected_piece;
                player.remove(choice);
                // println!("DEBUG: Removed piece {}|{} at index {}", player[choice].left_box.value, player[choice].right_box.value, choice-1);
                return true;
            }
        } else {
            let last_index: usize = last_valid_index(&table, row);
            if check_move(selected_piece, table[row][last_index], RIGHT_SIDE) {
                table[row][last_index + 1] = selected_piece;
                player.remove(choice);
                // println!("DEBUG: Removed piece {}|{} at index {}", player[choice].left_box.value, player[choice].right_box.value, choice-1);
                return true;
            }
        }
    } else {
        if row == MAX_ROWS - 1 {
            MAX_ROWS += 1;
        }
        if side == LEFT_SIDE {
            let first_index: usize = first_valid_index(&table, row);
            if check_move(selected_piece, table[row][first_index], LEFT_SIDE) {
                table[row][first_index-1].right_box.value = selected_piece.right_box.value;
                table[row][first_index-1].left_box.value = -1;
                table[row+1][first_index-1].left_box.value = selected_piece.left_box.value;
                table[row+1][first_index-1].right_box.value = -1;
                return true;
            }
        } else {
            let last_index: usize = last_valid_index(&table, row);
            if check_move(selected_piece, table[row][last_index], RIGHT_SIDE) {
                table[row][last_index+1].left_box.value = selected_piece.left_box.value;
                table[row][last_index+1].right_box.value = -1;
                table[row+1][last_index+1].right_box.value = selected_piece.right_box.value;
                table[row+1][last_index+1].left_box.value = -1;
                return true;
            }
        }
    }
    false
}

unsafe fn singleplayer(table: &mut Vec<Vec<Piece>>, mut player: &mut Vec<Piece>) {
    let pieces = player.len();

    println!("Your pieces:");
    print_player_debug(&player);

    let selected_piece_index: usize = read_int("Select the first piece you want to play");
    let selected_piece: Piece = player[selected_piece_index - 1].clone();

    table[0][8] = selected_piece; //insert first piece
    player.remove(selected_piece_index - 1);
    println!("Printing table");
    print_table(&table);

    loop {
        println!("Your pieces: ");
        print_player_debug(&player);
        let choice_str = read_char("Select a piece to play: \
        Press 0 to quit the game or 's' to switch two pieces");

        let choice_int = choice_str as usize - '0' as usize;

        if choice_str == '0' {
            println!("Game finished: final score {}", calculate_score(&table));
            exit(0);
        } else if choice_str == 'S' || choice_str == 's' {
            let n = read_int("Which piece do you want to switch?");
            switch_pieces(player, n - 1);
            println!("Table:");
            print_table(&table);
            continue;
        } else if choice_int > pieces || choice_int < 0 || choice_int > player.len() {
            println!("Please select a valid option!");
            continue;
        }

        let mut row = 1;
        if MAX_ROWS > 1 {
            print!("On which row do you want to put it? (1 - {})", MAX_ROWS);
            row = read_int("");
        }

        /*if row >= MAX_ROWS {
            row = MAX_ROWS - 1;
        } else if row < 1 {
            row = 1;
        }
        */
        let side = read_int("Press 1 to place it on left, and 2 to place it on right:");
        let orientation = read_int("Do you want to place it horizontally (1) or vertically(2)?");

        if use_piece(table, &mut player, choice_int - 1, side, row - 1, orientation) == true {
            println!("Piece placed successfully");
        } else {
            println!("Could not place the piece. Check again your move.");
        }

        println!("Table:");
        print_table(&table);
        if player.len() == 0 {
            println!("Game ended!\
             Final Score: {}", calculate_score(&table));
            break;
        }
    }
}

pub unsafe fn main_game(mut table: Vec<Vec<Piece>>, mut player: Vec<Piece>) {
    let n_pieces = read_string("How many pieces do you want to play with?");
    let n_pieces: i32 = n_pieces.trim().parse().expect("Please type a number!");

    if n_pieces > 20 {
        println!("You can't play with more than 20 pieces!");
    } else if n_pieces < 1 {
        println!("You can't play with less than 1 piece!");
    } else {
        assign_pieces(&mut player, n_pieces)
    }

    loop {
        //let choice = read_int("Play it yourself or make computer play?");
        let choice = 1;
        if choice == 1 {
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
