// quiz1.rs

// Put your function here!
fn calculate_price_of_apples(num: i32) -> i32 {
    if num > 40 {
        num
    } else {
        num * 2
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}