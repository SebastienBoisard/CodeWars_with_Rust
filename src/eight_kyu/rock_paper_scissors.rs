#[allow(dead_code)]

/// # Rock Paper Scissors
///
/// ## Instructions
///
/// Let's play! You have to return which player won! In case of a draw return Draw!.
///
/// ## Examples
///
/// (Input1, Input2 --> Output)
///
/// "scissors", "paper" --> "Player 1 won!"
/// "scissors", "rock" --> "Player 2 won!"
/// "paper", "paper" --> "Draw!"
///
/// ## What I learned
///
/// - in `match` expression like `match Scrutinee { MatchArms }`, the `Scrutinee` is an expression
/// so it can work with a tuple for exemple.
/// See: https://doc.rust-lang.org/reference/expressions/match-expr.html
///
///  - the `or_patterns`
/// See: https://rust-lang.github.io/rfcs/2535-or-patterns.html)
///

fn find_winner_at_rock_paper_scissors(p1: &str, p2: &str) -> &'static str {
    match (p1, p2) {
        ("scissors", "paper") | ("paper", "rock") | ("rock", "scissors") => "Player 1 won!",
        ("scissors", "rock") | ("paper", "scissors") | ("rock", "paper") => "Player 2 won!",
        (x, y) if x == y => "Draw!",
        _ => "error",
    }
}

#[cfg(test)]
mod tests {
    use super::find_winner_at_rock_paper_scissors;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(p1: &str, p2: &str, expected: &str) {
        assert_eq!(
            find_winner_at_rock_paper_scissors(p1, p2),
            expected,
            "{ERR_MSG} with p1 = \"{p1}\", p2 = \"{p2}\""
        )
    }

    #[test]
    fn fixed_tests() {
        dotest("rock", "scissors", "Player 1 won!");
        dotest("scissors", "rock", "Player 2 won!");
        dotest("rock", "rock", "Draw!");
    }
}
