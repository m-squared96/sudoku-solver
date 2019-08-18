/* Sudoku solver
 * Utilises the backtracking algorithm to solve a sudoku puzzle.
 * The code here is based on the Python code written by @mjancen on GH: https://github.com/mjancen/sudoku-solver
 * */

//TODO: Commentary and documentation
//TODO: Generate and output/random user IO of puzzle boards
//TODO: CLI app functionality

use std::env;
use std::fs;
use std::io;
use std::io::{Write, BufWriter};
use rand::Rng;

fn board_gen(difficulty: &str) -> Vec<Vec<u32>> {
    //Returns pre-defined testing board

    println!("\nGenerating board with a difficulty of {} ...", difficulty);

    let row1: Vec<u32> = vec![0,0,0,0,0,0,0,0,0];
    let row2: Vec<u32> = vec![0,0,0,0,0,0,0,0,0];
    let row3: Vec<u32> = vec![0,0,0,0,0,0,0,0,0];
    let row4: Vec<u32> = vec![0,0,0,0,0,0,0,0,0];
    let row5: Vec<u32> = vec![0,0,0,0,0,0,0,0,0];
    let row6: Vec<u32> = vec![0,0,0,0,0,0,0,0,0];
    let row7: Vec<u32> = vec![0,0,0,0,0,0,0,0,0];
    let row8: Vec<u32> = vec![0,0,0,0,0,0,0,0,0];
    let row9: Vec<u32> = vec![0,0,0,0,0,0,0,0,0];
    
    let mut board: Vec<Vec<u32>> = vec![row1, row2, row3, row4, row5, row6, row7, row8, row9];
    let mut clue_count: u32 = 0;
    let init_seed: u32 = 20;

    if let Some(num_clues) = clue_calculator(difficulty) {
        println!("Num clues: {}", num_clues);
        while clue_count < init_seed {
            let r: usize = rand::thread_rng().gen_range(0, 9);
            let c: usize = rand::thread_rng().gen_range(0, 9);
            let num_for_cell: u32 = rand::thread_rng().gen_range(1, 10);
            if cell_is_empty(&board, &r, &c) && move_is_valid(&board, &num_for_cell, &r, &c) {
                board = replace_value(board, num_for_cell, r, c);        
                clue_count += 1;
            }
        }

        match solve_backtracking(board) {
            None => {
                println!("Error in generating board");
                let final_board: Vec<Vec<u32>> = vec![vec![1]];
                return final_board
            },

            Some(full_board) => {
                let mut final_board = full_board.clone();
                clue_count = 81;
                while clue_count > num_clues {
                    let rand_r_rmv: usize = rand::thread_rng().gen_range(0, 9);
                    let rand_c_rmv: usize = rand::thread_rng().gen_range(0, 9);
                    let (rot_r_rmv, rot_c_rmv) = rotational_symmetry_pair(&rand_r_rmv, &rand_c_rmv);
                    if !cell_is_empty(&final_board, &rand_r_rmv, &rand_c_rmv) && !cell_is_empty(&final_board, &rot_r_rmv, &rot_c_rmv) {
                        for (&r_rmv, &c_rmv) in vec![rand_r_rmv, rot_r_rmv].iter().zip(vec![rand_c_rmv, rot_c_rmv].iter()) {
                            final_board = replace_value(final_board, 0, r_rmv, c_rmv);
                            clue_count -= 1;
                        }
                    }
                }
                let mut final_clue_count: u32 = 0;
                for row in 0..9 {
                    for col in 0..9 {
                        if !cell_is_empty(&final_board, &row, &col) {
                            final_clue_count += 1;
                        }
                    }
                }
                println!("Final clue count: {}", final_clue_count);
                return final_board
            }
        }
    }

    return vec![vec![1]]

}

fn rotational_symmetry_pair(row: &usize, col: &usize) -> (usize, usize) {
    /*Accepts a (row, col) pair and returns the corresponding pair under rotational symmetry
     *
     * */
    let rot_row : usize = 8 - *row;
    let rot_col : usize = 8 - *col;

    return (rot_row, rot_col)
}

fn clue_calculator(rating: &str) -> Option<u32> {
    /*Returns the required number of clues based on the difficulty rating
     *Difficulty rating (R) is between 0 and 1, with the number of clues (C) given by;
     *  C(R) = round(-63R + 80)
     * */ 

    let allowed_values: Vec<&str> = vec!["easy", "intermediate", "hard", "expert"];

    if allowed_values.contains(&rating) {
        match rating {
            "easy" => return Some(29),
            "intermediate" => return Some(25),
            "hard" => return Some(21),
            "expert" => return Some(17),
            _ => return None,
        }
    }

    None
}

