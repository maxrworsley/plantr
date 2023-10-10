use pots_parser;

#[test]
fn test_parse_plantr() {
    let input = "grow plant1 for 3;\nwait for 1;\nshow plant1;";
    pots_parser::parse_plantr(input);
}

#[test]
#[should_panic]
fn test_parse_panic_wrong_input() {
    let input = "help plant1 for 3;\nwait for 1;\nshow plant1;";
    pots_parser::parse_plantr(input);
}