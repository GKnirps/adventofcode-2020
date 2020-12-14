use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;
    let instructions = parse_instructions(&content)?;

    let memory = run_instructions(&instructions);
    let sum_memory: u64 = memory.values().sum();
    println!(
        "The sum of all values in memory after running all instructions is {}",
        sum_memory
    );

    Ok(())
}

fn run_instructions(instructions: &[Instruction]) -> HashMap<u64, u64> {
    let mut result: HashMap<u64, u64> = HashMap::with_capacity(instructions.len());
    let mut current_mask = Mask {
        pos: u64::MAX,
        bits: 0,
    };

    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask) => current_mask = *mask,
            Instruction::Assign(assign) => {
                result.insert(assign.address, current_mask.apply(assign.value));
            }
        }
    }

    result
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Instruction {
    Mask(Mask),
    Assign(Assign),
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>, String> {
    input
        .split_terminator('\n')
        .map(parse_instruction)
        .collect()
}

fn parse_instruction(line: &str) -> Result<Instruction, String> {
    if line.starts_with("mem[") {
        parse_assign(line).map(Instruction::Assign)
    } else if line.starts_with("mask =") {
        parse_mask(line).map(Instruction::Mask)
    } else {
        Err(format!("Unknown instruction: {}", line))
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
struct Mask {
    pos: u64,
    bits: u64,
}

fn parse_mask(line: &str) -> Result<Mask, String> {
    let mut pos = 0;
    let mut bits = 0;

    for c in line.trim_start_matches("mask = ").chars() {
        pos <<= 1;
        bits <<= 1;
        match c {
            '1' => {
                bits += 1;
            }
            '0' => (),
            'X' => {
                pos += 1;
            }
            _ => return Err(format!("Invalid character in  mask: {}: {}", line, c)),
        }
    }

    Ok(Mask { pos, bits })
}

impl Mask {
    fn apply(&self, value: u64) -> u64 {
        (value & self.pos) | self.bits
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
struct Assign {
    address: u64,
    value: u64,
}

fn parse_assign(line: &str) -> Result<Assign, String> {
    let mut split = line.splitn(2, " = ");
    let left = split
        .next()
        .expect("Expected at least a left side")
        .trim_start_matches("mem[")
        .trim_end_matches(']');
    let right = split.next().ok_or_else(|| {
        format!(
            "Unable to parse assignment instruction {}: no right hand side.",
            line
        )
    })?;

    let address: u64 = left
        .parse()
        .map_err(|e| format!("Unable to parse address in line {}: {}", line, e))?;
    let value: u64 = right
        .parse()
        .map_err(|e| format!("Unable to parse value in line {}: {}", line, e))?;

    Ok(Assign { address, value })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mask_application_works_correctly() {
        // given
        let mask = parse_mask("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")
            .expect("Expected valid example mask");

        // when
        let result = mask.apply(11);

        // then
        assert_eq!(result, 73);
    }
}
