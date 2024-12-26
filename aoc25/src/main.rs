use std::fs::read_to_string;

#[derive(Clone)]
struct LockOrKey {
    tumblers: Vec<usize>,
    typ: Typ,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Typ {
    LOCK,
    KEY,
}

impl LockOrKey {
    fn from<'a, F>(iter: &mut F) -> Option<Self>
    where
        F: Iterator<Item = &'a str>,
    {
        let mut iter = iter.take_while(|line| !line.is_empty());
        let top = iter.next()?;
        let mut tumblers = vec![0; top.len()];
        for line in iter {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    tumblers[i] += 1;
                }
            }
        }

        if top.chars().all(|c| c == '#') {
            // lock
            Some(LockOrKey {
                tumblers,
                typ: Typ::LOCK,
            })
        } else if top.chars().all(|c| c == '.') {
            // key
            for t in tumblers.iter_mut() {
                *t -= 1;
            }
            Some(LockOrKey {
                tumblers,
                typ: Typ::KEY,
            })
        } else {
            panic!();
        }
    }
}

fn parse_input(data: &str) -> (Vec<LockOrKey>, Vec<LockOrKey>) {
    let mut lines = data.lines();

    let mut locks_and_keys = Vec::new();

    while let Some(lock_or_key) = LockOrKey::from(&mut lines) {
        locks_and_keys.push(lock_or_key);
    }

    locks_and_keys
        .into_iter()
        .partition(|lock_or_key| lock_or_key.typ == Typ::LOCK)
}

fn lock_key_fit(lock: &LockOrKey, key: &LockOrKey) -> bool {
    lock.tumblers
        .iter()
        .enumerate()
        .all(|(i, _)| lock.tumblers[i] + key.tumblers[i] < 6)
}

fn lock_key_pairs_that_fit(locks: &[LockOrKey], keys: &[LockOrKey]) -> usize {
    locks
        .iter()
        .map(|lock| keys.iter().filter(|key| lock_key_fit(lock, key)).count())
        .sum()
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let input_file = if args.len() >= 2 {
        std::path::PathBuf::from(&args[1])
    } else {
        let exe = std::env::current_exe().unwrap();
        exe.parent()
            .unwrap()
            .join("../..")
            .join(exe.file_name().unwrap())
            .join("src/main.txt")
    };
    let data = read_to_string(&input_file).unwrap();
    let (locks, keys) = parse_input(&data);

    println!("lock/key pairs: {}", lock_key_pairs_that_fit(&locks, &keys));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let (locks, keys) = parse_input(&data);
        assert_eq!(locks.len(), 2);
        assert_eq!(locks[0].tumblers, vec![0, 5, 3, 4, 3]);
        assert_eq!(locks[1].tumblers, vec![1, 2, 0, 5, 3]);
        assert_eq!(keys.len(), 3);
        assert_eq!(keys[0].tumblers, vec![5, 0, 2, 1, 3]);
        assert_eq!(keys[1].tumblers, vec![4, 3, 4, 0, 2]);
        assert_eq!(keys[2].tumblers, vec![3, 0, 2, 0, 1]);

        assert_eq!(lock_key_pairs_that_fit(&locks, &keys), 3);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let (locks, keys) = parse_input(&data);

        assert_eq!(lock_key_pairs_that_fit(&locks, &keys), 3196);
    }
}
