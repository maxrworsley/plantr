use std::collections::HashMap;
mod parse_functions;
mod parse_grow;
mod parse_show;
mod vars;


pub fn parse_plantr(input: &str) {
    use parse_functions::parse_line;

    let tokens = input.split(';');
    let mut variables: HashMap<String, vars::Plant> = HashMap::new();

    for token in tokens {
        parse_line(token, &mut variables);
    }
}


