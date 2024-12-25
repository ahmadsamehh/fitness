pub const PERSONAL_TRAINER: &str = "Will Weight";
#[derive(Debug)]
pub struct Exercise {
    name: String,
    reps: u32,
}
impl Exercise {
    pub fn new(name: String, reps: u32) -> Self {
        Self { name, reps }
    }
}
pub fn ask_about_program() {
    println!("the weightlifting trainer is {}", PERSONAL_TRAINER);
}
