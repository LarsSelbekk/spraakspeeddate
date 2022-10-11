use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num = args.get(1).unwrap().parse::<u8>().unwrap();
    println!("{}", sierpinsky(num));
}

fn sierpinsky(num: u8) -> String {
    // Can calculate nCm mod 2 using n xor m
    let mut first_row: Vec<u8> = vec![0; (num*2-1).into()];
    first_row[(num-1) as usize] = 1;
    let mut res: Vec<Vec<u8>> = vec![first_row];
    for _ in 1..num {
        res.push(next_row(res.last().unwrap()))    
    }
    
    let mut str_res = "".to_string();
    for row in res {
        for digit in row {
            str_res.push(match digit {
                0 => ' ',
                1 => '■',
                _ => todo!()
            })
        }
        str_res.push('\n');
    }
    str_res
}

fn next_row(row: &Vec<u8>) -> Vec<u8>{
    let mut res = vec![0_u8; row.len()];
    for i in 0..row.len() {
        let left: &u8 = match i.checked_sub(1) {
            None => &0,
            Some(less) =>row.get(less).unwrap()
        };
        let right: &u8 = row.get(i+1).unwrap_or(&0);
        res[i] = left ^ right;
    }
    res
}