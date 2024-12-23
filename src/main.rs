use colored::*;
use csv::Reader;
use csv::StringRecord;
use prettytable::{format, Cell, Row, Table};
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, Write};

#[derive(PartialEq)]
struct LHS {
    state: String,
    input: String,
}

#[derive(PartialEq)]
struct RHS {
    state: String,
    replacement: String,
    direction: char,
}

#[derive(PartialEq)]
struct TransitionFunction {
    lhs: LHS,
    rhs: RHS,
}

#[derive(PartialEq)]
struct KeyStates {
    initial_state: String,
    final_states: Vec<String>,
}

struct Machine {
    transitions: Vec<TransitionFunction>,
    states: KeyStates,
    tracks: usize,
}

// main
fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    match arg_len {
        1 => default_behaviour(),
        3 => option_behaviour(args),
        2 | _ => help_behaviour(args),
    }
}

// main functions
fn default_behaviour() {
    println!("Turing Machine Simulator");
    print!("Number of tracks: ");
    io::stdout().flush().expect("failed to flush");
    let tracks: usize = get_input().trim().parse().unwrap();
    let transitions = get_transitions(tracks);
    let states = get_states(&transitions);
    let turing_machine = Machine {
        transitions: transitions,
        states: states,
        tracks: tracks,
    };
    loop {
        print!("Track 1: ");
        io::stdout().flush().expect("failed to flush");
        let mut inputs: Vec<String> = vec![get_input().trim().to_string().replace("\r\n", "")];

        // Get the remaining inputs, checking if they have the same length as the first one
        for i in 1..tracks {
            loop {
                print!("Tape (Track {}): ", i + 1);
                io::stdout().flush().expect("failed to flush");
                let input = get_input().trim().to_string().replace("\r\n", "");
                if input.len() == inputs[0].len() {
                    inputs.push(input);
                    break;
                } else {
                    println!("Error: Tracks must have the same length");
                }
            }
        }

        // Combine the inputs
        let combined: String = (0..inputs[0].len())
            .map(|i| {
                inputs
                    .iter()
                    .map(|s| s.chars().nth(i).unwrap())
                    .collect::<String>()
            })
            .collect();
        parse(
            combined,
            &turing_machine.transitions,
            &turing_machine.states,
            turing_machine.tracks,
        );
        println!("Parse another string? (Y/N)");
        if get_input()
            .trim()
            .to_string()
            .replace("\r\n", "")
            .to_uppercase()
            == "Y"
        {
            continue;
        } else {
            break;
        }
    }
}

fn help_behaviour(args: Vec<String>) {
    if args[1] != "-help" {
        println!("{}", "Error: Unkown option".red());
    }
    println!("Turing Machine Simulator");
    println!("Usage:\ncargo run -- <option>\nturing_sim.exe <option>");
    println!("\nOptions:");
    println!("-help : Shows help menu");
    println!("-csv <path> : loads transition functions from csv file");
    println!("-demo 0 : translates every 'a' to 'b'");
    println!("-demo 1 : accepts strings in form of a(n)b(n)");
    println!("-demo 2 : copies strings of '1'");
    println!("-demo 3 : checks two tracks of 'a' & 'b' and finds where tracks match");
    println!("-demo 4 : a turing machine for multiplication (e.g. input: 11*11)");
    println!("\n*Run without options to input your own turing machine")
}

fn option_behaviour(args: Vec<String>) {
    if args[1] == "-demo" {
        demo_behaviour(args);
    } else if args[1] == "-csv" {
        if let Err(e) = csv_behaviour(args) {
            eprintln!("Error: {}", e);
        }
    } else {
        help_behaviour(args);
    }
}

