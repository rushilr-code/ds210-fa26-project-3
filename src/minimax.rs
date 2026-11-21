//! Your agent. Checkpoint 2 is the search; checkpoint 3 is the heuristic.

use std::time::Duration;

use crate::agent::Agent;
use crate::board::{Board, Cell, Position};
use crate::player::Player;

/// The type of a heuristic: any function that looks at a board and returns a score from
/// X's point of view. `score_only` and `my_heuristic` below both have this type.
pub type Heuristic = fn(&Board) -> i32;

/// What your agent is called in standings. Change this when you enter the tournament:
/// letters, digits and underscores only, up to 20 characters.
pub const AGENT_NAME: &str = "Minimax";

pub struct Minimax {
    /// How many moves ahead to look. `usize::MAX` means all the way to the end.
    max_depth: usize,
    /// What this agent calls itself in standings.
    label: String,
    /// How to guess at a position it didn't finish searching.
    heuristic: Heuristic,
}

impl Minimax {
    /// Searches every line of play to the end of the game. Fine on 3x3, hopeless on 5x5.
    pub fn new() -> Minimax {
        Minimax { max_depth: usize::MAX, label: String::from(AGENT_NAME), heuristic: my_heuristic }
    }

    /// Looks `depth` moves ahead, then estimates with `my_heuristic`. Depth 1 looks at each
    /// legal move and nothing further, which with `score_only` is exactly what `Greedy` does.
    pub fn with_depth(depth: usize) -> Minimax {
        assert!(depth >= 1, "an agent has to look at least one move ahead");
        Minimax { max_depth: depth, label: format!("{AGENT_NAME}(depth {depth})"), heuristic: my_heuristic }
    }

    /// Looks `depth` moves ahead, then estimates with the function you pass. Give it a
    /// name, so you can tell two versions apart in the standings.
    ///
    /// ```ignore
    /// let agent = Minimax::with_heuristic(4, "score only", score_only);
    /// ```
    pub fn with_heuristic(depth: usize, label: &str, heuristic: Heuristic) -> Minimax {
        assert!(depth >= 1, "an agent has to look at least one move ahead");
        Minimax { max_depth: depth, label: format!("{label}(depth {depth})"), heuristic }
    }

    /// The best score the player to move can force from `board`, looking `depth` moves
    /// ahead. Scores are from X's point of view: X picks the biggest, O the smallest.
    fn search(&self, board: &mut Board, depth: usize) -> i32 {
        // Checkpoint 2. The recursive part.
        //   1. If the game is over, the score is `board.score()`. Nothing to search.
        if board.is_over(){
            return board.score();
        }
        //   2. If `depth` is 0, you have looked as far as you're allowed. Return the
        //      agent's guess: `(self.heuristic)(board)`. The parentheses around
        //      `self.heuristic` say "the function stored in this field", and the second
        //      pair calls it.
        if depth == 0 {
            return (self.heuristic)(board);
        }
        //   3. Otherwise, for every legal move: apply it, call `search` on the result with
        //      one less depth, and undo it. X keeps the biggest score it sees, O the
        //      smallest. That is the whole algorithm.
        // `board.to_move()` tells you whose turn it is at this point in the search.
        let mut best;

        if board.to_move() == Player::X {
            best = i32::MIN;
        } 
        else {
            best = i32::MAX;
        }
        
        
        for position in board.moves() {
            board.apply_move(position);

        let score = self.search(board, depth - 1);

        board.undo();

        if board.to_move() == Player::X {
            if score > best {
                best = score;
            }
        } 
        else {
            if score < best {
                best = score;
            }
        }
        }

        best
    }
}

impl Agent for Minimax {
    fn name(&self) -> String {
        self.label.clone()
    }

   
fn choose_move(&self, board: &Board, _time_limit: Duration) -> Position {
    // Clone the original board
    let mut board = board.clone();

    // turn
    let player = board.to_move();

    //legal moves
    let moves = board.moves();

    //first legal move
    let mut best_move = moves[0];

    //best score
    let mut best_score = if player == Player::X {
        i32::MIN
    } else {
        i32::MAX
    };

    //try every legal move
    for position in moves {
        // Apply the current move
        board.apply_move(position);

        //Search remaining depth
        let score = self.search(&mut board, self.max_depth - 1);

        // Undo the move afterward
        board.undo();

        // X wants the highest score
        if player == Player::X {
            if score > best_score {
                best_score = score;
                best_move = position;
            }
        } else {
            // O wants the lowest score
            if score < best_score {
                best_score = score;
                best_move = position;
            }
        }
    }

    // Return the best position
    best_move
}

}

/// The simplest heuristic: who is ahead right now. Ours, kept so you have something to
/// measure yours against.
pub fn score_only(board: &Board) -> i32 {
    board.score()
}

/// Your heuristic: a guess at how the game will end from a position that isn't finished.
/// Positive favors X, negative favors O, and the units are runs of three, like `score()`.
pub fn my_heuristic(board: &Board) -> i32 {
    // Checkpoint 3. `board.score()` is a reasonable first guess: it says who is ahead.
    // It's also blind to everything that hasn't finished yet. Make it see more.
    board.score()
    }
