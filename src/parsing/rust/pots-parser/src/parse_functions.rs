use std::collections::HashMap;

pub struct Plant {
    value: i32,
    cap: usize
}

pub fn parse_line(token: &str, variables: &mut HashMap<String, Plant>) {
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

pub fn parse_grow(input: &mut std::str::Split<'_, char>, variables: &mut HashMap<String, Plant>) {
    // grow <name> for <amount>
    if let Some(name) = input.next() {
        let for_present = input.next() == Some("for");
        if for_present {
            if let Some(val) = input.next() {
                let amount = val.parse::<usize>().unwrap();
                
                let plant = Plant {value: 0, cap: amount};
                variables.insert(name.to_string(), plant);
                return;
            }
        }       
    }

    panic!("Invalid grow statement: {:?}", input);
}

pub fn parse_if(input: &mut std::str::Split<'_, char>, variables: &mut HashMap<String, Plant>) {
    // if <name> is <alive/dead> [do this]
    if let Some(plant) = input.next() {
        if input.next() == Some("is") {
            let condition = input.next();
            let mut do_condition = false;
            let variable_value = variables.get(plant).unwrap().value;

            match condition {
                Some("dead") => {
                    if variable_value <= 0 {
                        do_condition = true;
                    }
                },
                Some("alive") => {
                    if variable_value > 0 {
                        do_condition = true;
                    }
                },
                _ => panic!("Invalid condition: {:?}", condition)
            }

            if do_condition {
                // Need to strip the [] off either side of the input
                parse_line(input, variables);
            }

        }
    }
}

pub fn parse_wait(input: &mut std::str::Split<'_, char>, variables: &mut HashMap<String, Plant>) {
    if let Some(time) = input.next() {
        let time = time.parse::<usize>().unwrap();
        tick_variables(variables, time);
    }
}

pub fn parse_show(input: &mut std::str::Split<'_, char>, variables: &mut HashMap<String, Plant>) {
    if let Some(plant_name) = input.next() {
        let plant = variables.get(plant_name).unwrap();
        println!("{}: {}", plant_name, plant.value);
    }
}

pub fn tick_variables(variables: &mut HashMap<String, Plant>, number_of_ticks: usize) {
    for _ in 0..number_of_ticks {
        for (name, plant) in variables.iter_mut() {
            if plant.value < plant.cap as i32{
                plant.value += 1;
            }
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that ensures the grow function works as expected
    #[test]
    fn test_grow_initial() {
        let input = "plant1 for 5";
        let mut input_iter = input.split(' ');

        let mut variables: HashMap<String, Plant> = HashMap::new();
        parse_grow(&mut input_iter, &mut variables);

        let plant = variables.get("plant1").unwrap();
        assert!(plant.value == 0);
    }

    // Test that ensures the grow function grows a variable for 2 ticks
    #[test]
    fn test_grow_tick() {
        let input = "plant1 for 5";
        let mut input_iter = input.split(' ');

        let mut variables: HashMap<String, Plant> = HashMap::new();
        parse_grow(&mut input_iter, &mut variables);

        tick_variables(&mut variables, 2);

        let plant = variables.get("plant1").unwrap();
        assert!(plant.value == 2);
    }

    // Test that ensures a panic is thrown by the grow function
    #[test]
    #[should_panic]
    fn test_grow_invalid() {
        let input = "plant1 for";
        let mut input_iter = input.split(' ');

        let mut variables: HashMap<String, Plant> = HashMap::new();
        parse_grow(&mut input_iter, &mut variables);
    }
}
