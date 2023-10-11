use crate::vars::Variables;

/// Parses a wait statement of the form "wait for <number of ticks>"
pub fn parse_wait(input: &mut std::str::Split<'_, char>, variables: &mut Variables) {
    if let Some("for") = input.next() {
        if let Some(time) = input.next() {
            let time = time.parse::<usize>().unwrap();
            tick_variables(variables, time);
            return;
        }
    }
    panic!("Invalid wait statement: {:?}", input)
    
}

/// Increments all the variables by the given number of ticks until they hit their cap
pub fn tick_variables(variables: &mut Variables, number_of_ticks: usize) {
    for (_, plant) in variables.iter_mut() {
        plant.value = std::cmp::min(plant.cap as i32, plant.value + number_of_ticks as i32)
    }
}

#[cfg(test)]
mod tests {
    use crate::vars::{init_variables, Plant};
    use super::*;

        #[test]
        fn test_grow_plant_for_two_ticks() {
            let mut variables = init_variables();
            variables.insert("plant1".to_string(), Plant {value: 0, cap: 3});

            tick_variables(&mut variables, 2);
    
            let plant = variables.get("plant1").unwrap();
            assert!(plant.value == 2);
        }
}