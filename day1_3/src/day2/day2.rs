use std::{collections::HashMap, io::BufRead};

#[derive(Copy, Clone, Debug)]
enum Status {
    Win,
    Lose,
    Draw,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
enum Moves {
    Rock,
    Paper,
    Scissors,
}

pub fn day2() -> std::io::Result<()> {
    let file = std::fs::File::open("./src/day2/input.txt")?;
    let reader = std::io::BufReader::new(file);

    let mut sum: i32 = 0;

    for game in reader.lines() {
        let current = game.unwrap();
        let (won, my_move) = calc_game(&current);

        // println!("{} - {:?} - {:?}", current, won, my_move);

        match won {
            Status::Win => sum += 6,
            Status::Draw => sum += 3,
            Status::Lose => sum += 0,
        }

        match my_move {
            Moves::Rock => sum += 1,
            Moves::Paper => sum += 2,
            Moves::Scissors => sum += 3,
        }
    }

    print!("{}", sum);

    Ok(())
}

fn calc_game(game: &str) -> (Status, Moves) {
    let needs_map: HashMap<&str, Status> =
        HashMap::from([("X", Status::Lose), ("Y", Status::Draw), ("Z", Status::Win)]);

    let (p1, mut p2) = game.split_at(1);

    p2 = p2.trim();

    let my_move: Moves = find_move(*needs_map.get(p2).expect("nothing there"), p1);
    let my_result: Status = *needs_map.get(p2).expect("ASDF");

    (my_result, my_move)
}

fn find_move(need: Status, opponent: &str) -> Moves {
    let win_map: HashMap<Moves, Moves> = HashMap::from([
        (Moves::Rock, Moves::Paper),
        (Moves::Paper, Moves::Scissors),
        (Moves::Scissors, Moves::Rock),
    ]);

    let draw_map: HashMap<Moves, Moves> = HashMap::from([
        (Moves::Rock, Moves::Rock),
        (Moves::Paper, Moves::Paper),
        (Moves::Scissors, Moves::Scissors),
    ]);

    let lose_map: HashMap<Moves, Moves> = HashMap::from([
        (Moves::Rock, Moves::Scissors),
        (Moves::Paper, Moves::Rock),
        (Moves::Scissors, Moves::Paper),
    ]);

    let translate: HashMap<&str, Moves> = HashMap::from([
        ("A", Moves::Rock),
        ("B", Moves::Paper),
        ("C", Moves::Scissors),
    ]);

    let my_move: Moves = match need {
        Status::Win => *win_map
            .get(translate.get(opponent).expect("nope"))
            .expect("nah"),
        Status::Lose => *lose_map
            .get(translate.get(opponent).expect("nope"))
            .expect("nah"),
        Status::Draw => *draw_map
            .get(translate.get(opponent).expect("nope"))
            .expect("nah"),
    };

    my_move
}
