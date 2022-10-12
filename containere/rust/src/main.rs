use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rule: u8 = args.get(1).expect("No rule provided").parse().expect("Rule was not an 8b-number");
    let num: u8 = args.get(2).expect("No height provided").parse().expect("Height was not an 8b-number");
    assert!(num > 0, "Height needs to be greater than 0");
    println!("{}", sierpinsky(rule, num));
}

fn sierpinsky(rule: u8, num: u8) -> String {
    let mut first_row: Vec<u8> = vec![0; (num*2-1).into()];
    first_row[(num-1) as usize] = 1;
    let mut res: Vec<Vec<u8>> = vec![first_row];
    for _ in 1..num {
        res.push(next_row(rule, res.last().unwrap()))    
    }
    
    let mut str_res = "".to_string();
    for row in res {
        for digit in row {
            str_res.push(match digit {
                0 => '⬜',
                1 => '⬛',
                _ => panic!("Number not 0/1!")
            })
        }
        str_res.push('\n');
    }
    str_res
}

fn next_row(rule: u8, row: &Vec<u8>) -> Vec<u8>{
    let mut res = vec![0_u8; row.len()];
    for i in 0..row.len() {
        let left: &u8 = match i.checked_sub(1) {
            None => row.last().unwrap(),
            Some(less) => row.get(less).unwrap()
        };
        let middle = row.get(i).unwrap();
        let right: &u8 = match i.checked_add(1) {
            None => row.first().unwrap(),
            Some(more) => match row.get(more) {
                None => row.first().unwrap(),
                Some(it) => it
            }
        };
        let offset: u8 = match (left, middle, right) {
            (1, 1, 1) => 7,
            (1, 1, 0) => 6,
            (1, 0, 1) => 5,
            (1, 0, 0) => 4,
            (0, 1, 1) => 3,
            (0, 1, 0) => 2,
            (0, 0, 1) => 1,
            (0, 0, 0) => 0,
            _ => panic!("Not found ({left}, {middle}, {right})")
        };
        res[i] = (rule & (1 << offset)) >> (offset)
    }
    res
}