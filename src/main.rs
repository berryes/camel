use std::{string, panic::panic_any};


#[derive(Debug,Copy, Clone)]
struct game_stats {
    player_pos: i16,
    badguys_pos: i16,
    remaining_km:i16,
    bottle: i8, // i8 since value wont go above 255
    player_thirst: i8,
    player_exhaust: i8,
}

#[derive(Debug, Copy, Clone)]
enum Steps{
    Step, // steps forward 5-12 km 
    Faststep, // steps forward 10-20 km
    Stop, // stops for the nigth
    Drink, // drinks from the kulacs
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


     // game goes untill player is not at end
     while stats.player_pos < 200 {
        let mut nextstep: Vec<Steps> = get_next_step(stats);
        println!("PPOS:{} | BGUYPOS: {} | BOTTLE {} | EXHAUST: {} | THIRS: {}", stats.player_pos,stats.badguys_pos,stats.bottle,stats.player_exhaust,stats.player_thirst);
        let next:Steps = select(nextstep.clone()); 
        
        // match for next steps

        println!("{:?}",nextstep);
     }


     // ends
     println!("You won");
}

fn get_next_step(stats:game_stats) -> Vec<Steps>{

    
    // full of enums
    let mut options: Vec<Steps> = Vec::new();

    // DETERMINE WHAT THE PLAYER CAN do 

    // exhausted and has to stop for the nigth, no other option
    if stats.player_exhaust + 1 > 8 {
        options.push( Steps::Stop );
        return options
    }

    // thirst is at max
    if stats.player_thirst + 1 > 6 {

       //  can drink
        if stats.bottle - 1 == 0 {
        options.push( Steps::Drink );
        return options;
        }

       //  CANT drink
       else {
        options.push( Steps::Stop ); // has to stop for tonigth
        return  options;
        } 
    }

    options.push( Steps::Step );
    options.push( Steps::Faststep );


    // print out the choises
    // return what the player have chosen
    return  options;
}



fn get_input(changer: &mut String){
    std::io::stdin().read_line(changer).unwrap();
    changer.strip_suffix("\n");
}
fn select(avail_steps:Vec<Steps>) -> Steps{

    // all the available step's string will be added here, then printed
    let mut prinln: String = String::new(); 
    // user's console input stored here
    let mut console:String = String::new();
    
    // checking what the user can do
    for step in avail_steps {
        match step {
            Steps::Step => prinln.push_str("A, Walk forward"),
            Steps::Faststep => prinln.push_str("B, Walk forward fast"),
            Steps::Stop => prinln.push_str("C, Stop for tonigth"),
            Steps::Drink => prinln.push_str("D, Drink")
        }

        prinln.push_str("\n");
    }

    // Getting input/nextstep from user
    println!("Choose your next step:\n {}",prinln);
    get_input(&mut console);

    // user mistype
    while console.trim_end().len() > 1 {
        console.clear();
        println!("Choose your next step:\n {}",prinln);
        get_input(&mut console);
    }

    while !vec!["A","B","C","D"].contains( &console.as_str().trim_end() ) {
        console.clear();
        println!("INVALLID | Choose your next step:\n {}",prinln);
        get_input(&mut console);
    }

    // Returning shit as Step enum
    match console.as_str() {
        "A" => return  Steps::Step,
        "B" => return Steps::Faststep,
        "C" => return Steps::Stop,
        "D" => return Steps::Drink,
        _ => panic!("idk what happened")
    }

}