use std::collections::{HashMap, HashSet};
use std::io::BufRead;

const LC_OFFSET: i32 = 96;
const UC_OFFSET: i32 = 38;

pub fn day3() -> std::io::Result<()> {
    let file = std::fs::File::open("./src/day3/input.txt")?;
    let reader = std::io::BufReader::new(file);

    let mut sum: i32 = 0;

    let mut group: Vec<String> = vec![];

    let mut seen_set: Vec<HashSet<char>> = vec![];
    for (_, line) in reader.lines().enumerate() {
        let item: String = line.unwrap();

        group.push(item.clone());

        let mut set: HashSet<char> = HashSet::new();
        for c in item.chars() {
            set.insert(c);
        }

        seen_set.push(set);

        if group.len() == 3 {
            let mut record: HashMap<char, HashSet<usize>> = HashMap::new();

            for (idx, val) in group.iter().enumerate() {
                for c in val.chars() {
                    record.entry(c).or_default().insert(idx);
                }
            }

            for (key, value) in record {
                println!("{:?} - {:?}", key, value);
                if value.len() == 3 {
                    sum += calc_priority(key);
                }
            }

            group = Vec::new();
            seen_set = Vec::new();
        }
    }

    println!("{:?}", sum);

    Ok(())
}

fn calc_priority(c: char) -> i32 {
    if c.is_lowercase() {
        c as i32 - LC_OFFSET
    } else if c.is_uppercase() {
        c as i32 - UC_OFFSET
    } else {
        0
    }
}
