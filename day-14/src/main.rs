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

    let sum_memory_v2 = run_instructions_v2(&instructions);
    println!(
        "The sum of all values in memory after running all instructions on a v2 decoder is {}",
        sum_memory_v2
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

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
struct FloatingAddress {
    floating_bits: u64,
    bits: u64,
}

impl FloatingAddress {
    fn new(floating_bits: u64, bits: u64) -> Self {
        FloatingAddress {
            floating_bits,
            bits,
        }
    }

    fn cut(&self, addr2: &FloatingAddress) -> Option<FloatingAddress> {
        if self.bits & !addr2.floating_bits != addr2.bits & !self.floating_bits {
            return None;
        }
        let floating_bits = self.floating_bits & addr2.floating_bits;
        let bits = (self.bits | addr2.bits) & !floating_bits;

        Some(FloatingAddress::new(floating_bits, bits))
    }

    fn cardinality(&self) -> u64 {
        1 << self.floating_bits.count_ones()
    }
}

fn decode_address(raw: u64, mask: &Mask) -> FloatingAddress {
    FloatingAddress {
        floating_bits: mask.pos,
        bits: (raw & !mask.pos) | mask.bits,
    }
}

fn decode_instructions_v2(instructions: &[Instruction]) -> Vec<(FloatingAddress, u64)> {
    let mut result = Vec::with_capacity(instructions.len());
    let mut current_mask = Mask { pos: 0, bits: 0 };
    for instruction in instructions {
        match instruction {
            Instruction::Mask(mask) => current_mask = *mask,
            Instruction::Assign(assign) => {
                result.push((decode_address(assign.address, &current_mask), assign.value));
            }
        }
    }
    result
}

fn run_instructions_v2(instructions: &[Instruction]) -> u64 {
    let mut decoded_instructions = decode_instructions_v2(instructions);
    decoded_instructions.reverse();

    sum_values(
        &decoded_instructions,
        Some(FloatingAddress::new((1 << 36) - 1, 0)),
        1,
    ) as u64
}

fn sum_values(
    instructions: &[(FloatingAddress, u64)],
    cut: Option<FloatingAddress>,
    sign: i64,
) -> i64 {
    if cut.is_none() {
        return 0;
    }
    instructions
        .iter()
        .enumerate()
        .map(|(i, (addr, value))| {
            let next_cut = cut.and_then(|c| c.cut(addr));
            (next_cut.map(|c| c.cardinality()).unwrap_or(0) * value) as i64 * sign
                + sum_values(&instructions[i + 1..], next_cut, -sign)
        })
        .sum()
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

    #[test]
    fn run_instructions_v2_works_for_example() {
        // given
        let instructions = parse_instructions(
            r"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1",
        )
        .expect("Expected example program to be valid");

        // when
        let result = run_instructions_v2(&instructions);

        // then
        assert_eq!(result, 208);
    }
    #[test]
    fn run_instructions_v2_works_for_extended_example() {
        // given
        let instructions = parse_instructions(
            r"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
mem[26] = 1",
        )
        .expect("Expected example program to be valid");

        // when
        let result = run_instructions_v2(&instructions);

        // then
        assert_eq!(result, 208);
    }

    #[test]
    fn floating_address_cut_creates_cut_set_of_addresses() {
        assert_eq!(
            FloatingAddress::new(0, 1).cut(&FloatingAddress::new(0, 0)),
            None
        );
        assert_eq!(
            FloatingAddress::new(1, 0).cut(&FloatingAddress::new(0, 1)),
            Some(FloatingAddress::new(0, 1))
        );
        assert_eq!(
            FloatingAddress::new(0b10, 0b01).cut(&FloatingAddress::new(0b01, 0b10)),
            Some(FloatingAddress::new(0, 0b11))
        );
        assert_eq!(
            FloatingAddress::new(0b100, 0b010).cut(&FloatingAddress::new(0b010, 0b100)),
            Some(FloatingAddress::new(0, 0b110))
        );
        assert_eq!(
            FloatingAddress::new(0b11, 0b00).cut(&FloatingAddress::new(0b11, 0b00)),
            Some(FloatingAddress::new(0b11, 0b00))
        );
        assert_eq!(
            FloatingAddress::new(0b110, 0b001).cut(&FloatingAddress::new(0b110, 0b000)),
            None
        );
        assert_eq!(
            FloatingAddress::new(0b1100, 0b0011).cut(&FloatingAddress::new(0b1010, 0b0101)),
            Some(FloatingAddress::new(0b1000, 0b0111))
        );
    }
}
