use tdd_money_rust::money;

#[test]
fn multiplication() {
    let five = money::Dollar::new(5);
    assert_eq!(money::Dollar::new(10), five.times(2));
    assert_eq!(money::Dollar::new(15), five.times(3));
}

#[test]
fn franc_multiplication() {
    let five = money::Franc::new(5);
    assert_eq!(money::Franc::new(10), five.times(2));
    assert_eq!(money::Franc::new(15), five.times(3));
}


#[test]
fn equality() {
    assert_eq!(money::Dollar::new(5), money::Dollar::new(5));
    assert_ne!(money::Dollar::new(5), money::Dollar::new(6));
}
