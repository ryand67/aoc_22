use std::{
    collections::HashSet,
    io::{BufRead, BufReader, Result},
};

fn main() -> Result<()> {
    let file = std::fs::File::open("./src/input.txt")?;
    let reader = BufReader::new(file);

    let mut count: u16 = 0;
    for line in reader.lines() {
        let current = line.unwrap();
        let (group_one, group_two) = current.split_once(',').expect("didn't split");
        let (first, second) = group_one.split_once('-').expect("didn't split");
        let (third, fourth) = group_two.split_once('-').expect("didn't split");

        let first_parse = first.parse::<usize>().expect("asdf");
        let second_parse = second.parse::<usize>().expect("asdf");
        let third_parse = third.parse::<usize>().expect("asdf");
        let fourth_parse = fourth.parse::<usize>().expect("asdf");

        if is_inside(first_parse, second_parse, third_parse, fourth_parse)
            || is_inside(third_parse, fourth_parse, first_parse, second_parse)
        {
            count += 1;
        }
    }

    println!("{:?}", count);

    Ok(())
}

fn is_inside(a: usize, b: usize, c: usize, d: usize) -> bool {
    let min_one = a.min(b);
    let max_one = a.max(b);
    let min_two = c.min(d);
    let max_two = c.max(d);

    let mut seen: HashSet<usize> = HashSet::new();
    for i in min_one..=max_one {
        seen.insert(i);
    }

    for i in min_two..=max_two {
        if seen.contains(&i) {
            return true;
        }
    }

    false
}
