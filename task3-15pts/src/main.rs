#![allow(unused_must_use)]

use std::fmt::Display;
use rand::Rng;

struct Roulette {
    people: Vec<RouletteParticipant>,
    people_outed_so_far: usize
}

fn generate_random_number(from: usize, to: usize) -> usize {
    return rand::thread_rng().gen_range(from..to);
}

impl Roulette {
    fn new(people: Vec<RouletteParticipant>) -> Self {
        // todo return new object!
    }

    fn kill(self, n: usize) {
        type Index = usize;
        let mut still_in_game_participants: Vec<(Index, RouletteParticipant)> = self.people.clone().into_iter().enumerate().filter(|item| match item.1 {
            RouletteParticipant::InGame{ .. } => true,
            RouletteParticipant::/* todo your enum and it's fields. You can omit all values with `..` */ => false,
        }).collect();

        // todo change people vec in a way that outed players will 
    }
}

enum RouletteParticipant {
    // todo your definition of this enum
}

// todo implement Display trait for RouletteParticipant
// todo implement Display trait for Roulette

fn main() {
    let mut roulette_session = Roulette::new(
        vec!(
            RouletteParticipant::InGame{name: "John".to_string()},
            RouletteParticipant::InGame{name: "Robert".to_string()},
            RouletteParticipant::InGame{name: "Joe".to_string()},
            RouletteParticipant::InGame{name: "Satoshi".to_string()},
            RouletteParticipant::InGame{name: "Peter".to_string()},
            RouletteParticipant::InGame{name: "Paul".to_string()},
            RouletteParticipant::InGame{name: "Vladimir".to_string()},
            RouletteParticipant::InGame{name: "Matthew".to_string()},
            RouletteParticipant::InGame{name: "Chen".to_string()},
            RouletteParticipant::InGame{name: "Isaac".to_string()},
            RouletteParticipant::InGame{name: "Anna".to_string()},
            RouletteParticipant::InGame{name: "Matilda".to_string()},
            RouletteParticipant::InGame{name: "Arjun".to_string()},
        )
    );

    roulette_session.kill(5);
    println!("{}", roulette_session);
}