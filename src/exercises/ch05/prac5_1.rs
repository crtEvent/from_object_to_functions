#![allow(unused)]
// 연습 문제 5.1: 재귀
// 재귀 스타일로 함수를 작성해 콜라츠 추측(Collatz conjecture) 구현하기
//
// 콜라츠 추측(Collatz conjecture)이란?
//   - 주어진 자연수에 대해 특정 규칙을 반복적으로 적용하면 결국 항상 1에 도달한다는 주장
//
// 콜라츠 추측의 규칙:
//   1. 임의의 자연수 n에 대해 다음 규칙을 반복적으로 적용한다.
//     - n이 짝수이면, n을 2로 나눈다. (n / 2)
//     - n이 홀수이면, n에 3을 곱하고 1을 더한다. (3n + 1)
//   2. 그 후, 이 과정을 계속해서 반복한다. 결국, 어떤 자연수에 대해 이 규칙을 적용하면 반드시 1에 도달한다.

trait Collatz {
    fn collatz(self) -> Vec<i32>;
    fn collatz_recursive(x: i32, acc: Vec<i32>) -> Vec<i32>;
}

impl Collatz for i32 {
    fn collatz(self) -> Vec<i32> {
        Self::collatz_recursive(self, vec![self])
    }

    fn collatz_recursive(x: i32, acc: Vec<i32>) -> Vec<i32> {
        if x == 1 {
            acc
        } else if x % 2 == 0 {
            let num = x / 2;
            let list = acc.iter().cloned().chain(std::iter::once(num)).collect();
            Self::collatz_recursive(num, list)
        } else {
            let num = (x * 3) + 1;
            let list = acc.iter().cloned().chain(std::iter::once(num)).collect();
            Self::collatz_recursive(num, list)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_collatz() {
        assert_eq!(13.collatz(), vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1]);
        assert_eq!(8.collatz(), vec![8, 4, 2, 1]);
    }
}