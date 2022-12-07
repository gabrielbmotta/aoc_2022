use std::str;

fn do_move9000(crate_piles: &mut Vec::<Vec::<char>>, src: u8, dst: u8, n: u8) {
    for _i in 0..n {
        if crate_piles[src as usize].is_empty() {
            return;
        }

        let c = crate_piles[src as usize].pop().unwrap();
        crate_piles[dst as usize].push(c);
    }
}
fn do_move9001(crate_piles: &mut Vec::<Vec::<char>>, src: u8, dst: u8, mut n: u8) {
    let src_size = crate_piles[src as usize].len(); // Boy are we hoping these instructions are good
    if src_size < n as usize {
        n = src_size as u8;
        if n == 0 {
            return;
        }
    }
    let src_pile = crate_piles.get(src as usize).unwrap();
    let mut moved = Vec::<char>::new();
    moved.extend_from_slice(&src_pile[src_size - n as usize..src_size]);

    for c in moved {
        crate_piles[dst as usize].push(c);
    }
    for _i in 0..n {
        crate_piles[src as usize].pop();
    }
}

fn solve(input: &str) {
    let mut crate_piles = Vec::<Vec::<char>>::new();
    crate_piles.resize(9, Vec::<char>::new());

    for line in input.split_terminator('\n') {
        if !line.is_empty() {
            if line.contains("[") {
                let mut start = 0;
                for i in 0..line.chars().count() {
                    if (i - start == 3) || (i == line.chars().count() - 1) {
                        let chunk = &line[start..i];

                        if !chunk.trim().is_empty() {
                            let bucket = i / 4usize;
                            crate_piles[bucket].insert(0, chunk.chars().nth(1).unwrap());
                        }

                        start = i + 1;
                    }
                }
            }
            else if line.contains("move") {
//                let mv = scan!(line, String, u8, String, u8, String, u8);
                let from = line.find("from").unwrap();
                let to   = line.find("to").unwrap();
                let n = &line[5..from].trim().parse::<u8>().unwrap();
                let src = &line[from+5..to].trim().parse::<u8>().unwrap();
                let dst = &line[to+3..].trim().parse::<u8>().unwrap();

                do_move9001(&mut crate_piles, *src - 1, *dst - 1, *n);
            }
        }
    }

    for mut pile in crate_piles {
        let c = match pile.pop() {
            Some(char)=>char,
            None=>' ',
        };

        print!("{c}");
    }
    println!("\n");
}


