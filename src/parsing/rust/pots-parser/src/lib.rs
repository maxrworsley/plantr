use core::time;
use std::collections::HashMap;

struct Plant {
    value: i32,
    cap: usize
}

pub fn parse_plantr(input: &str) {
    let tokens = input.split(';');
    let mut variables: HashMap<String, Plant> = HashMap::new();

    for token in tokens {
        let mut split_line = token.split(' ');
        if let Some(word) = split_line.next() {
            match word {
                "grow" => parse_grow(&mut split_line, &mut variables),
                "if" => parse_if(&mut split_line, &mut variables),
                "wait" => parse_wait(&mut split_line, &mut variables),
                "show" => parse_show(&mut split_line, &mut variables),
                _ => panic!("Invalid token: {:?}", split_line)
            }
        }        
    }
}
fn parse_grow(input: &mut std::str::Split<'_, char>, variables: &mut HashMap<String, Plant>) {
    // grow <name> for <amount>
    if let Some(name) = input.next() {
        let for_present = input.next() == Some("for");
        if for_present {
            if let Some(val) = input.next() {
                let amount = val.parse::<usize>().unwrap();
                
                let plant = Plant {value: 0, cap: amount};
                variables.insert(name.to_string(), plant);
            }
        }
    }

    panic!("Invalid grow statement: {:?}", input);
}

fn parse_if(input: &mut std::str::Split<'_, char>, variables: &mut HashMap<String, Plant>) {
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
                // Keep parsing statements like normal   
            }

        }
    }
}

fn parse_wait(input: &mut std::str::Split<'_, char>, variables: &mut HashMap<String, Plant>) {
    if let Some(time) = input.next() {
        let time = time.parse::<usize>().unwrap();
        tick_variables(variables, time);
    }
}

fn parse_show(input: &mut std::str::Split<'_, char>, variables: &mut HashMap<String, Plant>) {
    if let Some(plant_name) = input.next() {
        let plant = variables.get(plant_name).unwrap();
        println!("{:?}: {}", plant_name, plant.value);
    }
}

fn tick_variables(variables: &mut HashMap<String, Plant>, number_of_ticks: usize) {
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

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
