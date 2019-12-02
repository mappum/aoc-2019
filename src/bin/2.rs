use std::io;

fn parse_ints(mut line: String) -> Vec<usize> {
    let mut mem = Vec::with_capacity(100);
    let mut value = String::new();

    line.push(',');

    for c in line.chars() {
        match c {
            ',' => {
                let int = value.trim().parse().unwrap();
                value.clear();
                mem.push(int);
            },
            _ => value.push(c)
        }
    }

    mem
}

fn run(mem: &mut [usize]) {
    for i in (0..mem.len()).step_by(4) {
        if mem[i] == 99 {
            return;
        }

        let a = mem[mem[i + 1]];
        let b = mem[mem[i + 2]];
        let dest = mem[i + 3];
        let res = match mem[i] {
            1 => a + b,
            2 => a * b,
            _ => panic!("invalid opcode")
        };
        mem[dest] = res;
    }
}

fn part_one() {
    let mut line = String::with_capacity(256);
    io::stdin().read_line(&mut line).unwrap();
    
    let mut ints = parse_ints(line);

    run(&mut ints);

    println!("\n{:?}", ints);
}

fn part_two() {
    let program = vec![1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,1,19,5,23,1,23,9,27,2,27,6,31,1,31,6,35,2,35,9,39,1,6,39,43,2,10,43,47,1,47,9,51,1,51,6,55,1,55,6,59,2,59,10,63,1,6,63,67,2,6,67,71,1,71,5,75,2,13,75,79,1,10,79,83,1,5,83,87,2,87,10,91,1,5,91,95,2,95,6,99,1,99,6,103,2,103,6,107,2,107,9,111,1,111,5,115,1,115,6,119,2,6,119,123,1,5,123,127,1,127,13,131,1,2,131,135,1,135,10,0,99,2,14,0,0];
    let target = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut ints = program.clone();
            ints[1] = noun;
            ints[2] = verb;

            run(&mut ints);

            if ints[0] == target {
                println!("noun={}, verb={}", noun, verb);
                std::process::exit(0);
            }
        }
    }
}

fn main() {
    part_two();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_test1() {
        let mut ints = parse_ints("1,0,0,0,99".to_string());
        run(&mut ints);
        assert_eq!(ints, vec![2,0,0,0,99])
    }

    #[test]
    fn day2_test2() {
        let mut ints = parse_ints("2,3,0,3,99".to_string());
        run(&mut ints);
        assert_eq!(ints, vec![2,3,0,6,99])
    }

    #[test]
    fn day2_test3() {
        let mut ints = parse_ints("2,4,4,5,99,0".to_string());
        run(&mut ints);
        assert_eq!(ints, vec![2,4,4,5,99,9801])
    }

    #[test]
    fn day2_test4() {
        let mut ints = parse_ints("1,1,1,4,99,5,6,0,99".to_string());
        run(&mut ints);
        assert_eq!(ints, vec![30,1,1,4,2,5,6,0,99])
    }
}