fn show_board(board: &Vec<Vec<u32>>) {
    //Prints a nicely-formatted sudoku board, showing boxes and borders
    let display_board = board;
    let mut row_count = 0;
    //println!("{}[2J", 27 as char);
    println!("-------------------------");
    for row in display_board {
        println!("| {} {} {} | {} {} {} | {} {} {} |", 
                  row[0], row[1], row[2],
                  row[3], row[4], row[5],
                  row[6], row[7], row[8]);
        row_count += 1;
        
        if row_count%3 == 0{
            println!("-------------------------");
        }
    }
}

fn corresponding_box(row: &usize, col: &usize) -> u16 {
    //Returns the corresponding box number of a cell
    let board_ref: Vec<Vec<u16>> = vec![
        vec![1,1,1,2,2,2,3,3,3],
        vec![1,1,1,2,2,2,3,3,3],
        vec![1,1,1,2,2,2,3,3,3],
        vec![4,4,4,5,5,5,6,6,6],
        vec![4,4,4,5,5,5,6,6,6],
        vec![4,4,4,5,5,5,6,6,6],
        vec![7,7,7,8,8,8,9,9,9],
        vec![7,7,7,8,8,8,9,9,9],
        vec![7,7,7,8,8,8,9,9,9]];

    board_ref[*row][*col]
}

fn view_box(box_num: &u16, board: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    /*Returns the data in a given box. This data can then be checked for
     *the occurrence of a particular value in the box
     * */

    //These four variables are used to define the box
    let mut row_low: usize = 0;
    let mut row_high: usize = 0;
    let mut col_low: usize = 0;
    let mut col_high: usize = 0;

    //Each box has its corresponding row and col combinations
    match box_num {
        1 => {row_low = 0;
              row_high = 3;
              col_low = 0;
              col_high = 3;
              },
        2 => {row_low = 0;
              row_high = 3;
              col_low = 3;
              col_high = 6;
              },
        3 => {row_low = 0;
              row_high = 3;
              col_low = 6;
              col_high = 9;
              },
        4 => {row_low = 3;
              row_high = 6;
              col_low = 0;
              col_high = 3;
              },
        5 => {row_low = 3;
              row_high = 6;
              col_low = 3;
              col_high = 6;
              },
        6 => {row_low = 3;
              row_high = 6;
              col_low = 6;
              col_high = 9;
              },
        7 => {row_low = 6;
              row_high = 9;
              col_low = 0;
              col_high = 3;
              },
        8 => {row_low = 6;
              row_high = 9;
              col_low = 3;
              col_high = 6;
              },
        9 => {row_low = 6;
              row_high = 9;
              col_low = 6;
              col_high = 9;
              },
        _ => {println!("OB");}
    }

    //Save and return the data in a given box and return it
    let mut result_box: Vec<Vec<u32>> = Vec::new();
    for row in board[row_low..row_high].to_vec() {
        result_box.push(row[col_low..col_high].to_vec());
    }
    result_box
}

/*fn show_box(board_box: &Vec<Vec<u32>>) {
    //Prints the data in a box
    println!("------");
    for row in board_box {
        println!("{} {} {}", row[0], row[1], row[2]);
    }
    println!("------");
}*/

fn cell_is_empty(board: &Vec<Vec<u32>>, row: &usize, col: &usize) -> bool {
    //Returns whether or not a cell is empty
    board[*row].to_vec()[*col] == 0
}

fn board_is_full(board: &Vec<Vec<u32>>) -> bool {
    /*Returns whether or not a board is full. Ie, checks the board
     *for the occurrence of a zero (0).
     * */
    for row_num in 0..9 {
        for col_num in 0..9 {
            if cell_is_empty(board, &row_num, &col_num) {
                return false
            }
        }
    }

    return true
}

fn next_unassigned_cell(board: &Vec<Vec<u32>>) -> Option<(usize, usize)> {
    /*Returns the location of the next unassigned cell.
     *Checks rowise and then columnwise.
     */
    for row_num in 0..9 {
        for col_num in 0..9 {
            if cell_is_empty(board, &row_num, &col_num) {
                return Some((row_num, col_num))
            }
        }
    }

    return None
}

fn num_in_row(board: &Vec<Vec<u32>>, num: &u32, row: &usize) -> bool {
    //Checks for occurrence of num in row
    return board[*row].to_vec().contains(num)
}

fn num_in_col(board: &Vec<Vec<u32>>, num: &u32, col: &usize) -> bool {
    //Checks for occurrence of num in col (trickier)
    for row in 0..9 {
        if board[row].to_vec()[*col] == *num {
            return true
        }
    }

    return false
}

fn num_in_box(board: &Vec<Vec<u32>>, num: &u32, box_num: &u16) -> bool {
    //Checks for occurrence of num in box
    let board_box: Vec<Vec<u32>> = view_box(box_num, board);

    for row in 0..3 {
        if board_box[row].to_vec().contains(num) {
            return true
        }
    }

    return false
}