fn demo_behaviour(args: Vec<String>) {
    let demo_index: usize = match args[2].parse() {
        Ok(idx) => idx,
        Err(_) => {
            println!("Invalid demo index");
            return;
        }
    };
    let dem = demos();
    if demo_index < dem.len() {
        let demo = &dem[demo_index];
        loop {
            print!("Track 1: ");
            io::stdout().flush().expect("failed to flush");
            let mut inputs: Vec<String> = vec![get_input().trim().to_string().replace("\r\n", "")];
            // Get the remaining inputs, checking if they have the same length as the first one
            for i in 1..demo.tracks {
                loop {
                    print!("Track {}: ", i + 1);
                    io::stdout().flush().expect("failed to flush");
                    let input = get_input().trim().to_string().replace("\r\n", "");
                    if input.len() == inputs[0].len() {
                        inputs.push(input);
                        break;
                    } else {
                        println!("Error: Tracks must have the same length");
                    }
                }
            }
            // Combine the inputs
            let combined: String = (0..inputs[0].len())
                .map(|i| {
                    inputs
                        .iter()
                        .map(|s| s.chars().nth(i).unwrap())
                        .collect::<String>()
                })
                .collect();
            parse(combined, &demo.transitions, &demo.states, demo.tracks);
            println!("Parse another string? (Y/N)");
            if get_input()
                .trim()
                .to_string()
                .replace("\r\n", "")
                .to_uppercase()
                == "Y"
            {
                continue;
            } else {
                break;
            }
        }
    } else {
        println!("Demo index out of bounds");
    }
}

fn csv_behaviour(args: Vec<String>) -> io::Result<()> {
    println!("Turing Machine Simulator");
    print!("Number of tracks: ");
    io::stdout().flush().expect("failed to flush");

    let file_path = args[2].clone();
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);
    let headers = rdr.headers()?;
    let valid_headers = StringRecord::from(vec![
        "lhs_state",
        "input",
        "rhs_state",
        "replacement",
        "direction",
    ]);
    if headers.clone() != valid_headers {
        println!("Invalid headers.\n Headers must be \"lhs_state\", \"input\", \"rhs_state\", \"replacement\", \"direction\"");
        return Err(io::Error::new(io::ErrorKind::Other, "Invalid headers"));
    }

    let tracks: usize = get_input().trim().parse().unwrap();
    let mut functions: Vec<TransitionFunction> = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let Some(lhs_state) = record.get(0) else {
            panic!("No value found")
        };
        let Some(input) = record.get(1) else {
            panic!("No value found")
        };
        let Some(rhs_state) = record.get(2) else {
            panic!("No value found")
        };
        let Some(replacement) = record.get(3) else {
            panic!("No value found")
        };
        let Some(direction_str) = record.get(4) else {
            panic!("No value found")
        };
        let direction: Vec<char> = direction_str.to_string().chars().collect();
        if direction.len() != 1 {
            println!("Invalid Direction (Direction is more than a character)");
            println!("{:?}", record);
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Invalid Data in direction field",
            ));
        }
        if (input != "□".repeat(tracks) && input.len() != tracks)
            || (replacement != "□".repeat(tracks) && replacement.len() != tracks)
        {
            println!("Symbol length does not match number of tracks");
            println!("{:?}", record);
            return Err(io::Error::new(io::ErrorKind::Other, "Mismatch length"));
        }
        let direction_char: Vec<char> = direction[0].to_uppercase().collect();
        if direction_char[0] == 'R' || direction_char[0] == 'L' {
            let lhs = LHS {
                state: lhs_state.to_string(),
                input: input.to_string(),
            };
            let rhs = RHS {
                state: rhs_state.to_string(),
                replacement: replacement.to_string(),
                direction: direction_char[0],
            };
            let current_function = TransitionFunction { lhs, rhs };
            println!(
                "Transition function: δ({},{})=({},{},{})",
                current_function.lhs.state,
                current_function.lhs.input,
                current_function.rhs.state,
                current_function.rhs.replacement,
                current_function.rhs.direction
            );
            functions.push(current_function);
        } else {
            println!("Invalid Direction (Direction is not L or R)");
            println!("{:?}", record);
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Invalid Data in direction field",
            ));
        }
    }
    let key_states = get_states(&functions);
    let turing_machine = Machine {
        transitions: functions,
        tracks: tracks,
        states: key_states,
    };

    loop {
        print!("Track 1: ");
        io::stdout().flush().expect("failed to flush");
        let mut inputs: Vec<String> = vec![get_input().trim().to_string().replace("\r\n", "")];

        // Get the remaining inputs, checking if they have the same length as the first one
        for i in 1..tracks {
            loop {
                print!("Tape (Track {}): ", i + 1);
                io::stdout().flush().expect("failed to flush");
                let input = get_input().trim().to_string().replace("\r\n", "");
                if input.len() == inputs[0].len() {
                    inputs.push(input);
                    break;
                } else {
                    println!("Error: Tracks must have the same length");
                }
            }
        }

        // Combine the inputs
        let combined: String = (0..inputs[0].len())
            .map(|i| {
                inputs
                    .iter()
                    .map(|s| s.chars().nth(i).unwrap())
                    .collect::<String>()
            })
            .collect();
        parse(
            combined,
            &turing_machine.transitions,
            &turing_machine.states,
            turing_machine.tracks,
        );
        println!("Parse another string? (Y/N)");
        if get_input()
            .trim()
            .to_string()
            .replace("\r\n", "")
            .to_uppercase()
            == "Y"
        {
            continue;
        } else {
            break;
        }
    }

    Ok(())
}

