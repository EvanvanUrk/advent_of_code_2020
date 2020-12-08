extern crate advent_of_code_2020;

use self::advent_of_code_2020::*;
use Instruction::*;

fn main() {
    let input = get_input("day_8.txt");

    let instructions = Console::parse_instructions(&input[..]);

    let mut console = Console::new(instructions.clone());
    console.run();
    println!("Accumulator after first cycle: {}", console.get_acc());

    for i in 0..instructions.len() {
        let mut copy = instructions.clone();
        match instructions[i] {
            Acc(_) => continue,
            Jmp(arg) => copy[i] = Nop(arg),
            Nop(arg) => copy[i] = Jmp(arg),
        }

        let mut console = Console::new(copy);
        console.run();

        if !console.get_looped() {
            println!("Accumulator after finishing: {}", console.get_acc());
            break;
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

struct Console {
    acc: i32,
    pointer: i32,
    instructions: Vec<Instruction>,
    looped: bool
}

impl Console {
    fn new(instructions: Vec<Instruction>) -> Console {
        Console {
            acc: 0,
            pointer: 0,
            instructions,
            looped: false,
        }
    }

    fn get_looped(&self) -> bool {
        self.looped
    }

    fn get_acc(&self) -> i32 {
        self.acc
    }

    fn run(&mut self) {
        let mut visited = vec![false; self.instructions.len()];
        while let Some(instruction) = self.instructions.get(self.pointer as usize) {
            visited[self.pointer as usize] = true;

            match instruction {
                Acc(arg) => self.acc += *arg,
                Jmp(arg) => self.pointer += *arg - 1,
                Nop(_) => {},
            };
            self.pointer += 1;

            if let Some(visited) = visited.get(self.pointer as usize) {
                if *visited {
                    self.pointer = -1;
                    self.looped = true;
                }
            }
        }
    }

    fn parse_instructions(code: &str) -> Vec<Instruction> {
        code.split('\n').map(|line| { Self::parse_instruction(line) }).collect()
    }

    fn parse_instruction(line: &str) -> Instruction {
        let parts: Vec<&str> = line.split(' ').collect();
        let arg = parts[1].parse().unwrap();

        match parts[0] {
            "nop" => Nop(arg),
            "acc" => Acc(arg),
            "jmp" => Jmp(arg),
            op => panic!("Unknown op: {}", op),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            Nop(0),
            Console::parse_instruction("nop +0")
        );

        assert_eq!(
            Acc(5),
            Console::parse_instruction("acc +5")
        );

        assert_eq!(
            Acc(-5),
            Console::parse_instruction("acc -5")
        );

        assert_eq!(
            Jmp(5),
            Console::parse_instruction("jmp +5")
        );

        assert_eq!(
            Jmp(-5),
            Console::parse_instruction("jmp -5")
        );
    }
}
