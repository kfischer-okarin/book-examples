use tdd_money_rust::money;

#[test]
fn multiplication() {
    let five = money::Money::dollar(5);
    assert_eq!(money::Money::dollar(10), five.times(2));
    assert_eq!(money::Money::dollar(15), five.times(3));
}

#[test]
fn currency() {
    assert_eq!("USD", money::Money::dollar(1).currency());
    assert_eq!("CHF", money::Money::franc(1).currency());
}

#[test]
fn equality() {
    assert_eq!(money::Money::dollar(5), money::Money::dollar(5));
    assert_ne!(money::Money::dollar(5), money::Money::dollar(6));
    assert_ne!(money::Money::franc(5), money::Money::dollar(5));
}
