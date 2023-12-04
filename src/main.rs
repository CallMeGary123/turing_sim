use std::io::{self, Write};
#[derive(Debug, PartialEq)]
struct LHS {
    state:String,
    input:char
}
#[derive(Debug, PartialEq)]
struct RHS {
    state:String,
    char:char,
    direction:char
}

#[derive(Debug, PartialEq)]
struct TransitionFunction{
    lhs:LHS,
    rhs:RHS
}
#[derive(Debug, PartialEq)]
struct KeyStates{
    initial_state:String,
    final_states:Vec<String>
}

fn main() {
    let transitions = get_tarnsitions();
    let states = get_states();
    println!("Input:");
    let input = get_input().replace("\r\n", "");
    parse(input, transitions, states);
}

fn get_states() -> KeyStates {
    println!("Enter initial state:");
    let initial = get_input()
        .trim()
        .to_string();
    let initial = initial
        .trim_matches(|c| c == ' ' || c == '\r' || c == '\n')
        .to_string();

    println!("Enter final states:");
    let finals = get_input().trim().to_string();
    let finals = finals.replace(" ", "");
    let finals: Vec<String> = finals
        .trim_matches(|c| c == ' ' || c == '\r' || c == '\n')
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let states = KeyStates{
        initial_state: initial,
        final_states: finals
    };
    println!("{:?}",states);

    states
}

fn parse(mut input: String, transitions: Vec<TransitionFunction>, states: KeyStates) {
    let mut i = 1;
    let blank = "□";
    input.insert_str(0, blank);
    input.push_str(blank);
    let mut input: Vec<char> = input.chars().collect();
    let mut current_state = states.initial_state;

    while i < input.len() {
        let current_char = input[i];
        let lhs_to_find = LHS {
            state: current_state.clone(),
            input: current_char.clone(),
        };

        if let Some(transition) = transitions.iter().find(|t| t.lhs == lhs_to_find) {
            current_state = transition.rhs.state.clone();
            input[i] = transition.rhs.char;
            println!("{:?}", input);
            if transition.rhs.direction == 'L' {
                i = i - 1;
            } else {
                i = i + 1;
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

fn get_tarnsitions() -> Vec<TransitionFunction> {
    let mut functions: Vec<TransitionFunction> = Vec::new();

    println!("Enter functions e.g δ(q1,a)=(q2,b,L) [enter END if you don't want to add anymore functions]: ");
    println!("you can use 'blank' instead of □");

    loop {
        print!("δ");
        io::stdout().flush().expect("failed to flush");
        let func = get_input();
        if func.trim().to_uppercase() == "END"{ break; }
        let func = func.replace(" ", "");
        let func: Vec<&str> = func.split('=').collect();
        let mut lhs: Vec<String> = func[0]
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .map(|s| s.to_string())
            .collect();
        if lhs[1] == "blank" {
            lhs[1] = "□".to_string();
        }
        let lhs = LHS {
            state: lhs[0].clone(),
            input: lhs[1].chars().next().unwrap(),
        };
        let mut rhs: Vec<String> = func[1]
            .trim_matches(|c| c == '(' || c == ')' || c == '\r' || c == '\n')
            .split(',')
            .map(|s| s.to_string())
            .collect();
        if rhs[1] == "blank" {
            rhs[1] = "□".to_string();
        }
        let rhs = RHS {
            state: rhs[0].clone(),
            char: rhs[1].chars().next().unwrap(), 
            direction:rhs[2].chars().next().unwrap()};
        let current_function = TransitionFunction {lhs: lhs, rhs:rhs};
        functions.push(current_function);
        println!("{:?}", functions);        
    }
    functions
}

fn get_input() -> String{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error occurred");
    input
}
