use tdd_money_rust::money;
use tdd_money_rust::money::Expression;

#[test]
fn multiplication() {
    let five = money::Money::dollar(5);
    assert_eq!(money::Money::dollar(10), five.times(2));
    assert_eq!(money::Money::dollar(15), five.times(3));
}

#[test]
fn simple_addition() {
    let five = money::Money::dollar(5);
    let sum = five.plus(&five);
    let bank = money::Bank::new();
    let reduced = bank.reduce(&*sum, "USD");
    assert_eq!(money::Money::dollar(10), reduced);
}

#[test]
fn reduce_sum() {
    let augend = money::Money::dollar(3);
    let addend = money::Money::dollar(4);
    let sum = money::Sum::new(&augend, &addend);
    let bank = money::Bank::new();
    let reduced = bank.reduce(&sum, "USD");
    assert_eq!(money::Money::dollar(7), reduced);
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
