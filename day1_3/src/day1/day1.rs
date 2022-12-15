use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day1() -> std::io::Result<()> {
    let file = File::open("./src/day1/input_day1.txt")?;
    let reader = BufReader::new(file);

    let mut sum: u64 = 0;

    let mut sums: Vec<u64> = vec![];
    for line in reader.lines() {
        let num = line.unwrap();

        if num != "" {
            sum = sum + num.parse::<u64>().expect("Didn't parse");
        } else {
            sums.push(sum);
            sum = 0;
        }
    }

    sums.sort();

    let ans = sums[sums.len() - 1] + sums[sums.len() - 2] + sums[sums.len() - 3];

    print!("{:?}", ans);

    Ok(())
}
