#![allow(unused)]
// 연습 문제 5.4: 모노이드
struct Monoid<T, F>
where
    F: FnMut(T, T) -> T,
{
    init: T,
    op: F,
}

impl<T, F> Monoid<T, F>
where
    F: FnMut(T, T) -> T,
{
    fn init(init: T, op: F) -> Monoid<T, F> {
        Monoid { init, op }
    }
    
    fn fold(mut self, list: Vec<T>) -> T {
        list.into_iter().fold(self.init, self.op)
    }
}

#[derive(PartialEq, Debug)]
struct Money(f64);
impl Money {
    fn sum(a: Money, b: Money) -> Money {
        Money(a.0 + b.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn verify_monoid_of_integer() {
        let monoid_integer = 
            Monoid::init(0, |a, b| a + b)
            .fold(vec![1, 2, 3]);
        assert_eq!(monoid_integer, 6);
    }

    #[test]
    fn verify_monoid_of_string() {
        let monoid_string =
            Monoid::init("".to_string(), |a, b| a + b.as_str())
                .fold(vec!["My".to_string(), "Fair".to_string(), "Lady".to_string()]);
        assert_eq!(monoid_string, "MyFairLady");
    }

    #[test]
    fn verify_monoid_of_money() {
        let monoid_integer =
            Monoid::init(Money(0.0), |a, b| Money::sum(a, b))
                .fold(vec![Money(2.1), Money(3.9), Money(4.0)]);
        assert_eq!(monoid_integer, Money(10.0));
    }
}