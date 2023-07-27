use std::io::{self, Write};

mod game;

fn main() {
    let mut field = game::GameField::new();

    let mut current_player = game::Player::X;

    loop {
        clear_screen();
        println!("Tic Tac Toe\n");

        let result = field.check_win();
        if result != ' ' {
            println!("{} won!", result);
            break;
        }

        println!("{}\n", field.to_string());

        print!("{} move: ", current_player.to_char());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().parse::<u32>().expect("Please enter a number");
        if field.check_move(input) {
            field.make_move(input, current_player.to_char());
        } else {
            continue;
        }

        match current_player {
            game::Player::X => {
                current_player = game::Player::O;
            }
            game::Player::O => {
                current_player = game::Player::X;
            }
        }
    }
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
