use tdd_money_rust::money;

#[test]
fn multiplication() {
    let mut five = money::Dollar::new(5);
    five.times(2);
    assert_eq!(five.amount, 10);
}
