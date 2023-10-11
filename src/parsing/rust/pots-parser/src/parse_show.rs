use crate::vars::Variables;

/// Parses a show statement of the form "show <variable name>"
pub fn parse_show(input: &mut std::str::Split<'_, char>, variables: &mut Variables) {
    
    if let Some(plant_name) = input.next() {
        let plant = variables.get(plant_name).unwrap();
        println!("{}: {}", plant_name, plant.value);

        if let Some(_) = input.next() {
            panic!("Invalid show statement: {:?}", input)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vars::{init_variables, Plant};
    use super::*;

    #[test]
    fn test_show() {
        let mut input_iter = "plant1".split(' ');
        let mut variables = init_variables();
        variables.insert("plant1".to_string(), Plant { value: 3, cap: 3 });
        
        parse_show(&mut input_iter, &mut variables);
    }

    #[test]
    #[should_panic]
    fn test_show_panic_without_matching_variable() {
        let mut input_iter = "plant1".split(' ');
        let mut variables = init_variables();
        
        parse_show(&mut input_iter, &mut variables);
    }

    #[test]
    #[should_panic]
    fn test_show_panic_when_showing_number() {
        let mut input_iter = "2".split(' ');
        let mut variables = init_variables();
        variables.insert("plant1".to_string(), Plant { value: 3, cap: 3 });
        
        parse_show(&mut input_iter, &mut variables);
    }

    // The grow keyword should panic if given too much input
    #[test]
    #[should_panic]
    fn test_show_panic_with_too_much_input() {
        let mut input_iter = "plant1 for 5".split(' ');
        let mut variables = init_variables();
        variables.insert("plant1".to_string(), Plant { value: 3, cap: 3 });
        
        parse_show(&mut input_iter, &mut variables);
    }

}