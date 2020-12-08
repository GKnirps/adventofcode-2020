use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() -> Result<(), String> {
    let filename = env::args()
        .nth(1)
        .ok_or_else(|| "No file name given.".to_owned())?;
    let content = read_to_string(&Path::new(&filename)).map_err(|e| e.to_string())?;

    let program = parse_ops(&content)?;

    let accumulator_after_loop = detect_loop(&program)?;
    println!("Detected loop, accumulator is {}", accumulator_after_loop);

    Ok(())
}

fn detect_loop(program: &[Op]) -> Result<i32, String> {
    if program.is_empty() {
        return Err("Program is empty".to_owned());
    }
    let mut state = State::default();
    let mut visited = vec![false; program.len()];

    while !visited[state.ip as usize] {
        visited[state.ip as usize] = true;
        state = run_instruction(program, &state)?;
        if state.ip < 0 || state.ip >= program.len() as i32 {
            return Err(format!(
                "Instruction pointer out of bounds in loop detection: {}",
                state.ip
            ));
        }
    }

    Ok(state.accumulator)
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug, Hash, Default)]
struct State {
    ip: i32,
    accumulator: i32,
}

fn run_instruction(program: &[Op], state: &State) -> Result<State, String> {
    let ip = state.ip;
    let accumulator = state.accumulator;
    if ip < 0 || ip >= program.len() as i32 {
        Err(format!("Instruction pointer out of bounds: {}", ip))
    } else {
        Ok(match program[ip as usize] {
            Op::Acc(arg) => State {
                ip: ip + 1,
                accumulator: accumulator + arg,
            },
            Op::Jmp(arg) => State {
                ip: ip + arg,
                accumulator,
            },
            Op::Nop(_) => State {
                ip: ip + 1,
                accumulator,
            },
        })
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn parse_ops(code: &str) -> Result<Vec<Op>, String> {
    code.split_terminator('\n').map(parse_op).collect()
}

fn parse_op(line: &str) -> Result<Op, String> {
    let mut s = line.splitn(2, ' ');
    let opcode_s = s.next().expect("Expected at least one string after split");
    let argument = s
        .next()
        .ok_or_else(|| format!("missing argument in line '{}'", line))
        .and_then(|a| {
            a.parse::<i32>()
                .map_err(|e| format!("Unable to parse argument in line '{}': {}", line, e))
        })?;

    match opcode_s {
        "acc" => Ok(Op::Acc(argument)),
        "jmp" => Ok(Op::Jmp(argument)),
        "nop" => Ok(Op::Nop(argument)),
        _ => Err(format!("Unknown opcode: '{}'", opcode_s)),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detect_loop_works_for_example() {
        // given
        let input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";
        let program = parse_ops(input).expect("Expected program to parse");

        // when
        let result = detect_loop(&program);

        // then
        let acc = result.expect("Expected loop detection to be successful");
        assert_eq!(acc, 5);
    }

    #[test]
    fn parse_ops_parses_valid_ops() {
        // given
        let input = "acc +4\nnop -3000\njmp -2\n";

        // when
        let result = parse_ops(input);

        // then
        let ops = result.expect("Expected valid parsing");
        assert_eq!(&ops, &[Op::Acc(4), Op::Nop(-3000), Op::Jmp(-2)]);
    }

    #[test]
    fn parse_ops_fails_for_missing_argument() {
        // given
        let input = "acc\n";

        // when
        let result = parse_ops(input);

        // then
        assert_eq!(result, Err("missing argument in line 'acc'".to_owned()))
    }

    #[test]
    fn parse_ops_fails_for_non_int_argument() {
        // given
        let input = "acc bananas\n";

        // when
        let result = parse_ops(input);

        // then
        assert_eq!(
            result,
            Err(
                "Unable to parse argument in line 'acc bananas': invalid digit found in string"
                    .to_owned()
            )
        )
    }

    #[test]
    fn parse_ops_fails_for_invalid_opcode() {
        // given
        let input = "foo +42\n";

        // when
        let result = parse_ops(input);

        // then
        assert_eq!(result, Err("Unknown opcode: 'foo'".to_owned()))
    }

    #[test]
    fn run_instruction_runs_acc_correctly() {
        // given
        let program = &[Op::Nop(11), Op::Acc(42)];
        let state = State {
            ip: 1,
            accumulator: 11,
        };

        // when
        let result = run_instruction(program, &state);

        // then
        let new_state = result.expect("Expected no error");
        assert_eq!(
            new_state,
            State {
                ip: 2,
                accumulator: 53
            }
        );
    }

    #[test]
    fn run_instruction_runs_jmp_correctly() {
        // given
        let program = &[Op::Nop(11), Op::Jmp(42)];
        let state = State {
            ip: 1,
            accumulator: 11,
        };

        // when
        let result = run_instruction(program, &state);

        // then
        let new_state = result.expect("Expected no error");
        assert_eq!(
            new_state,
            State {
                ip: 43,
                accumulator: 11
            }
        );
    }

    #[test]
    fn run_instruction_runs_nop_correctly() {
        // given
        let program = &[Op::Nop(11), Op::Nop(42)];
        let state = State {
            ip: 1,
            accumulator: 11,
        };

        // when
        let result = run_instruction(program, &state);

        // then
        let new_state = result.expect("Expected no error");
        assert_eq!(
            new_state,
            State {
                ip: 2,
                accumulator: 11
            }
        );
    }

    #[test]
    fn run_instruction_fails_for_invalid_ip() {
        // given
        let program = &[Op::Nop(42)];
        let state = State {
            ip: -1,
            accumulator: 0,
        };

        // when
        let result = run_instruction(program, &state);

        // then
        assert_eq!(
            result,
            Err("Instruction pointer out of bounds: -1".to_owned())
        );
    }
}
