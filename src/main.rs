use regex::Regex;
use std::io::{self, Write};

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

fn main() {
    let transitions = get_transitions();
    let states = get_states(&transitions);
    println!("\nInput:");
    let input = get_input().replace("\r\n", "");
    parse(input, &transitions, &states);
}

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
    println!("{:?}", input);
    let mut i = 1;
    let mut current_state = states.initial_state.clone();

    while i < input.len() {
        let current_char = input[i];
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
            println!("{:?}", input);

            i = match transition.rhs.direction {
                'L' => i - 1,
                'R' => i + 1,
                _ => i,
            };
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

fn function_validator(function: &str) -> bool {
    let re = Regex::new(r"\(.*\,.\)\=\(.*\,(.|blank)\,(L|R|l|r)\)").unwrap();
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