fn move_is_valid(board: &Vec<Vec<u32>>, num: &u32, row: &usize, col: &usize) -> bool {
    //Returns true if no occurrence of num found in row, col or box.
    return !num_in_row(board, num, row) && !num_in_col(board, num, col) 
    && !num_in_box(board, num, &corresponding_box(row, col))
}

fn replace_value(mut board: Vec<Vec<u32>>, num: u32, row: usize, col: usize) -> Vec<Vec<u32>> {
    //Replaces the value of a cell and returns the modified board
    let mut modified_row: Vec<u32> = board[row].to_vec().clone();
    modified_row.remove(col);
    modified_row.insert(col, num);

    board.remove(row);
    board.insert(row, modified_row);
    return board
}

fn gen_possible_boards(board: &Vec<Vec<u32>>, row: usize, col: usize) -> Vec<Vec<Vec<u32>>> {
    /*Returns a vector of possible boards. If there are N valid moves given the next unassigned
     *cell, then a vector of N boards is returned.
     * */
    let mut next_boards = Vec::new();

    for num in 1..10 {
        if move_is_valid(&board, &num, &row, &col) {
            let new_board = board.clone();
            next_boards.push(replace_value(new_board, num, row, col));
        }
    }

    return next_boards
}

fn solution(final_board: &Vec<Vec<u32>>) {
    //Prints final solution
    println!("Final solution:");
    show_board(final_board);
}

fn no_solution() {
    println!("No solution could be found");
}

fn solve_backtracking(init_board: Vec<Vec<u32>>) -> Option<Vec<Vec<u32>>> {
    /*Given the initial board, begins iterating on possible board
     *solutions, until the board is full or possible moves are
     *exhausted.
     * */
    let mut solution_stack: Vec<Vec<Vec<u32>>> = Vec::new();
    solution_stack.push(init_board);

    while solution_stack.len() > 0 {
        let board = solution_stack.pop().unwrap();

        if board_is_full(&board) {
            return Some(board);
        }
        
        let nuc = next_unassigned_cell(&board);
        match nuc {
            None => {return None},
            Some(next_tup) => {
                let next_boards = gen_possible_boards(&board, next_tup.0, next_tup.1);
                for b in next_boards {
                    solution_stack.push(b);
                }
            }
        }
    }
    return None
}

fn read_from_file(args: Vec<String>) -> Option<Vec<Vec<u32>>> {
    /*Handles the case where the "f" flag is specified at launch. Checks if filepath
     *also specified. If so, read in contents of file. If file parse-able into puzzle
     *and filepath specified, returns Some(Vec). If not, returns None.

      TODO: Check if filepath specified and valid
      TODO: Read in file and attempt to parse
      TODO: Handle above cases appropriately
     * */

    println!("f flag");
    let mut filepath: Option<String> = Some(String::new());

    let mut row1: Vec<u32> = Vec::new();       
    let mut row2: Vec<u32> = Vec::new();
    let mut row3: Vec<u32> = Vec::new();
    let mut row4: Vec<u32> = Vec::new();
    let mut row5: Vec<u32> = Vec::new();
    let mut row6: Vec<u32> = Vec::new();
    let mut row7: Vec<u32> = Vec::new();
    let mut row8: Vec<u32> = Vec::new();
    let mut row9: Vec<u32> = Vec::new();

    if args.len() > 2 {
        filepath = Some(String::from(&args[2]));
        let contents = fs::read_to_string(filepath.unwrap())
            .expect("Something went wrong with the file");
        let mut char_count: u32 = 0;

        for x in contents.chars() {
            if x != '\n' {
                char_count += 1;

                if char_count >= 1 && char_count <= 9 {
                    row1.push(x.to_digit(10).unwrap());
                }
                else if char_count >= 10 && char_count <= 18 {
                    row2.push(x.to_digit(10).unwrap());
                }
                else if char_count >= 19 && char_count <= 27 {
                    row3.push(x.to_digit(10).unwrap());
                }
                else if char_count >= 28 && char_count <= 36 {
                    row4.push(x.to_digit(10).unwrap());
                }
                else if char_count >= 37 && char_count <= 45 {
                    row5.push(x.to_digit(10).unwrap());
                }
                else if char_count >= 46 && char_count <= 54 {
                    row6.push(x.to_digit(10).unwrap());
                }
                else if char_count >= 55 && char_count <= 63 {
                    row7.push(x.to_digit(10).unwrap());
                }
                else if char_count >= 64 && char_count <= 72 {
                    row8.push(x.to_digit(10).unwrap());
                }
                else if char_count >= 73 && char_count <= 81 {
                    row9.push(x.to_digit(10).unwrap());
                }
            }
        }
        let board: Vec<Vec<u32>> = vec![row1, row2, row3,
                                        row4, row5, row6,
                                        row7, row8, row9];
        return Some(board)
    }
    
    println!("No file specified; no results to show");
    return None
}

