use std::collections::HashMap;

pub type Variables = HashMap<String, Plant>;

pub fn init_variables() -> Variables { HashMap::new() }

#[derive(Debug)]
pub struct Plant {
    pub value: i32,
    pub cap: usize
}
