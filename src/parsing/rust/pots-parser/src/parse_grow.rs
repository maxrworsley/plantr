use crate::vars::{Plant, Variables};



pub fn parse_grow(input: &mut std::str::Split<'_, char>, variables: &mut Variables) {
    // grow <name> for <amount>
    // grow <name> mixing <var1> and <var2>

    if let Some(name) = input.next() {
        let next_keyword = input.next();
        match next_keyword {
            Some("for") => {if let Some(val) = input.next() {
                let amount = val.parse::<usize>().unwrap();
                
                let plant = Plant {value: 0, cap: amount};
                variables.insert(name.to_string(), plant);
                return;
            }},
            Some("mixing") => {
                // Average the values of the following two plants in the input
                let var1 = input.next().unwrap();
                if input.next() == Some("and") {
                    let var2 = input.next().unwrap();
                    let plant1 = variables.get(var1).unwrap();
                    let plant2 = variables.get(var2).unwrap();

                    let average = (plant1.value + plant2.value) / 2;
                    let cap_value = std::cmp::max(plant1.cap, plant2.cap);
                    let plant = Plant {value: average, cap: cap_value};
                    variables.insert(name.to_string(), plant);
                    return;
                } else {    
                    panic!("Invalid grow statement, missing 'and' keyword: {:?}", input)
                }
            }
            _ => panic!("Invalid grow statement: {:?}", input)
        }    
    }

    panic!("Invalid grow statement: {:?}", input);
}

#[cfg(test)]
mod tests {
    use crate::vars::init_variables;
    use super::*;

    // Test that ensures the grow function works as expected
    #[test]
    fn test_grow_initial() {
        let mut input_iter = "plant1 for 5".split(' ');
        let mut variables = init_variables();
        
        parse_grow(&mut input_iter, &mut variables);

        assert!(variables.get("plant1").unwrap().value == 0);
    }

    // Test that ensures a panic is thrown by the grow function
    #[test]
    #[should_panic]
    fn test_grow_invalid() {
        let mut input_iter = "plant1 for".split(' ');
        let mut variables = init_variables();

        parse_grow(&mut input_iter, &mut variables);
    }
}