fn get_transitions(chunk: usize) -> Vec<TransitionFunction> {
    let mut functions = Vec::new();

    println!("Enter functions e.g δ(q1,a)=(q2,b,L) [enter 'END' if you don't want to add anymore functions]: ");
    println!("*you can use 'blank' instead of □");

    loop {
        print!("δ");
        io::stdout().flush().expect("failed to flush");
        let func = get_input();
        if func.trim().to_uppercase() == "END" {
            break;
        }

        let func = func.replace(" ", "");
        if !function_validator(&func) {
            println!("invalid format... (function was not added)");
            continue;
        }
        let func = func.replace("blank", "□");
        let parts: Vec<&str> = func.split('=').collect();

        let lhs_parts: Vec<&str> = parts[0]
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .map(|s| s.trim())
            .collect();

        let lhs = LHS {
            state: lhs_parts[0].to_string(),
            input: lhs_parts[1].to_string(),
        };

        let rhs_parts: Vec<&str> = parts[1]
            .trim_matches(|c| c == '(' || c == ')' || c == '\r' || c == '\n')
            .split(',')
            .map(|s| s.trim())
            .collect();

        let rhs = RHS {
            state: rhs_parts[0].to_string(),
            replacement: rhs_parts[1].to_string(),
            direction: rhs_parts[2].to_uppercase().chars().next().unwrap(),
        };

        if (lhs.input != "□".repeat(chunk) && lhs.input.len() != chunk)
            || (rhs.replacement != "□".repeat(chunk) && rhs.replacement.len() != chunk)
        {
            println!("invalid format... length mismatch (function was not added)");
            continue;
        }
        let current_function = TransitionFunction { lhs, rhs };
        functions.push(current_function);
    }

    functions
}

fn get_states(transitions: &[TransitionFunction]) -> KeyStates {
    println!("Enter initial state e.g. q0:");
    let initial: String;
    loop {
        let init = get_input().trim().to_string();
        if state_validator(&init, transitions) {
            initial = init;
            break;
        } else {
            println!("Invalid initial state")
        }
    }

    println!(
        "Enter final state e.g. q1 [Enter 'END' if you don't want to add anymore final states]: "
    );
    let mut finals: Vec<String> = Vec::new();
    loop {
        let fin = get_input().trim().to_string();
        if fin.to_uppercase() == "END" {
            break;
        }
        if state_validator(&fin, transitions) {
            finals.push(fin);
        } else {
            println!("Invalid final state")
        }
    }
    let states = KeyStates {
        initial_state: initial,
        final_states: finals,
    };

    states
}

