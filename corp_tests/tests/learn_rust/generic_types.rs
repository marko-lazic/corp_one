use std::cmp::Ordering;

#[test]
fn find_the_largest() {
    let number_list = vec![32, 55, 592, 12, 34, 3, 1];

    let largest = find_greatest(number_list);

    println!("The largest number is {} ", largest);

    let char_list = vec!['m', 'a', 'r', 'k', 'o'];

    let largest = find_greatest(char_list);

    println!("The largest char is {} ", largest);

    let wallet_list = vec![Wallet { money: 100 }, Wallet { money: 200 }];

    let richest = find_greatest(wallet_list);

    println!("The richest wallet is {:?}", richest)
}

fn find_greatest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
#[derive(Copy, Clone, Debug)]
struct Wallet {
    money: i32,
}

impl PartialOrd for Wallet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.money.partial_cmp(&other.money)
    }
}
impl PartialEq for Wallet {
    fn eq(&self, other: &Self) -> bool {
        self.money == other.money
    }
}
