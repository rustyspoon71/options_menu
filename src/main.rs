use std::io;
use std::process;
use std::thread::current;

// Declare your Struct

#[derive(Clone, Copy)]
struct Settings{
    pub resolution_percentage: i32,
    pub dx11: bool,
    pub v_sync: bool,
    pub sound_level: i32,
    pub voice_chat: i32,
    pub party_chat: i32,
    pub music_volume: i32,
    pub sensitivity: i32,

}

// Implement methods.


// implement a new function will act as a constructor.
impl Settings{
    fn new() -> Self{
        Settings{
            resolution_percentage: 100,
            dx11: true,
            v_sync: true,
            sound_level: 100,
            voice_chat: 100,
            party_chat: 100,
            music_volume: 100,
            sensitivity: 10,
        }
    }
    // Implement a function to display values being change.
    fn display(&self) {

        println!("Resolution Percentage: {} ", self.resolution_percentage);
        println!("Graphics Mode dx11: {} ", self.dx11);
        println!("V-sync: {}", self.v_sync);
        println!("Game Volume: {}", self.sound_level);
        println!("Voice chat Volume: {}", self.voice_chat);
        println!("Party chat Volume: {}",self.party_chat);
        println!("Music Volume: {}",self.music_volume);
        println!("Controller Sensitivity: {}",self.sensitivity);
    }

    // Create a function to display values in adjusting format.
    fn adjust_display(&self)
    {

        println!("Resolution Percentage - 1");
        println!("Graphics Mode: - 2 ");
        println!("V-sync: - 3");
        println!("Game Volume: - 4");
        println!("Voice chat Volume: - 5");
        println!("Party chat Volume: - 6");
        println!("Music Volume: - 7");
        println!("Controller Sensitivity - 8");

    }
}








// Will act as driver of the program.
fn main() {

    // initialize the struct make sure it is mutable.
    let mut current_settings = Settings::new();

    // Print the menu.
    println!(" ");
    println!("Welcome to my settings outline.");
    println!("The menu is below enter you're selection.");

    // initialize the sentinel and begin menu loop.
    let mut _n : i32 = 10;
    while _n != 0
    {
        println!("  ");
        println!("  ");
        println!("Options");
        println!("Adjust Setting - 1");
        println!("Display all setting Values - 2");
        println!("Enter your option or 0 to quit:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        _n = input.trim().parse().unwrap();

        // use match statement for user input sorting.
        match _n {
            0 => adjust_settings(_n, &mut current_settings),
            1 => adjust_settings(_n, &mut current_settings),
            2 => adjust_settings(_n, &mut current_settings),

            _ => println!("Incorrect value entered.")
        }
    }
}

// will adjust user settings based on input.
fn adjust_settings(choice:i32, _settings: &mut Settings)
{
    // Initialize all variables that will be needed.

    let mut _adjust: i32;
    let mut _change_value: i32;

    // Initialize all input values for the buffers.

    let mut input_bool = String::new();
    let mut input_value = String::new();
    let mut input = String::new();

    // Case where they want to adjust settings.
    if choice == 1 {
        // Display the adjust menu.

        _settings.adjust_display();

        // Get the change choice.

        println!("Enter the setting option you want to adjust.");
        io::stdin().read_line(&mut input).unwrap();
        _adjust = input.trim().parse().unwrap();

        // The resolution setting and all the rest of the statements for each setting..
        if _adjust == 1
        {
             println!("What would you like to set the resolution setting to? 1-100");
             io::stdin().read_line(&mut input_value).unwrap();
             _change_value = input_value.trim().parse().unwrap();
             _settings.resolution_percentage = _change_value;

        }
        else if _adjust == 2
        {
            println!("If you would like dx11 on enter -> 1 if not enter any other number");
            io::stdin().read_line(&mut input_bool).unwrap();
            _change_value = input_bool.trim().parse().unwrap();
            if _change_value == 1
            {
                _settings.dx11 = true;
            }
            else { _settings.dx11 = false; }
        }
        else if _adjust == 3
        {
            println!("If you would like V-sync on enter -> 1 if not enter any other number");
            io::stdin().read_line(&mut input_bool).unwrap();
            _change_value = input_bool.trim().parse().unwrap();
            if _change_value == 1
            {
                _settings.v_sync = true;
            }
            else { _settings.v_sync = false; }

        }
        else if _adjust == 4
        {
            println!("Enter a value for Game Sound between 0-100");
            io::stdin().read_line(&mut input_value).unwrap();
            _change_value = input_value.trim().parse().unwrap();

            _settings.sound_level = _change_value;

        }
        else if _adjust == 5
        {
            println!("Enter a value for Voice Chat between 0-100");
            io::stdin().read_line(&mut input_value).unwrap();
            _change_value = input_value.trim().parse().unwrap();

            _settings.voice_chat = _change_value;
        }
        else if _adjust == 6
        {
            println!("Enter a value for Party Chat between 0-100");
            io::stdin().read_line(&mut input_value).unwrap();
            _change_value = input_value.trim().parse().unwrap();

            _settings.party_chat = _change_value;


        }
        else if _adjust == 7
        {
            println!("Enter a value for Music Volume between 0-100");
            io::stdin().read_line(&mut input_value).unwrap();
            _change_value = input_value.trim().parse().unwrap();

            _settings.music_volume = _change_value;

        }
        else if _adjust == 8
        {
            println!("Enter a value for Controller Sensitivity between 0-10");
            io::stdin().read_line(&mut input_value).unwrap();
            _change_value = input_value.trim().parse().unwrap();

            _settings.sensitivity = _change_value;


        }
        else {
            println!("Invalid setting number please re-enter.")

        }



    }
    // If choice 2 they want to see the settings value display it.
    else if choice == 2{
        _settings.display();
    }
    // If the choice is 0 display an exit message.
    else if choice == 0
    {
        println!("Now Exiting the menu.");


    }

}
