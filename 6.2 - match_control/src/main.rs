use rand::Rng;
use std::thread;
use std::time::Duration;

// // Option with String type
// fn lookup_player(id: u8) -> Option<String> {
//     if id == 1 {
//         return Some("crabby".to_string());
//     }
//
//     None
// }
//
// // Option String
// fn run_game(id: u8) {
//     let player = match lookup_player(id) {
//         Some(p) => {  // if there's some value in lookup_player call it 'p'
//             println!("{}", p);
//             Some(p)
//         },  // this is code executing too, returning a value
//         None =>  Some(String::new()), // return empty String
//     };
//     //let player = lookup_player(1)?;  //
//     println!("Player1: {:?}", player);
//     //Some(())  // this is how to return a unit type
//
//     match player {
//         Some(inner) => println!("inner: {}", inner),  // if there's some value in player call it 'inner'
//         None => println!("Player: None"),
//     }
// }
//
// // Option Tuple
// fn run_game2(id: u8) -> Option<()>{
//     let player = lookup_player(id)?;  //
//     println!("Player2: {:?}", player);
//     Some(())  // this is how to return a unit type
// }


enum State {
    Start,
    Running {hp: u32},
    CameOver(Animation)
}

enum Animation {
    Running,
    Stopped
}

fn main() {
    // let res = run_game(2);
    // println!("run_game returned: {:?}", res);
    // let res = run_game2(2);
    // println!("run_game2 returned: {:?}", res);

    let mut state = State::Start;
    let mut rng = rand::thread_rng();

    loop {
        match state {
            State::Start => {
                println!("Game started");
                state = State::Running {hp: 100} // k/v pairs use brackets
            }
            State::Running {hp: 0} => {
                state = State::CameOver(Animation::Running);  // tuple-like structure using parentheses
                println!("Ouch");
            }
            State::Running {ref mut hp} => {
                //let random_number = if *hp > 10 {rng.gen_range(1..=10)} else {1};

                let random_number = match *hp > 10 {
                    true => rng.gen_range(1..=10),
                    false => 1
                };


                *hp -= random_number;
                println!("player hit {}  hp: {}", random_number, *hp);
            }
            State::CameOver(Animation::Running) => {
                println!("Transition");
                state = State::CameOver(Animation::Stopped)
            }
            State::CameOver(Animation::Stopped) => break
        }
        thread::sleep(Duration::from_secs(1));
    }
}

