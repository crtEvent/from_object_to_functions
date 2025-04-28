#![allow(unused)]
// 연습문제 4.3: Currying

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn curry_i32(func: impl Fn(i32, i32) -> i32 + Clone + 'static) -> impl Fn(i32) -> Box<dyn Fn(i32) -> i32> {
    move |a| {
        let func = func.clone();
        Box::new(move |b| {
            func(a, b)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_curry() {
        let plus_3_func= curry_i32(add)(3);
        assert_eq!(plus_3_func(4), 7);
        assert_eq!(plus_3_func(5), 8);
    }

}
