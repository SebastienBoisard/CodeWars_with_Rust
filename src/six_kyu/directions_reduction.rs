#[allow(dead_code)]

/// # Directions Reduction
///
/// ## Instructions
///
///
///
/// ## Examples
///
///
///
/// ## What I learned
///
/// -
///

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    West,
    South,
}

fn is_opposite(d1: Direction, d2: Direction) -> bool {
    match (d1, d2) {
        (Direction::North, Direction::South)
        | (Direction::South, Direction::North)
        | (Direction::East, Direction::West)
        | (Direction::West, Direction::East) => true,
        _ => false,
    }
}
fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut stack: Vec<Direction> = Vec::new();
    for &d in arr {
        let Some(last) = stack.pop() else {
            stack.push(d);
            continue;
        };

        if !is_opposite(last, d) {
            stack.push(last);
            stack.push(d);
        }
    }
    println!("stack={stack:?}");
    stack
}

fn _dir_reduc_v1(arr: &[Direction]) -> Vec<Direction> {
    let mut stack = vec![];
    for &dir in arr.iter() {
        match (dir, stack.last()) {
            (Direction::North, Some(Direction::South))
            | (Direction::South, Some(Direction::North))
            | (Direction::East, Some(Direction::West))
            | (Direction::West, Some(Direction::East)) => {
                stack.pop();
            }
            _ => stack.push(dir),
        }
    }
    stack
}

#[cfg(test)]
mod tests {
    use super::{
        dir_reduc,
        Direction::{self, *},
    };

    #[test]
    fn basic() {
        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc(&a), [West]);

        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);
    }

    #[test]
    fn advanced() {
        let a = [North, South, South, East, West, North, West];
        assert_eq!(dir_reduc(&a), [West]);

        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);

        let a = [North, South, South, East, West, North, North];
        assert_eq!(dir_reduc(&a), [North]);

        let a = [
            East, East, West, North, West, East, East, South, North, West,
        ];
        assert_eq!(dir_reduc(&a), [East, North]);

        let a = [
            North, East, North, East, West, West, East, East, West, South,
        ];
        assert_eq!(dir_reduc(&a), [North, East]);

        let a = [North, West, South, East];
        assert_eq!(dir_reduc(&a), [North, West, South, East]);
    }

    use rand::{thread_rng, Rng};

    #[test]
    fn random() {
        let mut rng = thread_rng();

        for _ in 0..100 {
            let a = (0..rng.gen_range(1..10))
                .map(|_| match rng.gen_range(0..4) {
                    0 => North,
                    1 => East,
                    2 => South,
                    _ => West,
                })
                .collect::<Vec<_>>();

            let actual = dir_reduc(&a);
            let expect = dir_reduc_solution(&a);

            assert_eq!(actual, expect, "with directions {:?}", a);
        }
    }

    fn dir_reduc_solution(arr: &[Direction]) -> Vec<Direction> {
        let mut result: Vec<Direction> = Vec::new();

        for &s in arr {
            if !result.is_empty() && can_be_reduced(s, *result.last().unwrap()) {
                result.pop();
            } else {
                result.push(s);
            }
        }

        result
    }

    fn can_be_reduced(elem: Direction, last: Direction) -> bool {
        match (elem, last) {
            (North, South) | (South, North) | (West, East) | (East, West) => true,
            _ => false,
        }
    }
}
