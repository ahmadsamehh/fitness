use fitness::cardio; // Import the cardio module from the fitness crate.
use fitness::weighlifting; // Import the weighlifting module from the fitness crate.
use fitness::GymWorkout; // Import the GymWorkout struct from the fitness crate.

fn main() {
    // Create a new weighlifting exercise called "Bench Press" with 8 reps.
    let wlexercise = weighlifting::Exercise::new(String::from("Bench Press"), 8);

    // Create a new cardio exercise called "Thursday" using a Bike for 30 minutes.
    let cexercise = cardio::Exercise::new(String::from("Thursday"), cardio::CardioTool::Bike, 30);

    // Create a new GymWorkout combining the weighlifting and cardio exercises.
    let myworkout = GymWorkout::new(wlexercise, cexercise);

    // Print the details of the GymWorkout using pretty print.
    println!("{:#?}", myworkout);
}
