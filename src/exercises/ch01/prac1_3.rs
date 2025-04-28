#![allow(unused)]

#[cfg(test)]
mod tests {
    use rand::prelude::ThreadRng;
    use rand::Rng;
    use std::cmp::{max, min};

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(5 + 6, 11);
        assert_eq!(7 + 42, 49);
        assert_eq!(999 + 1, 1000);
        // 이런 테스트를 얼마나 많이 해야 안심할 수 있을까? 모든 수를 테스트할 순 없다
    }

    // 덧셈이라면 덧셈 연산의 속성을 테스트 할 수 있다.

    fn random_natural(rng: &mut ThreadRng) -> i32 {
        rng.random_range(1..=100_000_000)
    }

    #[test]
    fn test_zero_identity() {
        // 항등원: 어던 수에 0을 더하면 자기자신이 된다.
        let mut rng = rand::rng();

        for _ in 0..100 {
            let x = random_natural(&mut rng);

            assert_eq!(x + 0, x)
        }
    }

    #[test]
    fn test_commutative_property() {
        // 교환 법칙: 두 수를 더한 값은 두 수의 순서를 바꿔 더해도 달라지지 않는다
        let mut rng = rand::rng();

        for _ in 0..100 {
            let x = random_natural(&mut rng);
            let y = random_natural(&mut rng);

            assert_eq!(x + y, y + x)
        }
    }

    #[test]
    fn test_associative_property() {
        // 결합 법칙: 세 수를 더할  각 수를 어떻게 묵어도 결과가 달라지지 않는다.
        let mut rng = rand::rng();

        for _ in 0..100 {
            let x = random_natural(&mut rng);
            let y = random_natural(&mut rng);
            let z = random_natural(&mut rng);

            assert_eq!((x + y) + z, x + (y + z));
            assert_eq!((y + z) + x, y + (z + x));
            assert_eq!((z + x) + y, z + (x + y));
        }
    }

    #[test]
    fn test_adding_one() {
        // 연습문제 1.3: 두 정수중 작은 수에 1씩 더하면 결국 큰 수와 같아진다
        let mut rng = rand::rng();

        for _ in 0..100 {
            let (x, y) = (random_natural(&mut rng), random_natural(&mut rng));

            let mut min = min(x, y);
            let max = max(x, y);

            while min < max {
                min += 1;
            }

            assert_eq!(min, max);
        }
    }
}