fn random_gen_handler(args: &Vec<String>) -> Vec<Vec<u32>> {
    //TODO: Parse arguments and return random board of difficulty specified. If
    //  possible, make general enough to handle both "r" and "g" flags

    let mut difficulty: &str = "intermediate";
    if args.len() > 2 {
        difficulty = &args[2];
    }
    board_gen(difficulty)
}

fn output_puzzle(args: Vec<String>) -> Vec<Vec<u32>> {
    /*Handles the case where the "g" flag is specified at launch. Checks if filepath
     *also specified. If so, output puzzle to filepath. If not, output file to current
     *directory.

      TODO: Check if filepath specified and valid
      TODO: File output
     * */

    let board: Vec<Vec<u32>> = random_gen_handler(&args);
    let mut filepath: String = String::from("puzzle_outputs/puzzle.txt");
    if args.len() > 3 {
        filepath = String::from(&args[3]);
    }

    let mut output_file = fs::File::create(filepath.as_str()).unwrap();
    let mut writer = BufWriter::new(&output_file);

    for row in 0..9 {
        let mut line_string: String = String::new();
        for val in board[row].iter() {
            if let Some(c) = std::char::from_u32(*val) {
                line_string.push(c);
            }
        }
        println!("{:?}", line_string);
        write!(&mut writer, "{}", line_string);
    }  

    return board
}

fn manual_input() -> Option<Vec<Vec<u32>>> {
    /*Handles case where the "m" flag is specified at launch. Guides user through manual
     *input of sudoku puzzle. If valid puzzle at the end, return Some(Vec). Else, return None.
     * */

    let mut board : Vec<Vec<u32>> = Vec::new();

    println!("\nManual entry mode. Enter the digits on each ROW of the puzzle. \nEmpty cells should be represented by zeroes (0).");

    let mut row_counter: u16 = 1;
    while row_counter <= 9 {
        println!("\nEnter the cell values for row {}", row_counter);
        let mut entry : String = String::new();
        io::stdin().read_line(&mut entry)
            .expect("Failed to read line");

        let mut row_vec: Vec<u32> = Vec::new();

        // .trim() is necessary as the input string contains a space at the end by default
        if entry.trim().len() == 9 { // should be nine values in a row of Sudoku
            for c in entry.trim().chars() {
                match c.to_digit(10) { // Attempts to convert c to a number with base 10
                    Some(x) => row_vec.push(x),                                
                    None => println!("Error: Digit not read"),
                }
            }
        }

        //println!("{:?}", row_vec);
        // If 9 digits successfully cast to integers and pushed to vector, then push to board
        if row_vec.len() == 9 {
            row_counter += 1;
            board.push(row_vec);
        }
        // Else, print error message and the loop will restart without an increment increase
        else {
            println!("\nError in parsing entry. Please try again");
        }
    }

    Some(board)
}

fn arg_handler(args: Vec<String>) -> Option<Vec<Vec<u32>>> {
    /*Handles command line arguments when program is called. Argument options are:
     * 1. f [FILE_PATH] -> Read in puzzle from FILE_PATH and solve
     * 2. r [DIFFICULTY]-> Generate random puzzle of DIFFICULTY and solve
     * 3. g [DIFFICULTY] [FILE_PATH] -> Generate random puzzle of DIFFICULTY and output to FILE_PATH
     * 4. m -> Manual puzzle input from user and solve
     *
     * args is a vec of the form [filepath to executable (unwanted), flag [, file path]]
     * TODO: Argument-specific functions for each option. Current implementation too messy.
     * */
    //println!("{:?}", args);
    let mut command_flag = String::new();

    if args.len() > 1 {
        command_flag = String::from(&args[1]);
    }
    else {
        command_flag = String::from("no flag");
    }

    let mut board: Option<Vec<Vec<u32>>> = Some(Vec::new());
    match command_flag.as_ref() {
        "r" => {board = Some(random_gen_handler(&args))},
        "f" => {board = read_from_file(args);},
        // "g" => {board = Some(output_puzzle(args))},
        "m" => {board = manual_input();},
        _ => {board = None}
    }

    board
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let maybe_board: Option<Vec<Vec<u32>>> = arg_handler(args);

    match maybe_board {
        Some(board) => {
            println!("\nGiven the initial puzzle:");
            show_board(&board);
            println!("Solving...");
            let puzzle_solution = solve_backtracking(board);

            match puzzle_solution {
                None => no_solution(),
                Some(board) => solution(&board)
            }
        },
        None => {println!("No board");}
    }

}
