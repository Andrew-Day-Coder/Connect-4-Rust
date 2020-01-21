#![allow(clippy::needless_return)]
#![allow(clippy::suspicious_else_formatting)]

use std::cmp::{max, min};

pub trait Evaluable<T>
{
    type EvalOutput: PartialOrd + PartialEq + Copy + Ord;
    fn get_children(&self)-> Vec<T> where T: Evaluable<T>;
    fn get_min_evaluation() -> Self::EvalOutput;
    fn get_max_evaluation() -> Self::EvalOutput;
    fn is_terminal_state(&self) -> bool;
    fn evaluate(&self) -> Self::EvalOutput;
}

#[allow(dead_code)]
pub struct Minimax<InfoNode: Evaluable<InfoNode>>
{
    value: Option<<InfoNode as Evaluable<InfoNode>>::EvalOutput>,
    info: InfoNode,
}
#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Player
{
    MINIMIZING,
    MAXIMIZING,
}
#[allow(dead_code)]
impl Player
{
    fn get_other_player(&self) -> Player
    {
        match self
        {
            Player::MINIMIZING => Player::MAXIMIZING,
            Player::MAXIMIZING => Player::MINIMIZING,
        }
    }
}
#[allow(dead_code)]
struct MinimaxResult
{

}
#[allow(dead_code)]
impl<InfoNode: Evaluable<InfoNode>> Minimax<InfoNode>
{
    /*
        This is an implentation of the Minimax algorithm
            (https://en.wikipedia.org/wiki/Minimax)
        Useful for evaluating trees of data, and finding
        best outcomes for those trees for a given player.
        Only works for zero-sum trees
            (https://en.wikipedia.org/wiki/Zero-sum_game)
        which is where when a player gains an
        advantage, he takes it directly from his
        opponent
    */

    // creates a new minimax node
    pub fn new(info: InfoNode) -> Minimax<InfoNode>
    {
        Minimax{value: None, info}
    }
    fn get_bound(player: &Player) -> <InfoNode as Evaluable<InfoNode>>::EvalOutput
    {
        match player
        {
                Player::MINIMIZING => InfoNode::get_max_evaluation(),
                Player::MAXIMIZING => InfoNode::get_min_evaluation(),
        }
    }
    pub fn get_info(&mut self) -> &mut InfoNode
    {
        return &mut self.info;
    }
    pub fn get_positional_estimate(&self) -> Option<<InfoNode as Evaluable<InfoNode>>::EvalOutput>
    {
        return self.value;
    }
    // actual minimax implementation
    pub fn minimax(&mut self, depth: u16, player: Player) -> Option<Minimax<InfoNode>>
    {
        // generate the nodes used for the next depth of the branch
        let children = self.info.get_children();
        // if we are at the bottom of the current branh
        if children.is_empty() || depth == 0 || self.info.is_terminal_state()
        {
            self.value = Some(self.info.evaluate());
            return None;
        }
        // create a variable to store the best sub-node
        let mut best_child: Option<Minimax<InfoNode>> = None;

        // storing the hueristic value of node here
        self.value = Some(Minimax::<InfoNode>::get_bound(&player));

        // iterate over
        for child in children
        {
            // create a minimax node from the raw(info) child node
            let mut new_child = Minimax::new(child);
            // recursive call to minimax
            new_child.minimax(depth - 1, player.get_other_player());
            // get the heuristic value from the minimax evaluation
            let child_value = new_child.value.unwrap();
            // calcuate the best child node
            if best_child.is_none() ||
               player == Player::MINIMIZING && child_value < self.value.unwrap() ||
               player == Player::MAXIMIZING && child_value > self.value.unwrap()
            {
                best_child = Some(new_child);
                self.value = Some(child_value);
            }
        }
        return best_child;
    }
    fn alpha_beta_with_arguments(&mut self,depth: u16, mut alpha: <InfoNode as Evaluable<InfoNode>>::EvalOutput, mut beta: <InfoNode as Evaluable<InfoNode>>::EvalOutput, player: Player) -> Option<Minimax<InfoNode>>
    {
        // generate the nodes used for the next depth of the branch
        let children = self.info.get_children();
        // if we are at the bottom of the current branh
        if children.is_empty() || depth == 0 || self.info.is_terminal_state()
        {
            self.value = Some(self.info.evaluate());
            return None;
        }
        // create a variable to store the best sub-node
        let mut best_child: Option<Minimax<InfoNode>> = None;

        // storing the hueristic value of node here
        self.value = Some(Minimax::<InfoNode>::get_bound(&player));

        // iterate over
        for child in children
        {
            // create a minimax node from the raw(info) child node
            let mut new_child = Minimax::new(child);
            // recursive call to minimax
            new_child.minimax(depth - 1, player.get_other_player());
            // get the heuristic value from the minimax evaluation
            let child_value = new_child.value.unwrap();
            // calcuate the best child node
            if best_child.is_none() ||
               player == Player::MINIMIZING && child_value < self.value.unwrap() ||
               player == Player::MAXIMIZING && child_value > self.value.unwrap()
            {
                best_child = Some(new_child);
                self.value = Some(child_value);
                // set alpha/beta
                match player
                {
                    Player::MAXIMIZING => alpha = max(alpha, child_value),
                    Player::MINIMIZING => beta = min(beta, child_value),
                }
            }
            if alpha >= beta
            {
                return best_child;
            }
        }
        return best_child;
    }
    pub fn alpha_beta(&mut self, depth: u16, player: Player) -> Option<Minimax<InfoNode>>
    {
        self.alpha_beta_with_arguments(depth, InfoNode::get_min_evaluation(), InfoNode::get_max_evaluation(), player)
    }
}
