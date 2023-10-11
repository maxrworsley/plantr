use crate::parse_grow::parse_grow;
use crate::parse_show::parse_show;
use crate::parse_wait::parse_wait;
use crate::vars::Variables;    

/// Parses a single statement, after stripping newlines
pub fn parse_statement(token: &str, variables: &mut Variables) {
    let trimmed_token = token.replace("\n", "");
    
    let mut split_line = trimmed_token.split(' ');
    if let Some(keyword) = split_line.next() {
        match keyword {
            "grow" => parse_grow(&mut split_line, variables),
            "if" => parse_if(&mut split_line, variables),
            "wait" => parse_wait(&mut split_line, variables),
            "show" => parse_show(&mut split_line, variables),
            _ => panic!("Invalid token: {:?}", split_line)
        }
    }
}


/// Parses an if statement of the form "if <variable name> is <alive/dead> <do this>"
pub fn parse_if(input: &mut std::str::Split<'_, char>, variables: &mut Variables) {
    if let Some(plant) = input.next() {
        if input.next() == Some("is") {
            let condition = input.next();
            let variable_value = variables.get(plant).unwrap().value;

            match condition {
                Some("dead") => { if variable_value > 0 { return }},
                Some("alive") => { if variable_value <= 0 { return }},
                _ => panic!("Invalid condition, can only be dead/alive: {:?}", condition)
            }
            
            if let Some(keyword) = input.next() {
                match keyword {
                    "grow" => parse_grow(input, variables),
                    "wait" => parse_wait(input, variables),
                    "show" => parse_show(input, variables),
                    _ => panic!("Invalid keyword: {:?}", keyword)
                }
                return
            }
            panic!("Invalid if statement - didn't have a follow-up statement to carry out: {:?}", input)
        }
    }
    panic!("Invalid if statement: {:?}", input);
}