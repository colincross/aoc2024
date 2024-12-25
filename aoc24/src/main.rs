use std::{cell::RefCell, collections::HashMap, fs::read_to_string, rc::Rc};

type GateOp = fn(bool, bool) -> bool;

struct GateImpl {
    in1: Option<bool>,
    in2: Option<bool>,
    out_wire: Rc<Wire>,
    op: GateOp,
}

struct Gate {
    gate: RefCell<GateImpl>,
}

impl GateImpl {
    fn update(&mut self) {
        if let Some(in1) = self.in1 {
            if let Some(in2) = self.in2 {
                let out = (self.op)(in1, in2);
                self.out_wire.set(out);
            }
        }
    }
}

impl Gate {
    fn new(op: GateOp, out_wire: Rc<Wire>) -> Self {
        let gate = GateImpl {
            in1: None,
            in2: None,
            out_wire,
            op,
        };
        Self { gate: gate.into() }
    }

    fn input(&self, input: Input, val: bool) {
        let mut gate = self.gate.borrow_mut();
        match input {
            Input::INPUT1 => {
                assert!(gate.in1.is_none());
                gate.in1 = Some(val);
            }
            Input::INPUT2 => {
                assert!(gate.in2.is_none());
                gate.in2 = Some(val);
            }
        }
        gate.update();
    }
}

#[derive(Default)]
struct Wire {
    propagate_gates: RefCell<Vec<(Rc<Gate>, Input)>>,
    val: RefCell<bool>,
}

#[derive(Clone, Copy)]
enum Input {
    INPUT1,
    INPUT2,
}

impl Wire {
    fn set(&self, val: bool) {
        *self.val.borrow_mut() = val;
        for (gate, input) in self.propagate_gates.borrow().iter() {
            gate.input(*input, val);
        }
    }

    fn get(&self) -> bool {
        *self.val.borrow()
    }

    fn add_propagate(&self, gate: Rc<Gate>, input: Input) {
        self.propagate_gates.borrow_mut().push((gate, input));
    }
}

fn or(a: bool, b: bool) -> bool {
    a || b
}

fn and(a: bool, b: bool) -> bool {
    a && b
}

fn xor(a: bool, b: bool) -> bool {
    a ^ b
}

fn apply_inputs(inputs: &[(Rc<Wire>, bool)]) {
    for (wire, val) in inputs {
        wire.set(*val);
    }
}

fn apply_inputs_and_read_outputs(inputs: &[(Rc<Wire>, bool)], outputs: &[Rc<Wire>]) -> u64 {
    apply_inputs(inputs);

    let mut n = 0;
    for (i, output) in outputs.iter().enumerate() {
        if output.get() {
            n |= 1 << i;
        }
    }
    n
}

fn generate_inputs(
    wires: &HashMap<String, Rc<Wire>>,
    x: u64,
    y: u64,
    n: usize,
) -> Vec<(Rc<Wire>, bool)> {
    let mut inputs = Vec::new();

    for i in 0..n {
        inputs.push((wires[&format!("x{:02}", i)].clone(), (x >> i) & 1 == 1));
        inputs.push((wires[&format!("y{:02}", i)].clone(), (y >> i) & 1 == 1));
    }

    inputs
}

fn swap_outputs<'a>(name: &'a str) -> &'a str {
    // fgt,fpq,nqk,pcp,srn,z07,z24,z32
    match name {
        "z07" => "nqk",
        "nqk" => "z07",

        "pcp" => "fgt",
        "fgt" => "pcp",

        "fpq" => "z24",
        "z24" => "fpq",

        "z32" => "srn",
        "srn" => "z32",
        _ => name,
    }
}

fn parse_input(
    data: &str,
) -> (
    HashMap<String, Rc<Wire>>,
    Vec<(Rc<Wire>, bool)>,
    Vec<Rc<Wire>>,
) {
    parse_input_with_swaps(data, |s| s)
}

