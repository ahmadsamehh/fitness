/// Defines the name of the personal trainer for the cardio program.
pub const PERSONAL_TRAINER: &str = "Cardl Cardio";

/// Prints the name of the personal trainer for the cardio program.
pub fn ask_about_program() {
    println!("the cardio trainer is {}", PERSONAL_TRAINER);
}

/// Represents the different types of cardio equipment that can be used.
#[derive(Debug)]
pub enum CardioTool {
    Treadmill,
    Bike,
}

/// Represents a cardio exercise session.
#[derive(Debug)]
pub struct Exercise {
    /// The day the exercise was performed.
    pub day: String,
    /// The type of cardio equipment used.
    pub tool: CardioTool,
    /// The duration of the exercise in minutes.
    pub minutes: u32,
}

impl Exercise {
    /// Creates a new `Exercise` instance.
    pub fn new(day: String, tool: CardioTool, minutes: u32) -> Self {
        Self { day, tool, minutes }
    }
}
pub const PERSONAL_TRAINER: &str = "Cardl Cardio";
pub fn ask_about_program() {
    println!("the cardio trainer is {}", PERSONAL_TRAINER);
}

#[derive(Debug)]
pub enum CardioTool {
    Treadmill,
    Bike,
}

#[derive(Debug)]
pub struct Exercise {
    pub day: String,
    pub tool: CardioTool,
    pub minutes: u32,
}

impl Exercise {
    pub fn new(day: String, tool: CardioTool, minutes: u32) -> Self {
        Self { day, tool, minutes }
    }
}