fn parse(input: String, transitions: &[TransitionFunction], states: &KeyStates, chunk: usize) {
    println!("\nparsing...");
    let mut input: Vec<String> = input
        .chars()
        .collect::<String>()
        .chars()
        .collect::<Vec<char>>()
        .chunks(chunk)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();
    input.push(String::from("□").repeat(chunk));
    input.insert(0, String::from("□").repeat(chunk));
    let mut i = 1;
    let mut current_state = states.initial_state.clone();

    loop {
        let mut table = Table::new();
        table.set_format(
            format::FormatBuilder::new()
                .column_separator('│')
                .borders('│')
                .separators(
                    &[format::LinePosition::Top],
                    format::LineSeparator::new('─', '┬', '┌', '┐'),
                )
                .separators(
                    &[format::LinePosition::Intern],
                    format::LineSeparator::new('─', '┼', '├', '┤'),
                )
                .separators(
                    &[format::LinePosition::Bottom],
                    format::LineSeparator::new('─', '┴', '└', '┘'),
                )
                .padding(1, 1)
                .build(),
        );
        let current_input = input[i].clone();

        let mut head_row = Row::new(vec![Cell::new("HEAD")]);
        for (index, _) in input.iter().enumerate() {
            let head_symbol = if index == i { "▼" } else { " " };
            head_row.add_cell(Cell::new(head_symbol));
        }
        table.add_row(head_row);

        for n in 0..chunk {
            let mut tape_row = Row::new(vec![Cell::new("TAPE")]);
            for s in &input {
                let char_at_index = s.chars().nth(n);
                if let Some(c) = char_at_index {
                    tape_row.add_cell(Cell::new(c.to_string().as_str()));
                }
            }
            table.add_row(tape_row);
        }

        table.printstd();

        let lhs_to_find = LHS {
            state: current_state.clone(),
            input: current_input.clone(),
        };
        println!("Current state: {}", current_state);
        println!("Current input: '{}', Head position: {}", current_input, i);
        if let Some(transition) = transitions.iter().find(|t| t.lhs == lhs_to_find) {
            println!(
                "Transition function: δ({},{})=({},{},{})\n",
                transition.lhs.state,
                transition.lhs.input,
                transition.rhs.state,
                transition.rhs.replacement,
                transition.rhs.direction
            );
            current_state = transition.rhs.state.clone();
            input[i] = transition.rhs.replacement.clone();
            if transition.rhs.direction == 'L' {
                if i == 0 {
                    input.insert(0, String::from("□").repeat(chunk));
                } else {
                    i = i - 1;
                }
            } else if transition.rhs.direction == 'R' {
                i = i + 1;
                if i == input.len() {
                    input.push(String::from("□").repeat(chunk));
                }
            }
        } else {
            break;
        }
    }

    if states.final_states.contains(&current_state) {
        println!("{}", "Success".green());
    } else {
        println!("{}", "Failure".red());
    }
}

// helper functions
fn function_validator(function: &str) -> bool {
    let re = Regex::new(r"\(.*\,(.*)\)\=\(.*\,(.*)\,(L|R|l|r)\)").unwrap();
    re.is_match(function)
}

fn state_validator(state: &str, transitions: &[TransitionFunction]) -> bool {
    transitions
        .iter()
        .any(|transition| transition.lhs.state == state || transition.rhs.state == state)
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error occurred");
    input
}

