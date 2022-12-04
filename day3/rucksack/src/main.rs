use std::fs;

fn main() {
    let total_priority= read_file_to_vec("./src/contents.txt".to_string())
        .iter()
        .map(|content| get_priority_rucksack(content))
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();

    println!("{total_priority}");

    let content = read_file_to_vec("./src/contents.txt".to_string());
    let badge_priority = group_3_strings(&content)
        .iter()
        .map(|group| find_common_badge_priority(group))
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();

    println!("badge priority: {badge_priority}")
}

fn get_priority_rucksack(s :&String) -> u32 {
    let (s1,s2) = split_string_in_half(s);
    let c = find_common_character(s1, s2);
    find_character_priority(c)
}

fn split_string_in_half(s :&String) -> (String, String) {
    let mid = s.len()/2;
    (s[0..mid].to_string(), s[mid..].to_string())
}

fn find_common_character(s1 :String, s2 :String) -> char {
    for c1 in s1[..].chars() {
        for c2 in s2[..].chars() {
            if c1 == c2 {
                return c1
            }
        }
    }
    ' '
}

fn group_3_strings(lines :&Vec<String>) ->Vec<(String, String, String)> {
    let mut groups:Vec<(String, String, String)> = Vec::new();
    let mut group:Vec<String> = Vec::new();
    let mut i =0;
    for s in lines.iter() {
        group.push(s.to_string());
        i += 1;
        if i == 3 {
            i = 0;
            let s1 = group.remove(0);
            let s2 = group.remove(0);
            let s3 = group.remove(0);
            groups.push((s1, s2, s3));
            group.clear();
        }
    }
    groups
}

fn find_common_badge_priority( s :&(String, String, String)) -> u32 {
    for c1 in s.0[..].chars() {
        if s.1[..].contains(c1) && s.2[..].contains(c1) {
            return find_character_priority(c1)
        }
    }
    0
}

/*
A :65 -> 27
Z :90 -> 52

a :97 -> 1
z :122-> 26
*/
fn find_character_priority(c :char) -> u32 {
    if c.is_uppercase(){
        return c as u32 -38
    }
    c as u32 -96
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