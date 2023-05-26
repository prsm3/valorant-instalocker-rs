use std::io;
use inputbot::{KeybdKey::*, MouseButton::*, MouseCursor};
use std::{thread::sleep, time::Duration};

struct Coordinates {
    x: i32,
    y: i32
}

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y
        }
    }
    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    pub fn get(&self) -> String {
        let result = self.x.to_string() + ", " + &self.y.to_string();
        result
    }
    pub fn get_index(&self, index: i8) -> i32 {
        if index == 0 {
            return self.x;
        }
        if index == 1 {
            return self.y;
        }
        else {
            panic!()
        }
    }
}

fn main() {
    // list of all agents
    let agent_list: Vec<&str> = vec!["astra", "breach", "brimstone", "chamber", 
    "cypher", "fade", "gekko", "harbor", "jett", "kayo", "killjoy", "neon", "omen", "phoenix", "raze", 
    "reyna", "sage", "skye", "sova", "viper", "yoru"];

    println!("--Valorant Instalocker--");

    // lists all the agents with their respective index
    for (index, item) in agent_list.iter().enumerate() {
        println!("{}: {}", index, item);
    }
    println!("choose your agent: ");

    // reading the index from terminal
    let mut agent_string = String::new();
    io::stdin()
        .read_line(&mut agent_string)
        .expect("couldn't read from stdin");
    let agent_int: i32 = agent_string.trim().parse().expect("Not a valid int");
    let mut agent_coords: Coordinates = Coordinates::new(10, 10);

    // matching the index and setting the coordinates
    match agent_int {
        0 => Coordinates::set(&mut agent_coords, 540, 930),
        1 => Coordinates::set(&mut agent_coords, 630, 930),
        2 => Coordinates::set(&mut agent_coords, 710, 930),
        3 => Coordinates::set(&mut agent_coords, 800, 930),
        4 => Coordinates::set(&mut agent_coords, 880, 930),
        5 => Coordinates::set(&mut agent_coords, 965, 930),
        6 => Coordinates::set(&mut agent_coords, 1045, 930),
        7 => Coordinates::set(&mut agent_coords, 1130, 930),
        8 => Coordinates::set(&mut agent_coords, 1215, 930),
        9 => Coordinates::set(&mut agent_coords, 1300, 930),
        10 => Coordinates::set(&mut agent_coords, 1380, 930),
        11 => Coordinates::set(&mut agent_coords, 540, 1000),
        12 => Coordinates::set(&mut agent_coords, 630, 1000),
        13 => Coordinates::set(&mut agent_coords, 710, 1000),
        14 => Coordinates::set(&mut agent_coords, 800, 1000),
        15 => Coordinates::set(&mut agent_coords, 880, 1000),
        16 => Coordinates::set(&mut agent_coords, 965, 1000),
        17 => Coordinates::set(&mut agent_coords, 1045, 1000),
        18 => Coordinates::set(&mut agent_coords, 1130, 1000),
        19 => Coordinates::set(&mut agent_coords, 1215, 1000),
        20 => Coordinates::set(&mut agent_coords, 1300, 1000),
        i32::MIN..=0_i32 | 19_i32..=i32::MAX => println!("number out of scope!")
    }
    // mainly for debug
    println!("coords for agent: {}", agent_coords.get());

    // select button coordinates (default)
    let select_button = Coordinates::new(960, 815);
    println!("hold [caps_lock] to use the instalocker");
    println!("press [q] to exit the program");

    // creating variables for the functions that move the mouse
    let x_coord_agent: i32 = Coordinates::get_index(&agent_coords, 0);
    let y_coord_agent: i32 = Coordinates::get_index(&agent_coords, 1);
    let x_coord_button: i32 = Coordinates::get_index(&select_button, 0);
    let y_coord_button: i32 = Coordinates::get_index(&select_button, 1);

    // binding the movement to [caps_lock]
    CapsLockKey.bind(move || {
        while CapsLockKey.is_pressed() {
            MouseCursor::move_abs(x_coord_agent, y_coord_agent);
            sleep(Duration::from_millis(10));
            LeftButton.press();
            LeftButton.release();
            sleep(Duration::from_millis(10));
            MouseCursor::move_abs(x_coord_button, y_coord_button);
            sleep(Duration::from_millis(10));
            LeftButton.press();
            LeftButton.release();
            sleep(Duration::from_millis(10));
        }
    });

    // binding the exit function to [q]
    QKey.bind(|| {
        std::process::exit(0);
    });
    
    // starts listening for key events
    inputbot::handle_input_events();
}