// demos
fn demos() -> Vec<Machine> {
    let f1 = TransitionFunction {
        lhs: LHS {
            state: String::from("q0"),
            input: String::from("a"),
        },
        rhs: RHS {
            state: String::from("q0"),
            replacement: String::from("b"),
            direction: 'R',
        },
    };

    let f2 = TransitionFunction {
        lhs: LHS {
            state: String::from("q0"),
            input: String::from("b"),
        },
        rhs: RHS {
            state: String::from("q0"),
            replacement: String::from("b"),
            direction: 'R',
        },
    };

    let f3 = TransitionFunction {
        lhs: LHS {
            state: String::from("q0"),
            input: String::from("□"),
        },
        rhs: RHS {
            state: String::from("q1"),
            replacement: String::from("□"),
            direction: 'L',
        },
    };

    let s1 = KeyStates {
        initial_state: String::from("q0"),
        final_states: vec![String::from("q1")],
    };
    let functions_translator = vec![f1, f2, f3];

    let demo0 = Machine {
        transitions: functions_translator,
        states: s1,
        tracks: 1,
    };

    let f4 = TransitionFunction {
        lhs: LHS {
            state: String::from("q0"),
            input: String::from("a"),
        },
        rhs: RHS {
            state: String::from("q1"),
            replacement: String::from("x"),
            direction: 'R',
        },
    };

    let f5 = TransitionFunction {
        lhs: LHS {
            state: String::from("q1"),
            input: String::from("a"),
        },
        rhs: RHS {
            state: String::from("q1"),
            replacement: String::from("a"),
            direction: 'R',
        },
    };

    let f6 = TransitionFunction {
        lhs: LHS {
            state: String::from("q1"),
            input: String::from("y"),
        },
        rhs: RHS {
            state: String::from("q1"),
            replacement: String::from("y"),
            direction: 'R',
        },
    };

    let f7 = TransitionFunction {
        lhs: LHS {
            state: String::from("q1"),
            input: String::from("b"),
        },
        rhs: RHS {
            state: String::from("q2"),
            replacement: String::from("y"),
            direction: 'L',
        },
    };

    let f8 = TransitionFunction {
        lhs: LHS {
            state: String::from("q2"),
            input: String::from("y"),
        },
        rhs: RHS {
            state: String::from("q2"),
            replacement: String::from("y"),
            direction: 'L',
        },
    };

    let f9 = TransitionFunction {
        lhs: LHS {
            state: String::from("q2"),
            input: String::from("a"),
        },
        rhs: RHS {
            state: String::from("q2"),
            replacement: String::from("a"),
            direction: 'L',
        },
    };

    let f10 = TransitionFunction {
        lhs: LHS {
            state: String::from("q2"),
            input: String::from("x"),
        },
        rhs: RHS {
            state: String::from("q0"),
            replacement: String::from("x"),
            direction: 'R',
        },
    };
    let f11 = TransitionFunction {
        lhs: LHS {
            state: String::from("q0"),
            input: String::from("y"),
        },
        rhs: RHS {
            state: String::from("q3"),
            replacement: String::from("y"),
            direction: 'R',
        },
    };

    let f12 = TransitionFunction {
        lhs: LHS {
            state: String::from("q3"),
            input: String::from("y"),
        },
        rhs: RHS {
            state: String::from("q3"),
            replacement: String::from("y"),
            direction: 'R',
        },
    };

    let f13 = TransitionFunction {
        lhs: LHS {
            state: String::from("q3"),
            input: String::from("□"),
        },
        rhs: RHS {
            state: String::from("q4"),
            replacement: String::from("□"),
            direction: 'L',
        },
    };

    let s2 = KeyStates {
        initial_state: String::from("q0"),
        final_states: vec![String::from("q4")],
    };

    let functions_accepter = vec![f4, f5, f6, f7, f8, f9, f10, f11, f12, f13];

    let demo1 = Machine {
        transitions: functions_accepter,
        states: s2,
        tracks: 1,
    };

    let f14 = TransitionFunction {
        lhs: LHS {
            state: String::from("q0"),
            input: String::from("1"),
        },
        rhs: RHS {
            state: String::from("q0"),
            replacement: String::from("x"),
            direction: 'R',
        },
    };

    let f15 = TransitionFunction {
        lhs: LHS {
            state: String::from("q0"),
            input: String::from("□"),
        },
        rhs: RHS {
            state: String::from("q1"),
            replacement: String::from("□"),
            direction: 'L',
        },
    };

    let f16 = TransitionFunction {
        lhs: LHS {
            state: String::from("q1"),
            input: String::from("1"),
        },
        rhs: RHS {
            state: String::from("q1"),
            replacement: String::from("1"),
            direction: 'L',
        },
    };

    let f17 = TransitionFunction {
        lhs: LHS {
            state: String::from("q1"),
            input: String::from("□"),
        },
        rhs: RHS {
            state: String::from("q3"),
            replacement: String::from("□"),
            direction: 'R',
        },
    };

    let f18 = TransitionFunction {
        lhs: LHS {
            state: String::from("q1"),
            input: String::from("x"),
        },
        rhs: RHS {
            state: String::from("q2"),
            replacement: String::from("1"),
            direction: 'R',
        },
    };

    let f19 = TransitionFunction {
        lhs: LHS {
            state: String::from("q2"),
            input: String::from("1"),
        },
        rhs: RHS {
            state: String::from("q2"),
            replacement: String::from("1"),
            direction: 'R',
        },
    };

    let f20 = TransitionFunction {
        lhs: LHS {
            state: String::from("q2"),
            input: String::from("□"),
        },
        rhs: RHS {
            state: String::from("q1"),
            replacement: String::from("1"),
            direction: 'L',
        },
    };

    let s3 = KeyStates {
        initial_state: String::from("q0"),
        final_states: vec![String::from("q3")],
    };

    let functions_copier = vec![f14, f15, f16, f17, f18, f19, f20];

    let demo2 = Machine {
        transitions: functions_copier,
        states: s3,
        tracks: 1,
    };
    let f21 = TransitionFunction {
        lhs: LHS {
            state: String::from("q0"),
            input: String::from("aa"),
        },
        rhs: RHS {
            state: String::from("q0"),
            replacement: String::from("11"),
            direction: 'R',
        },
    };

    let f22 = TransitionFunction {
        lhs: LHS {
            state: String::from("q0"),
            input: String::from("bb"),
        },
        rhs: RHS {
            state: String::from("q0"),
            replacement: String::from("11"),
            direction: 'R',
        },
    };

    let f23 = TransitionFunction {
        lhs: LHS {
            state: String::from("q0"),
            input: String::from("ab"),
        },
        rhs: RHS {
            state: String::from("q0"),
            replacement: String::from("ab"),
            direction: 'R',
        },
    };
    let f24 = TransitionFunction {
        lhs: LHS {
            state: String::from("q0"),
            input: String::from("ba"),
        },
        rhs: RHS {
            state: String::from("q0"),
            replacement: String::from("ba"),
            direction: 'R',
        },
    };
    let f25 = TransitionFunction {
        lhs: LHS {
            state: String::from("q0"),
            input: String::from("□□"),
        },
        rhs: RHS {
            state: String::from("q1"),
            replacement: String::from("□□"),
            direction: 'L',
        },
    };

    let s4 = KeyStates {
        initial_state: String::from("q0"),
        final_states: vec![String::from("q1")],
    };
    let functions_translator1 = vec![f21, f22, f23, f24, f25];

    let demo3 = Machine {
        transitions: functions_translator1,
        states: s4,
        tracks: 2,
    };
    // ref: https://www.geeksforgeeks.org/turing-machine-for-multiplication/
    let f26 = TransitionFunction {
        lhs: LHS {
            state: String::from("q0"),
            input: String::from("1"),
        },
        rhs: RHS {
            state: String::from("q0"),
            replacement: String::from("1"),
            direction: 'R',
        },
    };
    let f27 = TransitionFunction {
        lhs: LHS {
            state: String::from("q0"),
            input: String::from("*"),
        },
        rhs: RHS {
            state: String::from("q1"),
            replacement: String::from("*"),
            direction: 'R',
        },
    };
    let f28 = TransitionFunction {
        lhs: LHS {
            state: String::from("q1"),
            input: String::from("1"),
        },
        rhs: RHS {
            state: String::from("q1"),
            replacement: String::from("1"),
            direction: 'R',
        },
    };
    let f29 = TransitionFunction {
        lhs: LHS {
            state: String::from("q1"),
            input: String::from("□"),
        },
        rhs: RHS {
            state: String::from("q2"),
            replacement: String::from("*"),
            direction: 'L',
        },
    };
    let f30 = TransitionFunction {
        lhs: LHS {
            state: String::from("q2"),
            input: String::from("1"),
        },
        rhs: RHS {
            state: String::from("q2"),
            replacement: String::from("1"),
            direction: 'L',
        },
    };
    let f31 = TransitionFunction {
        lhs: LHS {
            state: String::from("q2"),
            input: String::from("*"),
        },
        rhs: RHS {
            state: String::from("q3"),
            replacement: String::from("*"),
            direction: 'R',
        },
    };
    let f32 = TransitionFunction {
        lhs: LHS {
            state: String::from("q3"),
            input: String::from("X"),
        },
        rhs: RHS {
            state: String::from("q3"),
            replacement: String::from("X"),
            direction: 'R',
        },
    };
    let f33 = TransitionFunction {
        lhs: LHS {
            state: String::from("q3"),
            input: String::from("1"),
        },
        rhs: RHS {
            state: String::from("q4"),
            replacement: String::from("X"),
            direction: 'L',
        },
    };
    let f34 = TransitionFunction {
        lhs: LHS {
            state: String::from("q3"),
            input: String::from("*"),
        },
        rhs: RHS {
            state: String::from("q12"),
            replacement: String::from("□"),
            direction: 'R',
        },
    };
    let f35 = TransitionFunction {
        lhs: LHS {
            state: String::from("q4"),
            input: String::from("X"),
        },
        rhs: RHS {
            state: String::from("q4"),
            replacement: String::from("X"),
            direction: 'L',
        },
    };
    let f36 = TransitionFunction {
        lhs: LHS {
            state: String::from("q4"),
            input: String::from("*"),
        },
        rhs: RHS {
            state: String::from("q5"),
            replacement: String::from("*"),
            direction: 'L',
        },
    };
    let f37 = TransitionFunction {
        lhs: LHS {
            state: String::from("q5"),
            input: String::from("Y"),
        },
        rhs: RHS {
            state: String::from("q5"),
            replacement: String::from("Y"),
            direction: 'L',
        },
    };
    let f38 = TransitionFunction {
        lhs: LHS {
            state: String::from("q5"),
            input: String::from("1"),
        },
        rhs: RHS {
            state: String::from("q6"),
            replacement: String::from("Y"),
            direction: 'R',
        },
    };
    let f39 = TransitionFunction {
        lhs: LHS {
            state: String::from("q5"),
            input: String::from("□"),
        },
        rhs: RHS {
            state: String::from("q11"),
            replacement: String::from("□"),
            direction: 'R',
        },
    };
    let f40 = TransitionFunction {
        lhs: LHS {
            state: String::from("q6"),
            input: String::from("Y"),
        },
        rhs: RHS {
            state: String::from("q6"),
            replacement: String::from("Y"),
            direction: 'R',
        },
    };
    let f41 = TransitionFunction {
        lhs: LHS {
            state: String::from("q6"),
            input: String::from("*"),
        },
        rhs: RHS {
            state: String::from("q7"),
            replacement: String::from("*"),
            direction: 'R',
        },
    };
    let f42 = TransitionFunction {
        lhs: LHS {
            state: String::from("q7"),
            input: String::from("1"),
        },
        rhs: RHS {
            state: String::from("q7"),
            replacement: String::from("1"),
            direction: 'R',
        },
    };
    let f43 = TransitionFunction {
        lhs: LHS {
            state: String::from("q7"),
            input: String::from("X"),
        },
        rhs: RHS {
            state: String::from("q7"),
            replacement: String::from("X"),
            direction: 'R',
        },
    };
    let f44 = TransitionFunction {
        lhs: LHS {
            state: String::from("q7"),
            input: String::from("*"),
        },
        rhs: RHS {
            state: String::from("q8"),
            replacement: String::from("*"),
            direction: 'R',
        },
    };
    let f45 = TransitionFunction {
        lhs: LHS {
            state: String::from("q8"),
            input: String::from("1"),
        },
        rhs: RHS {
            state: String::from("q8"),
            replacement: String::from("1"),
            direction: 'R',
        },
    };
    let f46 = TransitionFunction {
        lhs: LHS {
            state: String::from("q8"),
            input: String::from("□"),
        },
        rhs: RHS {
            state: String::from("q9"),
            replacement: String::from("1"),
            direction: 'L',
        },
    };
    let f47 = TransitionFunction {
        lhs: LHS {
            state: String::from("q9"),
            input: String::from("1"),
        },
        rhs: RHS {
            state: String::from("q9"),
            replacement: String::from("1"),
            direction: 'L',
        },
    };
    let f48 = TransitionFunction {
        lhs: LHS {
            state: String::from("q9"),
            input: String::from("*"),
        },
        rhs: RHS {
            state: String::from("q10"),
            replacement: String::from("*"),
            direction: 'L',
        },
    };
    let f49 = TransitionFunction {
        lhs: LHS {
            state: String::from("q10"),
            input: String::from("1"),
        },
        rhs: RHS {
            state: String::from("q10"),
            replacement: String::from("1"),
            direction: 'L',
        },
    };
    let f50 = TransitionFunction {
        lhs: LHS {
            state: String::from("q10"),
            input: String::from("X"),
        },
        rhs: RHS {
            state: String::from("q10"),
            replacement: String::from("X"),
            direction: 'L',
        },
    };
    let f51 = TransitionFunction {
        lhs: LHS {
            state: String::from("q10"),
            input: String::from("*"),
        },
        rhs: RHS {
            state: String::from("q5"),
            replacement: String::from("*"),
            direction: 'L',
        },
    };
    let f52 = TransitionFunction {
        lhs: LHS {
            state: String::from("q11"),
            input: String::from("Y"),
        },
        rhs: RHS {
            state: String::from("q11"),
            replacement: String::from("1"),
            direction: 'R',
        },
    };
    let f53 = TransitionFunction {
        lhs: LHS {
            state: String::from("q11"),
            input: String::from("*"),
        },
        rhs: RHS {
            state: String::from("q3"),
            replacement: String::from("*"),
            direction: 'R',
        },
    };
    let s5 = KeyStates {
        initial_state: String::from("q0"),
        final_states: vec![String::from("q12")],
    };
    let functions_mult = vec![
        f26, f27, f28, f29, f30, f31, f32, f33, f34, f35, f36, f37, f38, f39, f40, f41, f42, f43,
        f44, f45, f46, f47, f48, f49, f50, f51, f52, f53,
    ];
    let demo4 = Machine {
        transitions: functions_mult,
        states: s5,
        tracks: 1,
    };
    let demos = vec![demo0, demo1, demo2, demo3, demo4];

    demos
}
