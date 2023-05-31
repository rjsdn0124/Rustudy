// hashmaps2.rs
// We're collecting different fruits to bake a delicious fruit cake.
// For this, we have a basket, which we'll represent in the form of a hash
// map. The key represents the name of each fruit we collect and the value
// represents how many of that particular fruit we have collected.
// Three types of fruits - Apple (4), Mango (2) and Lychee (5) are already
// in the basket hash map.
// You must add fruit to the basket so that there is at least
// one of each kind and more than 11 in total - we have a lot of mouths to feed.
// You are not allowed to insert any more of these fruits!
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps2` or use the `hint` watch subcommand for a hint.

// 제너릭 타입 초기화할 수 있넹ㅎ ::<>::new()로 할 수 있대용~
// hashmap 여러 함수들을 알아보았음니다.

use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        // TODO: Insert new fruits if they are not already present in the basket.
        // Note that you are not allowed to put any type of fruit that's already
        // present!
        // if 문으로 fruit가 없으면 1개 넣어주는 문장. 이 방법도 가능함!
        // get은 참조값으로 들어가야함!
        // insert할때는 참조 말고 그대로 들어감! 만약 fruit다시 쓰고 싶다면 copy하자!
        // if basket.get(&fruit)==None {basket.insert(fruit,1);}

        // hashmap의 함수를 써서 구현!
        // entry 함수로 들어있는지 확인하고 or_insert로 없다면 값을 넣어주는 작업을 구현!
        // fruit가 move되니 fruit를 재사용하고 싶다면 copy를 사용하자!
        basket.entry(fruit).or_insert(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Don't modify this function!
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let mut basket = HashMap::<Fruit, u32>::new();
        basket.insert(Fruit::Apple, 4);
        basket.insert(Fruit::Mango, 2);
        basket.insert(Fruit::Lychee, 5);

        basket
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }
}
