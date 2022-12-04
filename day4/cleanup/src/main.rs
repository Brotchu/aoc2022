use std::fs;

fn main() {
    let total_overlaps = read_file("./src/cleanup_assignment.txt")
        .iter()
        .map(|l| ranges_from_line(l))
        .map(|r| check_overlap(r))
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();
    println!("total overlaps: {total_overlaps}");
}


struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn create_range(s :u32, e :u32) -> Range {
        Range{
            start: s,
            end: e,
        }
    }

    fn contains(&self, r :&Range) -> bool {
        r.start >= self.start && r.end <= self.end
    }

    fn overlaps(&self, r: &Range) -> bool {
        (r.start >= self.start  && r.start <= self.end ) || (r.end <= self.end && r.end >= self.start)
    }
}

// fn check_overlap(r1 :&Range, r2 :&Range) -> u32 {
//     if r1.contains(r2) || r2.contains(r1) {
//         return 1
//     }
//     0
// }

fn check_containing(r :Option<(Option<Range>, Option<Range>)>) -> u32 {
    match r {
        None => 0,
        Some((r1, r2))=> match (r1, r2) {
            (Some(r1), Some(r2)) => if r1.contains(&r2) || r2.contains(&r1) {
                return 1
            }else {
                return 0
            },
            _ => 0
        } 
    }
}

fn check_overlap(r :Option<(Option<Range>, Option<Range>)>) -> u32{
    match r {
        None => 0,
        Some((r1, r2)) => match(r1, r2) {
            (Some(r1), Some(r2)) => if r1.overlaps(&r2) || r2.overlaps(&r1) {
                return 1
            } else {
                return 0
            }
            _ => 0
        }
    }
}

// (r1, r2) => match (r1, r2) {
//     (None, None) => 0,
//     (None, _) => 0,
//     (_, None) => 0,
//     _ => if r1.contains(r2) || r2.contains(r1) {
//         return 1
//     } else {
//         return 0
//     }
// }
fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("error reading file");
    let split = contents.split('\n');
    let mut lines: Vec<String> = Vec::new();
    for s in split{
        lines.push(s.to_string());
    }
    lines
}

fn ranges_from_line(s :&str) -> Option<(Option<Range>, Option<Range> )>{
    let deers = s.split_once(',');
    deers.map(|(d1, d2)| ( text_to_range(d1),  text_to_range(d2)))
}

fn text_to_range(s :&str) -> Option<Range>{
    let range = s.split_once('-');
    range.map(|(s,e)| Range::create_range(
        s.to_string().parse::<u32>().unwrap(),
        e.to_string().parse::<u32>().unwrap()
    ))
}