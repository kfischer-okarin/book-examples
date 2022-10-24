use tdd_money_rust::money;

#[test]
fn multiplication() {
    let five = money::Dollar::new(5);
    let product = five.times(2);
    assert_eq!(product.amount, 10);
    let product = five.times(3);
    assert_eq!(product.amount, 15);
}


#[test]
fn equality() {
    assert_eq!(money::Dollar::new(5), money::Dollar::new(5));
    assert_ne!(money::Dollar::new(5), money::Dollar::new(6));
}
