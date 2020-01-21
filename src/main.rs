extern crate minimax;
extern crate game;

use minimax::{Evaluable, Minimax, Player as MinimaxPlayer};
use game::{ConnectFour};
use std::io::{self, prelude::*, Write};

fn main()
{
    let game = ConnectFour::new();
    let mut ai: Minimax<ConnectFour> = Minimax::new(game);

    println!(" 1  2  3  4  5  6  7  ");
    ai.get_info().print();
    let winner = loop
    {
        let stdin = io::stdin();
        let mut valid_move = false;
        while !valid_move
        {
            // handle input
            print!("Please enter your move: ");
            std::io::stdout().flush().expect("Oops, An Error Occurred");
            let line = stdin.lock().lines().next().unwrap();
            let player_move_raw = line.unwrap().parse::<i8>();
            // play the actual move
            if let Ok(player_move) = player_move_raw
            {
                valid_move = match player_move
                {
                    1 => ai.get_info().play_move(0),
                    2 => ai.get_info().play_move(1),
                    3 => ai.get_info().play_move(2),
                    4 => ai.get_info().play_move(3),
                    5 => ai.get_info().play_move(4),
                    6 => ai.get_info().play_move(5),
                    7 => ai.get_info().play_move(6),
                    _ => false,
                };
            }
            if !valid_move
            {
                println!("That is not a valid move!!!");
            }
        }
        ai.get_info().print();

        if ai.get_info().is_terminal_state()
        {
            break ai.get_info().get_winner();
        }

        ai = ai.alpha_beta(5, MinimaxPlayer::MINIMIZING).unwrap();
        println!("Computer Evaluation Value: {}", ai.get_positional_estimate().unwrap());
        println!(" 1  2  3  4  5  6  7  ");
        ai.get_info().print();

        if ai.get_info().is_terminal_state()
        {
            break ai.get_info().get_winner();
        }
    };
    println!("The player \" {} \" has won the game", winner);
}
