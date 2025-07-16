use regex::Regex;

struct CPU<'a> {
    instruction_pointer: usize,
    program: &'a Vec<usize>,
    registers: [usize; 3],
    output: Vec<usize>,
}

fn combo_operand(cpu: &CPU, operand: usize) -> usize {
    match operand {
        0..=3 => operand,
        4..=6 => cpu.registers[operand - 4],
        _ => panic!("Invalid combo operand"),
    }
}

const ADV: usize = 0;
const BXL: usize = 1;
const BST: usize = 2;
const JNZ: usize = 3;
const BXC: usize = 4;
const OUT: usize = 5;
const BDV: usize = 6;
const CDV: usize = 7;

fn execute_instruction(cpu: &mut CPU) {
    let operator = cpu.program[cpu.instruction_pointer];
    let operand = cpu.program[cpu.instruction_pointer + 1];
    cpu.instruction_pointer += 2;

    match operator {
        ADV => cpu.registers[0] = cpu.registers[0] >> combo_operand(cpu, operand),
        BXL => cpu.registers[1] = cpu.registers[1] ^ operand,
        BST => cpu.registers[1] = combo_operand(cpu, operand) % 8,
        JNZ => {
            if cpu.registers[0] != 0 {
                cpu.instruction_pointer = operand;
            }
        }
        BXC => cpu.registers[1] = cpu.registers[1] ^ cpu.registers[2],
        OUT => cpu.output.push(combo_operand(cpu, operand) % 8),
        BDV => cpu.registers[1] = cpu.registers[0] >> combo_operand(cpu, operand),
        CDV => cpu.registers[2] = cpu.registers[0] >> combo_operand(cpu, operand),
        _ => panic!("Invalid operator"),
    }
}

fn main() {
    let mut input = std::fs::read_to_string("input.txt").unwrap();
    input.pop();

    let (registers, program) = input.split_once("\n\n").unwrap();
    let re_registers = Regex::new(r"Register [A-C]: ([0-9]+)").unwrap();
    let _register_values: Vec<usize> = registers
        .lines()
        .map(|line| {
            re_registers
                .captures(line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()
                .unwrap()
        })
        .collect();

    let re_program = Regex::new(r"Program: ([0-9\,]+)").unwrap();
    let program = re_program
        .captures(program)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    let program: Vec<usize> = program
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let lsbs: usize = 0b0000010101000101010;
    let mut i: usize = 0;
    'outer: loop {
        i += 1;
        let init_a = (i << 19) | lsbs;
        //println!("{}", format!("{init_a:b}"));
        let mut cpu = CPU {
            registers: [init_a, 0, 0],
            instruction_pointer: 0,
            output: Vec::new(),
            program: &program,
        };

        while cpu.instruction_pointer < cpu.program.len() {
            execute_instruction(&mut cpu);
            if cpu.output.len() > cpu.program.len()
                || (!cpu.output.is_empty()
                    && *cpu.output.last().unwrap() != cpu.program[cpu.output.len() - 1])
            {
                continue 'outer;
            }
        }

        dbg!((init_a, cpu.output.len()));
        if cpu.output == *cpu.program {
            dbg!(init_a);
            break;
        }
    }
}
