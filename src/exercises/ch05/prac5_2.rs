#![allow(unused)]
// 연습 문제 5.2: 접기
// 엘리베이터가 이동한 방향을 기록한 이벤트를 기반으로 엘리베이터의 현재 층을 계산하기

struct Elevator { floor: i32 }
impl Elevator {
    fn move_to(&self, direction: &Direction) -> Elevator {
        let next_floor = match direction {
            Direction::Up => self.floor + 1,
            Direction::Down => self.floor - 1,
        };
        Elevator { floor: next_floor }
    }
}
enum Direction { Up, Down }

#[cfg(test)]
mod tests {
    use crate::exercises::ch05::prac5_2::Direction::{Down, Up};
    use super::*;
    #[test]
    fn it_works() {
        let values: Vec<Direction> =
            vec![Up, Up, Down, Up, Down, Down, Up, Up, Up, Down];
        let tot = values.iter()
            .fold(Elevator { floor: 0 }, |acc, x| {
                acc.move_to(x)
            });

        assert_eq!(tot.floor, 2);
    }
}