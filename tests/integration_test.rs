use changers::logic;
use std::collections::HashMap;

#[test]
fn test_change_possible() {
    let result = logic::make_change(10.0, vec![1.0, 2.0, 5.0, 10.0]).unwrap();
    let target: HashMap<String, i32> = [
        (String::from("1"), 0),
        (String::from("2"), 0),
        (String::from("5"), 0),
        (String::from("10"), 1),
    ]
    .iter()
    .cloned()
    .collect();

    assert_eq!(result, target);
}

#[test]
fn test_change_impossible() {
    let result = logic::make_change(10.0, vec![3.0, 8.0, 12.0]);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Amount cannot be reached with the given set of coins"
    );
}

#[test]
fn test_change_impossible2() {
    let result = logic::make_change(10.5, vec![1.0, 2.0, 5.0, 10.0]);

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "Amount cannot be reached with the given set of coins"
    );
}
