use std::collections::HashMap;
mod parse_functions;
mod parse_grow;
mod parse_show;
mod parse_wait;
mod vars;

/// Parses a plantr program
/// 
/// # Example
/// ```
/// use pots_parser;
/// 
/// let input = "grow plant1 for 3;\nwait for 1;\nshow plant1;";
/// pots_parser::parse_plantr(input);
/// ```
/// 
/// # Panics
/// 
/// Panics if the input is invalid
pub fn parse_plantr(input: &str) {
    use parse_functions::parse_statement;

    let statements = input.split(';');
    let mut variables: HashMap<String, vars::Plant> = HashMap::new();

    for statement in statements {
        if statement == "" { 
            continue;
        }
        parse_statement(statement, &mut variables);
    }
    println!("{:?}", variables);
}
