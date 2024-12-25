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

fn parse_input(data: &str) -> (Vec<(Rc<Wire>, bool)>, Vec<Rc<Wire>>) {
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
        let output_name = split.next().unwrap();

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

    let mut outputs = wires
        .into_iter()
        .filter(|(name, _)| name.starts_with("z"))
        .collect::<Vec<_>>();
    outputs.sort_by_key(|(name, _)| name[1..].parse::<u32>().unwrap());

    (
        inputs,
        outputs
            .into_iter()
            .map(|(_, wire)| wire)
            .collect::<Vec<_>>(),
    )
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
    let (inputs, outputs) = parse_input(&data);

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
        let (inputs, outputs) = parse_input(&data);
        assert_eq!(apply_inputs_and_read_outputs(&inputs, &outputs), 2024);
    }

    #[test]
    fn answer_part1() {
        let data = read_to_string("src/main.txt").unwrap();
        let (inputs, outputs) = parse_input(&data);
        assert_eq!(apply_inputs_and_read_outputs(&inputs, &outputs), 0);
    }
}
