use chrono;
use lipsum::lipsum;
use rand::random_range;

mod money;
fn main() {
    let mut my_money = money::MyMoney::new();
    let range: i32 = random_range(5..20);
    for _i in 1..=range {
        let whose: money::Whose = money::Whose::from_isize(random_range(0..2) as isize);
        let m: money::Money = money::Money::new(
            lipsum(random_range(4..10)),
            random_range(1000..9999999),
            chrono::Utc::now(),
            whose,
        );

        println!("{}", m);
        my_money.remember(m);
    }

    let mut range_forget: i32 = range / 3;
    if range_forget < 1 {
        range_forget = 1;
    }
    for i in 1..=range_forget {
        my_money.forget(i as usize);
    }

    println!("===========================================");
    println!("{}", my_money.summary());
}