fn parse_input_with_swaps(
    data: &str,
    swap: fn(&str) -> &str,
) -> (
    HashMap<String, Rc<Wire>>,
    Vec<(Rc<Wire>, bool)>,
    Vec<Rc<Wire>>,
) {
    let mut wires = HashMap::<String, Rc<Wire>>::new();

    let mut lines = data.lines();
    let inputs_iter = lines.by_ref().take_while(|line| !line.is_empty());

    let mut inputs = Vec::<(Rc<Wire>, bool)>::new();
    for input_line in inputs_iter {
        let (name, val) = input_line.split_once(": ").unwrap();
        let wire = wires.entry(name.to_owned()).or_default();
        inputs.push((wire.clone(), val.parse::<u8>().unwrap() == 1));
    }

    for gate_line in lines {
        let mut split = gate_line.split_ascii_whitespace();
        let input1_name = split.next().unwrap();
        let operation = split.next().unwrap();
        let input2_name = split.next().unwrap();
        split.next();
        let output_name = swap(split.next().unwrap());

        let output = wires.entry(output_name.to_owned()).or_default();

        let op = match operation {
            "AND" => and,
            "OR" => or,
            "XOR" => xor,
            _ => panic!(),
        };

        let gate = Rc::new(Gate::new(op, output.clone()));

        let input1 = wires.entry(input1_name.to_owned()).or_default();
        input1.add_propagate(gate.clone(), Input::INPUT1);

        let input2 = wires.entry(input2_name.to_owned()).or_default();
        input2.add_propagate(gate.clone(), Input::INPUT2);
    }

    let mut output_wires = wires
        .iter()
        .filter(|&(name, _)| name.starts_with("z"))
        .collect::<Vec<_>>();
    output_wires.sort_by_key(|(name, _)| name[1..].parse::<u32>().unwrap());
    let outputs = output_wires
        .into_iter()
        .map(|(_, wire)| wire.clone())
        .collect::<Vec<_>>();

    (wires, inputs, outputs)
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let input_file = if args.len() >= 2 {
        std::path::PathBuf::from(&args[1])
    } else {
        let exe = std::env::current_exe().unwrap();
        exe.parent()
            .unwrap()
            .join("../..")
            .join(exe.file_name().unwrap())
            .join("src/main.txt")
    };
    let data = read_to_string(&input_file).unwrap();
    let (_, inputs, outputs) = parse_input(&data);

    println!(
        "output: {}",
        apply_inputs_and_read_outputs(&inputs, &outputs),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = read_to_string("src/test.txt").unwrap();
        let (_, inputs, outputs) = parse_input(&data);
        assert_eq!(apply_inputs_and_read_outputs(&inputs, &outputs), 2024);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let (_, inputs, outputs) = parse_input(&data);
        assert_eq!(
            apply_inputs_and_read_outputs(&inputs, &outputs),
            61886126253040
        );
    }

    #[test]
    fn answer_part2() {
        let data = read_to_string("src/main.txt").unwrap();
        let (wires, _, outputs) = parse_input_with_swaps(&data, swap_outputs);
        let a = apply_inputs_and_read_outputs(&generate_inputs(&wires, 0, 0, 45), &outputs);
        assert_eq!(a, 0, "expect 0 + 0 = 0");
        for i in 0..45 {
            let (wires, _, outputs) = parse_input_with_swaps(&data, swap_outputs);
            let a: u64 =
                apply_inputs_and_read_outputs(&generate_inputs(&wires, 1 << i, 0, 45), &outputs);
            assert_eq!(a, 1u64 << i, "expect {:x} + 0 = {:x}", 1u64 << i, 1u64 << i);

            let (wires, _, outputs) = parse_input_with_swaps(&data, swap_outputs);
            let a: u64 =
                apply_inputs_and_read_outputs(&generate_inputs(&wires, 0, 1 << i, 45), &outputs);
            assert_eq!(a, 1u64 << i, "expect 0 + {:x} = {:x}", 1u64 << i, 1u64 << i);
        }
        for i in 0..44 {
            let (wires, _, outputs) = parse_input_with_swaps(&data, swap_outputs);
            let a: u64 = apply_inputs_and_read_outputs(
                &generate_inputs(&wires, 3 << i, 1 << i, 45),
                &outputs,
            );
            assert_eq!(
                a,
                4u64 << i,
                "expect {:x} + {:x} = {:x}",
                (3u64 << i),
                (1u64 << i),
                (4u64 << i),
            );

            let (wires, _, outputs) = parse_input_with_swaps(&data, swap_outputs);
            let a: u64 = apply_inputs_and_read_outputs(
                &generate_inputs(&wires, 1 << i, 3 << i, 45),
                &outputs,
            );
            assert_eq!(
                a,
                4u64 << i,
                "expect {:x} + {:x} = {:x}",
                1u64 << i,
                3u64 << i,
                4u64 << i
            );
        }
    }
}
