use crate::parse_grow::parse_grow;
use crate::vars::Variables;    


pub fn parse_line(token: &str, variables: &mut Variables) {
    let mut split_line = token.split(' ');
    if let Some(keyword) = split_line.next() {
        match keyword {
            "grow" => parse_grow(&mut split_line, variables),
            "if" => parse_if(&mut split_line, variables),
            "wait" => parse_wait(&mut split_line, variables),
            "show" => parse_show(&mut split_line, variables),
            _ => panic!("Invalid token: {:?}", split_line)
        }
    }
    tick_variables(variables, 1);
}



pub fn parse_if(input: &mut std::str::Split<'_, char>, variables: &mut Variables) {
    // if <name> is <alive/dead> [do this]
    if let Some(plant) = input.next() {
        if input.next() == Some("is") {
            let condition = input.next();
            let variable_value = variables.get(plant).unwrap().value;

            match condition {
                Some("dead") => { if variable_value > 0 { return }},
                Some("alive") => { if variable_value <= 0 { return }},
                _ => panic!("Invalid condition: {:?}", condition)
            }

            // Need to strip the [] off either side of the input
            // parse_line(input, variables);
        }
    }
}

pub fn parse_wait(input: &mut std::str::Split<'_, char>, variables: &mut Variables) {
    if let Some(time) = input.next() {
        let time = time.parse::<usize>().unwrap();
        tick_variables(variables, time);
    }
}

pub fn parse_show(input: &mut std::str::Split<'_, char>, variables: &mut Variables) {
    if let Some(plant_name) = input.next() {
        let plant = variables.get(plant_name).unwrap();
        println!("{}: {}", plant_name, plant.value);
    }
}

pub fn tick_variables(variables: &mut Variables, number_of_ticks: usize) {
    for (_, plant) in variables.iter_mut() {
        plant.value = std::cmp::min(plant.cap as i32, plant.value + number_of_ticks as i32)
    }
}

// Write test boilderplate
#[cfg(test)]
mod tests {
    use crate::vars::init_variables;
    use super::*;

        // Test that ensures the grow function grows a variable for 2 ticks
        #[test]
        fn test_grow_tick() {
            let input = "plant1 for 5";
            let mut input_iter = input.split(' ');
    
            let mut variables = init_variables();
            parse_grow(&mut input_iter, &mut variables);
    
            tick_variables(&mut variables, 2);
    
            let plant = variables.get("plant1").unwrap();
            assert!(plant.value == 2);
        }
}