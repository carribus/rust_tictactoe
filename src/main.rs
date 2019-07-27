use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn read_line() -> String {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
}

struct Board {
    spots: [u8; 9]
}

impl Board {
    pub fn new() -> Board {
        Board {
            spots: [0; 9]
        }
    }

    pub fn set_piece(&mut self, player_num: u8, position: usize) -> bool{
        if self.spots[position] == 0 {
            self.spots[position] = player_num+1;
            true
        } else {
            false
        }
    }

    pub fn get_winner(&self) -> u8 {
        let winning_arrangements = [
            // diagonals
            [0, 4, 8], [1, 4, 7], [2, 4, 6],
            // horizontals
            [0, 1, 2], [3, 4, 5], [6, 7, 8],
            // verticals
            [0, 3, 6], [1, 4, 7], [2, 5, 8]
        ];
        let mut winner = 0;

        for arrangement in winning_arrangements.iter() {
            let player = self.spots[arrangement[0]];
            if player != 0 && player == self.spots[arrangement[1] as usize] && player == self.spots[arrangement[2] as usize] {
                winner = player;
                break;
            }
        }

        winner
    }

    pub fn has_space(&self) -> bool {
        self.spots.iter().any(|&x| x == 0)
    }
}

fn print_board(board: &Board) {
    eprintln!("{:?}", board.spots);
    const LINE: &str = "-----------";
    let piece_symbol = |x: &u8| {
        match x {
            0 => ' ',
            1 => 'X',
            2 => 'O',
            _ => panic!(),
        }
    };

    for (index, x) in board.spots.iter().enumerate() {
        match index {
            index if index % 3 == 2 => println!(" {}\n{}", piece_symbol(x), LINE),
            _ => print!(" {} |", piece_symbol(x)),
        }
    }
}

fn prompt_player(player_num: u8) -> Option<u8> {
    println!("[Player {}] Place your piece (0-8 are valid choices): ", player_num+1);
    let position = parse_input!(read_line(), u8);
    match position {
        0...8 => Some(position),
        _ => None
    }
}

fn main() {
    let mut board = Board::new();
    let mut playing = true;
    let mut current_player = 0;

    while playing == true {
        print_board(&board);
        if let Some(position) = prompt_player(current_player) {
            if board.set_piece(current_player, position as usize) {
                let winner = board.get_winner();
                match winner {
                    1 | 2 => {
                        println!("Player {} wins! Starting new game...\n", winner);
                        board = Board::new();
                        current_player = 0;
                    }
                    _ => {
                        if board.has_space() {
                            current_player ^= 1;
                        } else {
                            println!("Game is a draw!\n");
                            board = Board::new();
                            current_player = 0;
                        }
                    }
                }
            }
        } else {
            playing = false;
        }

    }
}
