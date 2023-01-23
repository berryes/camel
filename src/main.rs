use std::{process::exit, option};

use rand::Rng;
use terminal_size::terminal_size;
use unicode_width::UnicodeWidthStr;
fn rng( from: i16, to:i16) -> i16{
    let mut rng = rand::thread_rng();
    return rng.gen_range(from..to);
}

fn rng_i8( from: i8, to:i8) -> i8{
    let mut rng = rand::thread_rng();
    return rng.gen_range(from..to);
}

#[derive(Debug,Copy, Clone)]
struct game_stats {
    player_pos: i16,
    badguys_pos: i16,
    remaining_km:i16,
    bottle: i8, // i8 since value wont go above 255
    player_thirst: i8,
    player_exhaust: i8,
}
trait Actions {
    fn step(self);
}
impl Actions for game_stats{
    fn step(mut self) {
        self.player_thirst += 1;
        self.player_exhaust += 1;
        self.player_pos += rng(5,12);
        self.badguys_pos += rng(7, 14);
    }
}


#[derive(Debug, Copy, Clone)]
enum Steps{
    Step, // steps forward 5-12 km 
    Faststep, // steps forward 10-20 km
    Stop, // stops for the nigth
    Drink, // drinks from the kulacs
}



fn main() {

    let tsize = terminal_size()
    .expect("Failed to read the terminal diameters");

    let teststr = "🐫";
    let width = UnicodeWidthStr::width(teststr);
    
    let dirt:String = String::new();



    // memory of game | starting stats
    let mut stats:game_stats = game_stats { 
        player_pos: 0, 
        badguys_pos: -20, 
        remaining_km: 200, 
        bottle: 3,
        player_exhaust: 0,
        player_thirst: 0,
     };


     // game goes untill player is not at end
     while stats.player_pos < 200 {

        // badguys catch up
        if stats.player_pos == stats.badguys_pos || stats.player_pos < stats.badguys_pos {
            println!("The bandits have cougth up to you, YOU LOST");
            return;
        }


        let mut nextstep: Vec<Steps> = get_next_step(stats);
        // no steps to be made, kill the game
        if nextstep.len() == 0 || nextstep.len() < 0 { return; }


        println!("PPOS:{} | BGUYPOS: {} | BOTTLE {} | EXHAUST: {} | THIRS: {}", stats.player_pos,stats.badguys_pos,stats.bottle,stats.player_exhaust,stats.player_thirst);
    
        let next:Steps = select(nextstep.clone()); 
    
        match next {
            Steps::Step => { /* stats.step() */
                stats.player_thirst += 1;
                stats.player_exhaust += 1;
                stats.player_pos += rng(5,12);
                stats.badguys_pos += rng(7, 14);
            },

            Steps::Faststep =>{
                stats.player_thirst += 1;
                stats.player_exhaust += rng_i8(1, 3);
                stats.player_pos += rng(10,20);
                stats.badguys_pos += rng(7, 14);
            },

            Steps::Stop => {
                stats.player_exhaust = 0;
                stats.badguys_pos += rng(7, 14);
            },

            Steps::Drink => {
                stats.player_thirst = 0;
                stats.bottle += -1;
            }
        }
     }


     // ends
     println!("You won");
}

fn get_next_step(stats:game_stats) -> Vec<Steps>{    
    // full of enums
    let mut options: Vec<Steps> = Vec::new();

    // DETERMINE WHAT THE PLAYER CAN do 


    // exhausted and has to stop for the nigth, no other option
    if stats.player_exhaust == 8 || stats.player_exhaust > 8 {
        println!("Too tired to go forward, get some sleep");
        options.push( Steps::Stop );
        return options
    }

    // too thirsty
    if stats.player_thirst == 6 {
        println!("Too thirsty to go forward, you died");
        return options
    }

    if stats.bottle > 0 {
        options.push( Steps::Drink )
    }


    options.push( Steps::Step );
    options.push( Steps::Faststep );

    
    return  options;
}



fn get_input(changer: &mut String){
    std::io::stdin().read_line(changer).unwrap();
    changer.strip_suffix("\n"); // removing stupid enter
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
        "A\n" => return  Steps::Step,
        "B\n" => return Steps::Faststep,
        "C\n" => return Steps::Stop,
        "D\n" => return Steps::Drink,
        &_ => return Steps::Step
    }


}