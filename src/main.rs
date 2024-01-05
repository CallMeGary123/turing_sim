use regex::Regex;
use std::env;
use std::io::{self, Write};
use prettytable::{format,ptable, row, table, Cell, Row, Table};

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

    if args.len() >= 3 && args[1] == "demo" {
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
            println!("\nInput:");
            let input = get_input().replace("\r\n", "");
            parse(input, &demo.transitions, &demo.states, demo.tracks);
        } else {
            println!("Demo index out of bounds");
        }
    } else if args.len() == 2 {
        println!("Turing Machine Simulator");
        println!("Usage:\ncargo run -- <option>\nturing_sim.exe <option>");
        println!("\nOptions:");
        println!("help : Shows help menu");
        println!("demo 0 : translates every 'a' to 'b'");
        println!("demo 1 : accepts strings in form of a(n)b(n)");
        println!("demo 2 : copies strings of '1'");
        println!("\n*Run without options to input your own turing machine")
    } else {
        // Default behavior when no arguments are provided
        println!("run with 'cargo run -- help' or 'turing_sim.exe help' to see the help menu");
        let tracks: usize = get_input().parse().unwrap(); 
        let transitions = get_transitions();
        let states = get_states(&transitions);
        let turing_machine = Machine {
            transitions: transitions,
            states: states,
            tracks: tracks,
        };
        println!("\nInput:");
        let input = get_input().replace("\r\n", "");
        parse(input, &turing_machine.transitions, &turing_machine.states, turing_machine.tracks);
    }
}

// main functions

fn get_transitions() -> Vec<TransitionFunction> {
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
    //let input = format!("□{}□", input);
    //let mut input: Vec<char> = input.chars().collect();
    
    let mut input: Vec<String> = input.chars().collect::<String>()
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
        for (index, &ref substr) in input.iter().enumerate() {
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
        println!("Success");
    } else {
        println!("Failure");
    }
}

// helper functions
fn function_validator(function: &str) -> bool {
    let re = Regex::new(r"\(.*\,(.|blank)\)\=\(.*\,(.|blank)\,(L|R|l|r)\)").unwrap();
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

    let demo1 = Machine {
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

    let demo2 = Machine {
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

    let demo3 = Machine {
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
    let functions_translator1 = vec![f21, f22, f23];

    let demo4 = Machine {
        transitions: functions_translator1,
        states: s4,
        tracks: 3,
    };
    let demos = vec![demo1, demo2, demo3, demo4];

    demos
}