// I don't want to have to worry about dependencies or anything else,
// so just pass input as a long string. Easy to copy, not so easy to read.
fn main() {
    solve("
                        [R] [J] [W]
            [R] [N]     [T] [T] [C]
[R]         [P] [G]     [J] [P] [T]
[Q]     [C] [M] [V]     [F] [F] [H]
[G] [P] [M] [S] [Z]     [Z] [C] [Q]
[P] [C] [P] [Q] [J] [J] [P] [H] [Z]
[C] [T] [H] [T] [H] [P] [G] [L] [V]
[F] [W] [B] [L] [P] [D] [L] [N] [G]
 1   2   3   4   5   6   7   8   9 

move 2 from 2 to 8
move 2 from 1 to 6
move 8 from 7 to 1
move 7 from 5 to 4
move 1 from 6 to 4
move 1 from 6 to 3
move 6 from 3 to 5
move 9 from 8 to 1
move 3 from 6 to 7
move 14 from 4 to 1
move 6 from 1 to 7
move 16 from 1 to 9
move 6 from 1 to 4
move 1 from 8 to 6
move 4 from 1 to 5
move 11 from 9 to 7
move 2 from 1 to 8
move 1 from 6 to 7
move 1 from 8 to 7
move 1 from 8 to 3
move 7 from 4 to 3
move 14 from 7 to 6
move 8 from 6 to 9
move 19 from 9 to 2
move 1 from 1 to 2
move 2 from 9 to 7
move 9 from 7 to 8
move 2 from 2 to 8
move 16 from 2 to 9
move 4 from 8 to 2
move 1 from 7 to 9
move 3 from 9 to 6
move 3 from 3 to 6
move 11 from 9 to 2
move 7 from 5 to 3
move 2 from 5 to 9
move 6 from 6 to 4
move 1 from 6 to 4
move 4 from 6 to 8
move 5 from 9 to 1
move 4 from 1 to 7
move 3 from 2 to 6
move 3 from 4 to 1
move 1 from 4 to 1
move 2 from 1 to 3
move 4 from 3 to 7
move 1 from 5 to 2
move 3 from 1 to 6
move 15 from 2 to 5
move 3 from 6 to 3
move 13 from 3 to 8
move 2 from 4 to 2
move 9 from 5 to 4
move 2 from 2 to 5
move 5 from 7 to 5
move 10 from 8 to 6
move 1 from 2 to 5
move 10 from 4 to 6
move 4 from 8 to 6
move 3 from 7 to 1
move 3 from 1 to 9
move 1 from 2 to 1
move 8 from 5 to 2
move 3 from 6 to 9
move 6 from 8 to 5
move 6 from 9 to 2
move 1 from 1 to 9
move 10 from 2 to 1
move 4 from 8 to 5
move 10 from 5 to 9
move 11 from 9 to 7
move 5 from 7 to 9
move 1 from 9 to 2
move 3 from 2 to 9
move 2 from 2 to 8
move 4 from 9 to 5
move 4 from 1 to 9
move 5 from 5 to 2
move 5 from 1 to 4
move 21 from 6 to 9
move 3 from 2 to 9
move 2 from 8 to 1
move 25 from 9 to 6
move 4 from 5 to 7
move 1 from 4 to 6
move 6 from 6 to 4
move 3 from 4 to 6
move 7 from 7 to 3
move 4 from 9 to 1
move 3 from 7 to 8
move 2 from 9 to 8
move 2 from 2 to 8
move 4 from 1 to 3
move 9 from 6 to 2
move 13 from 6 to 4
move 13 from 4 to 5
move 1 from 5 to 8
move 2 from 2 to 3
move 6 from 5 to 3
move 19 from 3 to 6
move 1 from 4 to 9
move 2 from 8 to 1
move 5 from 2 to 3
move 5 from 1 to 9
move 7 from 5 to 4
move 1 from 8 to 3
move 1 from 2 to 6
move 8 from 6 to 3
move 1 from 9 to 8
move 11 from 4 to 2
move 1 from 4 to 6
move 1 from 2 to 8
move 5 from 3 to 4
move 4 from 9 to 6
move 1 from 6 to 8
move 9 from 3 to 1
move 7 from 2 to 9
move 1 from 2 to 6
move 3 from 1 to 8
move 2 from 2 to 3
move 3 from 9 to 7
move 3 from 4 to 7
move 2 from 4 to 3
move 2 from 3 to 5
move 8 from 6 to 4
move 6 from 8 to 6
move 2 from 9 to 4
move 5 from 8 to 6
move 3 from 7 to 5
move 1 from 5 to 8
move 1 from 8 to 2
move 1 from 5 to 1
move 11 from 4 to 9
move 2 from 6 to 3
move 2 from 2 to 4
move 6 from 1 to 2
move 6 from 2 to 1
move 3 from 7 to 3
move 2 from 4 to 7
move 4 from 6 to 5
move 7 from 3 to 7
move 5 from 9 to 6
move 22 from 6 to 8
move 2 from 6 to 5
move 2 from 8 to 4
move 14 from 8 to 7
move 11 from 7 to 4
move 3 from 8 to 1
move 9 from 7 to 8
move 10 from 1 to 4
move 1 from 7 to 4
move 4 from 8 to 7
move 6 from 4 to 9
move 7 from 4 to 1
move 3 from 4 to 8
move 1 from 5 to 8
move 8 from 5 to 3
move 4 from 3 to 9
move 7 from 8 to 9
move 3 from 8 to 3
move 2 from 8 to 2
move 7 from 9 to 1
move 2 from 2 to 8
move 8 from 9 to 1
move 8 from 1 to 7
move 7 from 1 to 5
move 7 from 7 to 1
move 11 from 9 to 8
move 9 from 8 to 5
move 2 from 8 to 5
move 3 from 1 to 8
move 2 from 3 to 7
move 6 from 4 to 1
move 6 from 1 to 6
move 5 from 7 to 1
move 2 from 4 to 6
move 1 from 3 to 5
move 4 from 7 to 4
move 2 from 8 to 7
move 10 from 5 to 6
move 9 from 6 to 1
move 8 from 1 to 6
move 1 from 7 to 2
move 9 from 6 to 4
move 2 from 4 to 3
move 3 from 8 to 1
move 1 from 2 to 4
move 4 from 4 to 1
move 7 from 4 to 3
move 3 from 3 to 2
move 1 from 7 to 6
move 9 from 6 to 7
move 6 from 7 to 4
move 2 from 7 to 2
move 6 from 4 to 7
move 2 from 2 to 9
move 1 from 2 to 4
move 1 from 7 to 4
move 4 from 7 to 6
move 4 from 5 to 4
move 1 from 2 to 5
move 1 from 7 to 5
move 1 from 2 to 6
move 6 from 4 to 3
move 9 from 3 to 9
move 4 from 6 to 2
move 7 from 3 to 8
move 22 from 1 to 7
move 1 from 1 to 7
move 2 from 8 to 3
move 4 from 5 to 6
move 2 from 3 to 2
move 6 from 2 to 8
move 3 from 8 to 6
move 1 from 4 to 8
move 1 from 1 to 8
move 8 from 6 to 7
move 7 from 8 to 9
move 22 from 7 to 4
move 3 from 5 to 6
move 1 from 8 to 1
move 2 from 8 to 2
move 3 from 6 to 4
move 1 from 1 to 3
move 15 from 9 to 1
move 5 from 1 to 5
move 3 from 7 to 6
move 5 from 5 to 6
move 4 from 4 to 3
move 6 from 6 to 9
move 7 from 7 to 6
move 5 from 6 to 7
move 4 from 1 to 9
move 3 from 7 to 4
move 2 from 9 to 7
move 5 from 3 to 5
move 3 from 6 to 3
move 5 from 4 to 6
move 10 from 9 to 5
move 1 from 2 to 9
move 1 from 3 to 5
move 1 from 2 to 9
move 3 from 1 to 6
move 2 from 9 to 2
move 7 from 6 to 5
move 15 from 4 to 9
move 2 from 4 to 5
move 1 from 3 to 4
move 9 from 9 to 1
move 1 from 9 to 2
move 2 from 9 to 4
move 11 from 5 to 4
move 1 from 9 to 3
move 1 from 6 to 8
move 4 from 7 to 8
move 4 from 8 to 9
move 15 from 4 to 7
move 1 from 6 to 7
move 1 from 3 to 7
move 6 from 9 to 6
move 1 from 3 to 7
move 1 from 2 to 1
move 1 from 9 to 5
move 3 from 6 to 1
move 11 from 1 to 4
move 6 from 5 to 1
move 2 from 2 to 5
move 1 from 5 to 7
move 2 from 6 to 1
move 7 from 5 to 7
move 3 from 5 to 6
move 4 from 6 to 1
move 11 from 4 to 3
move 1 from 8 to 5
move 23 from 7 to 6
move 18 from 6 to 9
move 1 from 5 to 9
move 1 from 4 to 2
move 3 from 3 to 7
move 3 from 3 to 8
move 17 from 1 to 8
move 5 from 6 to 5
move 2 from 7 to 1
move 20 from 8 to 2
move 4 from 7 to 2
move 3 from 9 to 5
move 7 from 9 to 7
move 6 from 9 to 2
move 1 from 1 to 8
move 3 from 9 to 4
move 7 from 5 to 2
move 6 from 7 to 1
move 1 from 1 to 8
move 3 from 2 to 6
move 1 from 7 to 6
move 2 from 8 to 9
move 35 from 2 to 4
move 3 from 3 to 2
move 1 from 5 to 7
move 2 from 3 to 9
move 3 from 1 to 6
move 2 from 2 to 1
move 32 from 4 to 7
move 3 from 4 to 8
move 3 from 9 to 5
move 1 from 1 to 2
move 21 from 7 to 5
move 2 from 2 to 1
move 3 from 1 to 2
move 15 from 5 to 1
move 3 from 6 to 7
move 3 from 4 to 6
move 3 from 8 to 5
move 1 from 9 to 3
move 8 from 7 to 2
move 6 from 5 to 2
move 9 from 1 to 6
move 4 from 7 to 1
move 2 from 5 to 4
move 2 from 4 to 3
move 3 from 5 to 4
move 17 from 2 to 7
move 3 from 3 to 5
move 2 from 4 to 8
move 1 from 4 to 3
move 5 from 7 to 9
move 1 from 3 to 6
move 4 from 1 to 7
move 4 from 6 to 7
move 2 from 5 to 2
move 1 from 1 to 3
move 10 from 6 to 4
move 1 from 3 to 7
move 20 from 7 to 8
move 8 from 4 to 8
move 1 from 2 to 8
move 4 from 9 to 1
move 3 from 7 to 4
move 2 from 4 to 9
move 2 from 6 to 3
move 1 from 2 to 8
move 1 from 7 to 6
move 1 from 9 to 5
move 3 from 5 to 9
move 4 from 9 to 2
move 1 from 4 to 5
move 1 from 5 to 3
move 3 from 2 to 4
move 1 from 9 to 7
move 1 from 2 to 1
move 1 from 7 to 1
move 11 from 1 to 2
move 3 from 1 to 7
move 25 from 8 to 5
move 1 from 6 to 3
move 1 from 6 to 2
move 7 from 8 to 2
move 9 from 2 to 8
move 2 from 4 to 7
move 2 from 5 to 7
move 2 from 5 to 2
move 5 from 5 to 1
move 7 from 5 to 1
move 2 from 4 to 9
move 3 from 5 to 6
move 1 from 1 to 8
move 1 from 5 to 6
move 1 from 4 to 7
move 1 from 9 to 2
move 3 from 5 to 2
move 2 from 6 to 9
move 3 from 9 to 8
move 1 from 5 to 4
move 3 from 3 to 9
move 10 from 1 to 5
move 4 from 2 to 8
move 2 from 6 to 1
move 3 from 9 to 7
move 1 from 1 to 9
move 1 from 4 to 3
move 1 from 9 to 2
move 9 from 8 to 2
move 2 from 3 to 7
move 2 from 7 to 6
move 3 from 5 to 6
move 4 from 8 to 6
move 4 from 8 to 3
move 4 from 3 to 2
move 4 from 6 to 8
move 1 from 7 to 9
move 2 from 1 to 8
move 2 from 8 to 3
move 1 from 9 to 2
move 13 from 2 to 4
move 6 from 5 to 7
move 2 from 5 to 7
move 10 from 2 to 4
move 11 from 7 to 8
move 1 from 6 to 4
move 4 from 6 to 7
move 24 from 4 to 9
move 11 from 7 to 4
move 1 from 3 to 8
move 1 from 3 to 5
move 4 from 4 to 2
move 5 from 4 to 2
move 9 from 2 to 5
move 4 from 9 to 5
move 1 from 5 to 1
move 2 from 5 to 7
move 2 from 2 to 5
move 1 from 1 to 7
move 2 from 2 to 3
move 18 from 9 to 6
move 9 from 8 to 1
move 2 from 9 to 5
move 5 from 1 to 8
move 2 from 8 to 7
move 4 from 8 to 4
move 5 from 8 to 7
move 10 from 5 to 1
move 10 from 7 to 4
move 4 from 5 to 8
move 14 from 1 to 9
move 6 from 9 to 8
move 1 from 5 to 1
move 12 from 6 to 9
move 4 from 6 to 8
move 11 from 8 to 5
move 1 from 6 to 1
move 19 from 9 to 7
move 2 from 3 to 5
move 13 from 7 to 5
move 3 from 7 to 1
move 4 from 8 to 9
move 2 from 7 to 6
move 7 from 4 to 8
move 5 from 8 to 1
move 1 from 1 to 3
move 1 from 7 to 2
move 6 from 1 to 6
move 1 from 2 to 5
move 1 from 8 to 1
move 1 from 8 to 2
move 2 from 4 to 8
move 5 from 6 to 1
move 2 from 4 to 7
move 2 from 9 to 6
move 1 from 6 to 5
move 4 from 6 to 2
move 1 from 9 to 5
move 2 from 4 to 5
move 4 from 2 to 4
move 2 from 8 to 3
move 3 from 3 to 2
move 4 from 1 to 2
move 2 from 4 to 7
move 4 from 2 to 3
move 4 from 1 to 2
move 13 from 5 to 1
move 1 from 6 to 2
move 1 from 1 to 8
move 15 from 5 to 2
move 4 from 3 to 1
move 5 from 4 to 3
move 1 from 3 to 6
move 1 from 8 to 7
move 1 from 9 to 8
move 1 from 7 to 8
move 3 from 3 to 2
move 1 from 8 to 2
move 1 from 3 to 7
move 13 from 1 to 4
move 3 from 5 to 3
move 1 from 1 to 2
move 1 from 8 to 5
move 5 from 7 to 2
move 1 from 6 to 5
move 2 from 3 to 4
move 10 from 2 to 5
move 1 from 9 to 5
move 3 from 1 to 8
move 3 from 8 to 3
move 11 from 4 to 5
move 12 from 2 to 8
move 4 from 4 to 7
move 10 from 8 to 5
move 2 from 8 to 1
move 1 from 7 to 3
move 1 from 7 to 9
move 5 from 3 to 7
move 1 from 9 to 4
move 7 from 7 to 6
move 13 from 5 to 8
move 6 from 6 to 7
move 5 from 7 to 4
move 1 from 6 to 4
move 2 from 4 to 9
move 1 from 7 to 9
move 3 from 4 to 3
move 1 from 3 to 6
move 4 from 5 to 7

");
}
