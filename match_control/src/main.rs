// enum = list of types not numbered but can be
    #[derive(Debug)]
    enum CoinList {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStateList),
}

#[derive(Debug)]
enum UsStateList {
    Alabama(ResourceList),
    Alaska(ResourceList),
}

#[derive(Debug)]
enum ResourceList {
    Gold,
    Oil,
}


fn value_in_cents(coin: &CoinList) -> u8 {
    // if match do this =>
    match coin {
        CoinList::Penny =>  {
            println!("Lucky penny!");
            1
        }
        CoinList::Nickel => 5,
        CoinList::Dime => 10,
        // CoinList::Quarter(state) returns a tuple of it's values
        CoinList::Quarter(state) => {
            match state {
                UsStateList::Alaska(resource) => {
                    println!("State quarter from {:?}, resource is {:?}!", state, resource);
                    25
                },
                _ => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        },
    }
}

fn main() {
    let mut value;
    
    for coin in [CoinList::Penny,
                 CoinList::Nickel,
                 CoinList::Dime,
                 CoinList::Quarter(UsStateList::Alabama(ResourceList::Oil)),
                 CoinList::Quarter(UsStateList::Alaska(ResourceList::Gold))] {
        value = value_in_cents(&coin);
        println!("value of {:?} in cents = {}", coin, value);
    }
 }
