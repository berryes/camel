struct game_stats {
    player_pos: i16,
    badguys_pos: i16,
    remaining_km:i16,
    bottle: i8, // i8 since value wont go above 255
    player_thirst: i8,
    player_exhaust: i8,
}
struct allowed_steps {
    can_drink: bool,
    has_to_drink: bool,
}

fn determine_steps(stats:game_stats) -> allowed_steps {

    let mut allowed:allowed_steps = allowed_steps {  
        can_drink: true,
        has_to_drink: false
    };
    
    // player is out of water and is exhausted
    if stats.bottle == 0 && stats.player_thirst + 1 > 6  {
        allowed.can_drink = false
    }

    // player is out of stamina
    if stats.player_exhaust + 1 == 9 {
        allowed.has_to_drink = true
    }


    return allowed;

}
fn main() {
    // memory of game | starting stats
    let mut stats:game_stats = game_stats { 
        player_pos: 0, 
        badguys_pos: -20, 
        remaining_km: 200, 
        bottle: 3,
        player_exhaust: 0,
        player_thirst: 0,
     };

     // defining what step the player can make
     let allow: allowed_steps = determine_steps(stats);

     let next_step: i8 = selector(allow);

     // game goes untill player is not at end
     while stats.player_pos < 200 {

        
     }

     // ends
     println!("You won");
}

fn selector(allowed: allowed_steps) -> i8{

    let mut print_string: String = String::new();

    // player must drink since they cant step forward
    if allowed.can_drink && !allowed.can_step{
        
    }
    if allowed.can_drink && allowed.can_step {

    }

    let options:Vec<String> = vec![
        String::from("A, Ivás a kulacsodból"),
        String::from("B, Tovább haladás"),
        String::from("C, Tovább haladás sietve"),
        String::from("D, Megállás egy éjszakára"),
        String::from("Q, Kilépés")
    ];

    let mut line:String = String::new();

    std::io::stdin().read_line(&mut line).unwrap();

    
    return 3;
}