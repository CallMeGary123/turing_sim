use regex::Regex;
use std::io::{self, Write};
use stybulate::{Cell, Style, Table};

#[derive(PartialEq)]
struct LHS {
    state: String,
    input: char,
}

#[derive(PartialEq)]
struct RHS {
    state: String,
    char: char,
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

// main
fn main() {
    let transitions = get_transitions();
    let states = get_states(&transitions);
    println!("\nInput:");
    let input = get_input().replace("\r\n", "");
    parse(input, &transitions, &states);
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
            input: lhs_parts[1].chars().next().unwrap(),
        };

        let rhs_parts: Vec<&str> = parts[1]
            .trim_matches(|c| c == '(' || c == ')' || c == '\r' || c == '\n')
            .split(',')
            .map(|s| s.trim())
            .collect();

        let rhs = RHS {
            state: rhs_parts[0].to_string(),
            char: rhs_parts[1].chars().next().unwrap(),
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

fn parse(input: String, transitions: &[TransitionFunction], states: &KeyStates) {
    println!("\nparsing...");
    let input = format!("□{}□", input);
    let mut input: Vec<char> = input.chars().collect();

    let mut i = 1;
    let mut current_state = states.initial_state.clone();

    loop {
        let mut tape_cell: Vec<Cell> = Vec::new();
        let mut head_cell: Vec<Cell> = Vec::new();

        let current_char = input[i];

        for (index, &c) in input.iter().enumerate() {
            let cell = Cell::from(c.to_string().as_str());
            tape_cell.push(cell);

            let head_symbol = if index == i { "▼" } else { " " };
            head_cell.push(Cell::from(head_symbol));
        }

        head_cell.insert(0, Cell::from("HEAD"));
        tape_cell.insert(0, Cell::from("TAPE"));

        let table = Table::new(Style::Fancy, vec![head_cell, tape_cell], None);
        println!("{}", table.tabulate());

        let lhs_to_find = LHS {
            state: current_state.clone(),
            input: current_char,
        };
        println!("Current state: {}", current_state);
        println!("Current input: '{}', Head position: {}", current_char, i);
        if let Some(transition) = transitions.iter().find(|t| t.lhs == lhs_to_find) {
            println!(
                "Transition function: δ({},{})=({},{},{})\n",
                transition.lhs.state,
                transition.lhs.input,
                transition.rhs.state,
                transition.rhs.char,
                transition.rhs.direction
            );
            current_state = transition.rhs.state.clone();
            input[i] = transition.rhs.char;
            if transition.rhs.direction == 'L' {
                if i == 0 {
                    input.insert(0, '□');
                } else {
                    i = i - 1;
                }
            } else if transition.rhs.direction == 'R' {
                i = i + 1;
                if i == input.len() {
                    input.push('□');
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
