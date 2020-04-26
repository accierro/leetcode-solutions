// On an 8 x 8 chessboard, there is one white rook.  There also may be empty squares, white bishops, and black pawns.  These are given as characters 'R', '.', 'B', and 'p' respectively. Uppercase characters represent white pieces, and lowercase characters represent black pieces.

// The rook moves as in the rules of Chess: it chooses one of four cardinal directions (north, east, west, and south), then moves in that direction until it chooses to stop, reaches the edge of the board, or captures an opposite colored pawn by moving to the same square it occupies.  Also, rooks cannot move into the same square as other friendly bishops.

// Return the number of pawns the rook can capture in one move.

// Example 1:

// Input: [['.','.','.','.','.','.','.','.'],['.','.','.','p','.','.','.','.'],['.','.','.','R','.','.','.','p'],['.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.'],['.','.','.','p','.','.','.','.'],['.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.']]
// Output: 3
// Explanation:
// In this example the rook is able to capture all the pawns.
// Example 2:

// Input: [['.','.','.','.','.','.','.','.'],['.','p','p','p','p','p','.','.'],['.','p','p','B','p','p','.','.'],['.','p','B','R','B','p','.','.'],['.','p','p','B','p','p','.','.'],['.','p','p','p','p','p','.','.'],['.','.','.','.','.','.','.','.'],['.','.','.','.','.','.','.','.']]
// Output: 0
// Explanation:
// Bishops are blocking the rook to capture any pawn.
// Example 3:

// Input: [['.','.','.','.','.','.','.','.'],['.','.','.','p','.','.','.','.'],['.','.','.','p','.','.','.','.'],['p','p','.','R','.','p','B','.'],['.','.','.','.','.','.','.','.'],['.','.','.','B','.','.','.','.'],['.','.','.','p','.','.','.','.'],['.','.','.','.','.','.','.','.']]
// Output: 3
// Explanation:
// The rook can capture the pawns at positions b5, d6 and f5.

// Note:

// board.length == board[i].length == 8
// board[i][j] is either 'R', '.', 'B', or 'p'
// There is exactly one cel

struct Solution;

impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        let mut rook = (0, 0);

        'outer: for m in 0..board.len() {
            for n in 0..board.len() {
                if board[m][n] == 'R' {
                    rook.0 = m;
                    rook.1 = n;
                    break 'outer;
                }
            }
        }

        let mut n = rook.0 + 1;
        while n < 8 {
            match board[n][rook.1] {
                'p' => {
                    result += 1;
                    break;
                }
                'B' => {
                    break;
                }
                _ => (),
            }
            n += 1;
        }

        let mut n = rook.0 - 1;
        while n > 0 {
            match board[n][rook.1] {
                'p' => {
                    result += 1;
                    break;
                }
                'B' => {
                    break;
                }
                _ => (),
            }
            n -= 1;
        }

        let mut n = rook.1 - 1;
        while n > 0 {
            match board[rook.0][n] {
                'p' => {
                    result += 1;
                    break;
                }
                'B' => {
                    break;
                }
                _ => (),
            }
            n -= 1;
        }

        let mut n = rook.0 + 1;
        while n < 8 {
            match board[rook.0][n] {
                'p' => {
                    result += 1;
                    break;
                }
                'B' => {
                    break;
                }
                _ => (),
            }
            n += 1;
        }

        result
    }
}

fn main() {
    assert_eq!(
        Solution::num_rook_captures(vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.']
        ]),
        3
    );
    assert_eq!(
        Solution::num_rook_captures(vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'R', 'B', '.', '.', 'p'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.']
        ]),
        2
    );
}
