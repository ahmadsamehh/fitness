pub mod cardio;
pub mod weighlifting;

use cardio::Exercise as CardioExercise;
use weighlifting::Exercise as WeightLiftingExercise;

#[derive(Debug)]
pub struct GymWorkout {
    pub weightlifting: WeightLiftingExercise,
    pub cardio: CardioExercise,
}

impl GymWorkout {
    pub fn new(weightlifting: WeightLiftingExercise, cardio: CardioExercise) -> Self {
        cardio::ask_about_program();
        weighlifting::ask_about_program();
        Self {
            weightlifting,
            cardio,
        }
    }
}
