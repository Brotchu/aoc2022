use std::fs;

fn main() {
    let l = read_file_to_vec("./src/strategy.txt".to_string());

    let mut score:i32 = 0;
    for play in l.iter(){
        if !play.is_empty() {
            let opponent_move = play.chars().nth(0).unwrap();
            let opponent_move = Move::create_opponent_move(opponent_move);
            // let self_move = play.chars().nth(2).unwrap();
            let required_result = play.chars().nth(2).unwrap();
            let required_result = Result::create_required_result(required_result);
            score += points_in_round(required_result, opponent_move);
            // println!("{opponent_move}x{self_move}");
            // if self_move == 'X' {
            //     score += 1;
            //     if opponent_move == 'C' {
            //         score += 6;
            //     } else if opponent_move == 'A' {
            //         score += 3;
            //     }
            // } else if  self_move == 'Y' {
            //     score += 2;
            //     if opponent_move == 'A' {
            //         score += 6;
            //     } else if opponent_move == 'B' {
            //         score+= 3;
            //     }
            // } else if self_move == 'Z' {
            //     score += 3;
            //     if opponent_move == 'B' {
            //         score += 6;
            //     } else if opponent_move == 'C' {
            //         score += 3;
            //     }
            // }
        }
    }
    println!("total score {score}")
}


enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn create_opponent_move(m: char) -> Move {
        if m == 'A' {
            Move::Rock
        } else if  m == 'B' {
            Move::Paper
        } else  {
            Move::Scissors
        }
    }

    fn points(&self) -> i32 {
        match self{
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

enum Result {
    Win,
    Draw,
    Loose,
}

impl Result {
    fn create_required_result(r: char) -> Result {
        if r == 'X' {
            Result::Loose
        } else if r == 'Y' {
            Result::Draw
        } else {
            Result::Win
        }
    }

    fn points(&self) -> i32 {
        match self{
            Result::Win => 6,
            Result::Draw => 3,
            Result::Loose => 0,
        }
    }
}

fn find_suitable_move(required: &Result, opponent: Move) -> Move{
    match opponent {
        Move::Rock => match required{
            Result::Win => Move::Paper,
            Result::Draw => Move::Rock,
            Result::Loose => Move::Scissors
        }
        Move::Paper => match required{
            Result::Win => Move::Scissors,
            Result::Draw => Move::Paper,
            Result::Loose => Move::Rock
        }
        Move::Scissors => match required{
            Result::Win => Move::Rock,
            Result::Draw => Move::Scissors,
            Result::Loose => Move::Paper,
        }
    }
}

fn points_in_round(required: Result, opponent: Move) -> i32 {
    let self_move = find_suitable_move(&required, opponent);
    required.points() + self_move.points()
}



fn read_file_to_vec(filename :String) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("error reading file");
    let split = contents.split('\n');
    let mut lines: Vec<String> = Vec::new();
    for s in split{
        lines.push(s.to_string());
    }
    lines
}

/*
A - rock
B - paper
C - scissors

X - rock        -> 1
Y - paper       -> 2
Z - scissors    -> 3

correction:
X - need to lose -> 0
Y - need to draw -> 3
Z - need to win  -> 6
*/