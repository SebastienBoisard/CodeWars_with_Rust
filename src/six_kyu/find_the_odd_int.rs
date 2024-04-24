use std::collections::HashMap;

#[allow(dead_code)]

/// # Find the odd int
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

fn find_odd(arr: &[i32]) -> i32 {
    let mut h: HashMap<i32, i8> = HashMap::new();
    let _c = arr
        .iter()
        .map(|&x| {
            let count = h.entry(x).or_insert(0i8);
            *count += 1i8;
            ()
        })
        .collect::<()>();
    let b: Vec<&i32> = h
        .iter()
        .filter(|(_k, &v)| v % 2 != 0)
        .map(|(k, _v)| k)
        .collect();
    *b[0]
}

fn _find_odd_v1(arr: &[i32]) -> i32 {
    arr.iter().fold(0_i32, |a, v| a ^ v)
}

use std::collections::BTreeSet;

fn _find_odd_v2(arr: &[i32]) -> i32 {
    let mut set = BTreeSet::new();
    for num in arr {
        if !set.insert(*num) {
            set.remove(num);
        }
    }
    set.pop_first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::find_odd;
    use rand::seq::SliceRandom;
    use rand::Rng;
    use std::collections::HashSet;

    #[test]
    fn basic_tests() {
        assert_eq!(
            find_odd(&vec![
                20, 1, -1, 2, -2, 3, 3, 5, 5, 1, 2, 4, 20, 4, -1, -2, 5
            ]),
            5
        );
        assert_eq!(find_odd(&vec![1, 1, 2, -2, 5, 2, 4, 4, -1, -2, 5]), -1);
        assert_eq!(find_odd(&vec![20, 1, 1, 2, 2, 3, 3, 5, 5, 4, 20, 4, 5]), 5);
        assert_eq!(find_odd(&vec![10]), 10);
        assert_eq!(find_odd(&vec![1, 1, 1, 1, 1, 1, 10, 1, 1, 1, 1]), 10);
        assert_eq!(find_odd(&vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 10, 10]), 1);
    }

    #[test]
    fn random_tests() {
        let mut rng = rand::thread_rng();

        for _ in 0..40 {
            // create distinct random ints
            let mut distinct_ints = HashSet::with_capacity(20);
            while distinct_ints.len() < 20 {
                distinct_ints.insert(rng.gen::<i32>());
            }

            // repeat each distinct int even times (2|4|6)
            let mut ints = distinct_ints
                .iter()
                .map(|n| vec![n; 2 * rng.gen_range(1..3)])
                .flatten()
                .cloned()
                .collect::<Vec<i32>>();

            // generate random int to be repeted odd times
            let mut the_only_odd = rng.gen::<i32>();
            while distinct_ints.contains(&the_only_odd) {
                the_only_odd = rng.gen::<i32>();
            }

            // add the only odd with (1|3|5) repeats
            let odd_repeats = 2 * rng.gen_range(1..3) - 1;
            for _ in 0..odd_repeats {
                ints.push(the_only_odd);
            }

            ints.shuffle(&mut rng);

            assert_eq!(find_odd(&ints), the_only_odd);
        }
    }
}
