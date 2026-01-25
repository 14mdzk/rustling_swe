use core::fmt;

#[derive(PartialEq, Eq, Debug)]
#[repr(isize)]
pub enum Whose {
    Theirs = 0,
    Mine = 1,
}

impl Whose {
    pub fn from_isize(v: isize) -> Whose {
        match v {
            0 => Whose::Theirs,
            1 => Whose::Mine,
            _ => panic!("Invalid value"),
        }
    }
    pub fn to_narative(&self) -> String {
        match self {
            Whose::Mine => "They gave me".to_string(),
            Whose::Theirs => "You spent".to_string(),
        }
    }
}

pub struct Money {
    for_what: String,
    how_much: i128,
    when: chrono::DateTime<chrono::Utc>,
    whose: Whose,
}

impl Money {
    pub fn new(
        for_what: String,
        how_much: i128,
        when: chrono::DateTime<chrono::Utc>,
        whose: Whose,
    ) -> Money {
        Money {
            for_what,
            how_much,
            when,
            whose,
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} for {} on {}",
            self.whose.to_narative(),
            self.how_much,
            self.for_what,
            self.when.to_rfc3339()
        )
    }
}

pub struct MyMoney {
    moneys: Vec<Money>,
}

impl MyMoney {
    pub fn new() -> MyMoney {
        MyMoney { moneys: Vec::new() }
    }

    pub fn remember(&mut self, money: Money) {
        self.moneys.push(money);
    }

    pub fn forget(&mut self, i: usize) {
        self.moneys.remove(i);
    }

    #[allow(dead_code)]
    pub fn fix(&mut self, money: Money, i: usize) {
        self.moneys[i] = money;
    }

    pub fn summary(&self) -> String {
        let mut summary = String::new();
        let mut total_receive = 0;
        let mut total_left = 0;

        for money in &self.moneys {
            if money.whose == Whose::Mine {
                total_receive += money.how_much;
            } else {
                total_left += money.how_much;
            }
        }

        summary.push_str(&format!(
            "This is how much money I received: {}\n",
            total_receive
        ));
        summary.push_str(&format!(
            "This is how much money I have left: {}\n",
            total_receive - total_left
        ));
        summary
    }
}
