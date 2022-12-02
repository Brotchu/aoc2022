use std::fs;

fn main(){
    let contents = fs::read_to_string("./src/calories.txt")
        .expect("error reading file");
    // println!("{contents}")   
    let split = contents.split('\n');
    let mut calories: Vec<i32> = Vec::new();
    let mut temp:i32 = 0;
    for s in split {
        if s == "" {
            calories.push(temp);
            temp = 0;
        } else {
            let c:i32 = s.parse().unwrap();
            temp += c;
        }
    }

    println!("{:?}", calories);

    let mut top:i32 = 0;
    let mut top2:i32 = 0;
    let mut top3:i32 = 0;
    for calorie in calories.iter(){
        if calorie > &top {
            top3 = top2;
            top2 = top;
            top = *calorie;
        } else if calorie > &top2 {
            top3 = top2;
            top2 = *calorie;
        } else if calorie > &top3 {
            top3 = *calorie;
        }
    }
    let sum:i32 = top+top2+top3;
    println!("top 3 calories: {top},{top2},{top3}. total of top 3: {sum}")
}