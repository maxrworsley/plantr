use std::collections::HashMap;
mod parse_functions;



pub fn parse_plantr(input: &str) {
    use parse_functions::{Plant, parse_line};

    let tokens = input.split(';');
    let mut variables: HashMap<String, Plant> = HashMap::new();

    for token in tokens {
        parse_line(token, &mut variables);
    }
}


