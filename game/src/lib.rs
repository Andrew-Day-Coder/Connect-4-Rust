#![allow(dead_code)]
#![allow(clippy::needless_return)]
#![allow(clippy::new_without_default)]

extern crate minimax;
use minimax::Evaluable;

#[derive(Copy, Clone, PartialEq)]
pub enum Player
{
    Red,
    Black,
    Empty,
}

impl std::fmt::Display for Player
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            Player::Red => write!(f,"R"),
            Player::Black => write!(f,"B"),
            Player::Empty => write!(f,"+"),
        }
    }
}
impl Player
{
    fn get_oppenent(self) -> Player
    {
        match self
        {
            Player::Red => Player::Black,
            Player::Black => Player::Red,
            Player::Empty => Player::Empty,
        }
    }
}
pub struct ConnectFour
{
    // Note To future me: If we index out of bounds flip the 7 and the 6
    state: [[Player; 7]; 6],
    turn: Player,
}


impl ConnectFour
{
    pub fn new() -> ConnectFour
    {
        ConnectFour{state: [[Player::Empty; 7]; 6], turn: Player::Red}
    }
    pub fn new_from_existing(cf: &ConnectFour) -> Self
    {
        ConnectFour{state: cf.state, turn: cf.turn}
    }
    pub fn get_winner(&self) -> Player
    {
        let players: Vec<Player> = vec![Player::Red, Player::Black];
        for player in players
        {
            // vertical test
            for row in 0..self.state.len() - 3
            {
                for col in 0..self.state[row].len()
                {
                    if  self.state[row    ][col] == player &&
                        self.state[row + 1][col] == player &&
                        self.state[row + 2][col] == player &&
                        self.state[row + 3][col] == player
                    {
                        return player;
                    }
                }
            }
            // horizontal test
            for row in 0..self.state.len()
            {
                for col in 0..self.state[row].len() - 3
                {
                    if  self.state[row][col    ] == player &&
                        self.state[row][col + 1] == player &&
                        self.state[row][col + 2] == player &&
                        self.state[row][col + 3] == player
                    {
                        return player;
                    }
                }
            }
            // diagonal test
            for row in 0..self.state.len() - 3
            {
                for col in 0..self.state[row].len() - 3
                {
                       // a diagonal
                    if self.state[row    ][col    ] == player &&
                       self.state[row + 1][col + 1] == player &&
                       self.state[row + 2][col + 2] == player &&
                       self.state[row + 3][col + 3] == player
                    {
                        return player;
                    }
                }
            }
            // other diagonal direction
            for row in 0..self.state.len() - 3
            {
                for col in 3..self.state[row].len()
                {
                    if  self.state[row    ][col    ] == player &&
                        self.state[row + 1][col - 1] == player &&
                        self.state[row + 2][col - 2] == player &&
                        self.state[row + 3][col - 3] == player
                    {
                        return player;
                    }
                }
            }
        }
        return Player::Empty;
    }
    pub fn play_move(&mut self, col: usize) -> bool
    {
        for row in (0..self.state.len()).rev()
        {
            if self.state[row][col] == Player::Empty
            {
                self.state[row][col] = self.turn;
                self.turn = self.turn.get_oppenent();
                return true;
            }
        }
        return false;
    }
    pub fn is_filled(&self) -> bool
    {
        for row in 0..self.state.len()
        {
            for col in 0..self.state[row].len()
            {
                if self.state[row][col] == Player::Empty
                {
                    return false;
                }
            }
        }
        return true;
    }
    pub fn print(&self)
    {
        for row in 0..self.state.len()
        {
            for col in 0..self.state[0].len()
            {
                print!(" {} ", self.state[row][col]);
            }
            println!();
        }
        print!("\n\n\n\n\n\n");
    }
}
impl Evaluable<ConnectFour> for ConnectFour
{
    type EvalOutput = i8;

    fn get_children(&self) -> Vec<ConnectFour>
    {
        let mut children: Vec<ConnectFour> = Vec::new();

        for col in 0..self.state[0].len()
        {
            let mut new_child = ConnectFour::new_from_existing(self);
            let is_legal_move = new_child.play_move(col);
            if is_legal_move
            {
                children.push(new_child);
            }
        }
        return children;
    }

    fn get_min_evaluation() -> Self::EvalOutput { -1 }
    fn get_max_evaluation() -> Self::EvalOutput { 1 }

    fn is_terminal_state(&self) -> bool
    {
        return !(self.get_winner() == Player::Empty) || self.is_filled();
    }

    fn evaluate(&self) -> Self::EvalOutput
    {
        match self.get_winner()
        {
            Player::Red => 1,
            Player::Black => -1,
            Player::Empty => 0,
        }
    }
}
