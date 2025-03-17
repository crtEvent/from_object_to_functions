use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone)]
enum Items {
    Milk, Fish
}

struct CashRegister {
    price_info: HashMap<Items, i64>,
    promotion_info: HashMap<Items, PromotionPolicy>,
}

#[derive(Clone)]
struct PromotionPolicy {
    required_quantity: i64, // 필요한 개수
    paid_quantity: i64,     // 실제 지불하는 개수
}

impl CashRegister {
    fn new(
        price_info: HashMap<Items, i64>,
        promotion_info: HashMap<Items, PromotionPolicy>,
    ) -> Self {
        Self {price_info, promotion_info}
    }

    fn checkout(&self, items: Vec<Items>) -> Result<i64, String> {
        let mut item_counts: HashMap<Items, i64> = HashMap::new();

        for item in items {
            *item_counts.entry(item).or_insert(0) += 1;
        }

        let mut total_price: i64 = 0;

        for (item, count) in &item_counts {
            if let Some(price) = self.price_info.get(item) {
                if let Some(policy) = self.promotion_info.get(item) {
                    let discount_sets = (count / policy.required_quantity) * policy.paid_quantity;
                    let remaining = count % policy.required_quantity;
                    total_price += (discount_sets + remaining) * price;
                } else {
                    total_price += count * price;
                }
            } else {
                return Err("Item not found".to_string())
            }
        }

        Ok(total_price)
    }
    
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use super::*;

    // 의문점: 테스트를 난수 값으로 사용하라고 하는데. 그럼 테스트 코드에도 비즈니스 계산 로직이 들어가야 하는거 아닌지?
    // 그럼 비즈니스 로직이 잘못 되었는지 어떻게 확인함?
    #[test]
    fn test_checkout_with_promotion() {
        let mut rng = rand::rng();

        let price_info: HashMap<Items, i64> = HashMap::from([
            (Items::Milk, rng.random_range(1..=50) * 100_i64),
            (Items::Fish, rng.random_range(1..=50) * 100_i64),
        ]);

        let promotion_info: HashMap<Items, PromotionPolicy> = HashMap::from([
            (Items::Milk, PromotionPolicy { required_quantity: 3, paid_quantity: 2 }),
            (Items::Fish, PromotionPolicy { required_quantity: 2, paid_quantity: 1 }),
        ]);

        let cash_register = CashRegister::new(price_info.clone(), promotion_info.clone());

        let items = vec![Items::Milk, Items::Milk, Items::Milk, Items::Fish, Items::Fish];

        let result = cash_register.checkout(items);

        let expected_price = price_info.get(&Items::Milk).unwrap() * 2 + price_info.get(&Items::Fish).unwrap();
        assert_eq!(result.unwrap(), expected_price);
    }

    #[test]
    fn test_checkout_without_promotion() {
        let price_info: HashMap<Items, i64> = HashMap::from([
            (Items::Milk, 1500),
            (Items::Fish, 5000),
        ]);

        let promotion_info = HashMap::new(); // 프로모션 없음

        let cash_register = CashRegister::new(price_info, promotion_info);

        let items = vec![Items::Milk, Items::Milk, Items::Milk, Items::Fish, Items::Fish];

        let result = cash_register.checkout(items);

        assert_eq!(result.unwrap(), 1500 * 3 + 5000 * 2);
    }

    #[test]
    fn test_checkout_with_non_existing_item() {
        let price_info = HashMap::from([
            (Items::Milk, 1500),
        ]);

        let promotion_info = HashMap::from([
            (Items::Milk, PromotionPolicy { required_quantity: 3, paid_quantity: 2 })
        ]);

        let cash_register = CashRegister::new(price_info, promotion_info);

        let items = vec![Items::Milk, Items::Fish]; // Fish는 price_info에 없음

        let result = cash_register.checkout(items);

        // Fish 항목이 없으므로 에러 발생
        assert!(result.is_err());
    }
}