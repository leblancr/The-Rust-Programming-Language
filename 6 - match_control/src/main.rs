// enum = list of values (variants) not numbered but can be
#[derive(Debug)]
enum CoinList {
    Penny,  // variants like variable names so no quotes
    Nickel,
    Dime,
    Quarter(UsStateList),  // variants can take values like functions
}

#[derive(Debug)]
enum UsStateList {
    Alabama,
    Alaska(ResourceList),
    California(ResourceList),
}

#[derive(Debug)]
enum ResourceList {
    Gold,
    Oil,
}


fn value_in_cents(coin: &CoinList) -> u8 {
    println!("coin {:?}", coin);
    // match against an enum, if match do this =>
    match coin {
        // enum name::variant name
        CoinList::Penny =>  {
            println!("Lucky penny!");
            1
        }
        CoinList::Nickel => 5,
        CoinList::Dime => 10,
        // CoinList::Quarter(state) returns a tuple of it's values
        CoinList::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            match state {
                UsStateList::Alaska(resource) => {
                    println!("State quarter from {:?}, resource is {:?}!", state, resource);
                    25
                },
                UsStateList::California(resource) => {
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
    let coinlist = [CoinList::Penny,
                    CoinList::Nickel,
                    CoinList::Dime,
                    CoinList::Quarter(UsStateList::Alabama),
                    CoinList::Quarter(UsStateList::Alaska(ResourceList::Oil)),
                    CoinList::Quarter(UsStateList::California(ResourceList::Gold))];

    // send each thing to value_in_cents()
    for coin in coinlist {
        value = value_in_cents(&coin);
        println!("value of {:?} in cents = {}", coin, value);
    }
 }
