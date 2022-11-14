// START IN lib.rs!

use anyhow::{Context, Result};
use aquarium::Dolphin;
// Silence some warnings so they don't distract from the exercise.
#[allow(clippy::vec_init_then_push)]

// (You already did #1 in lib.rs, right?)
//
// 2a. Uncomment and finish the play_time function below
// - Bring anyhow::Result into scope with a `use` statement
// - Have the play_time function return a `Result<Vec<String>>`. The vector of Strings will
//   represent successful outcomes of various dolphin tricks.

fn play_time(dolphin: &Dolphin) -> Result<Vec<String>, anyhow::Error> {
    let mut responses = vec![];
    // 2b. Call the .say_your_name() method on `dolphin`, use `?` to unwrap the value, and push
    // the value onto the `responses` vector.
    //
    let response = dolphin.say_your_name()?; // this can be done with an intermediate variable...
    responses.push(response); // ...or all on one line. Either way is fine!
                              //
                              // 2c. Do the same thing as #2b for the .flip() method
    let response: String = dolphin.flip()?;
    responses.push(response); // ...or all on one line. Either way is fine!

    // 2d. Do the same thing as #2b for the .shake_hands() method
    let response: String = dolphin.shake_hands()?;
    responses.push(response); // ...or all on one line. Either way is fine!

    Ok(responses)
}

fn main() -> Result<(), anyhow::Error> {
    let dolphins = vec![
        Dolphin {
            name: "Augustinius".into(),
            age: 7,
            hungry: false,
        },
        Dolphin {
            name: "Bitty".into(),
            age: 2,
            hungry: true,
        },
        Dolphin {
            name: "Carson".into(),
            age: 5,
            hungry: true,
        },
        Dolphin {
            name: "Devin".into(),
            age: 6,
            hungry: false,
        },
    ];
    for dolphin in &dolphins {
        let responses: Vec<String> =
            play_time(dolphin).context(format!("{} can't perform today: ", dolphin.name))?;
        println!("{} did a FABULOUS PERFORMANCE!", dolphin.name);
        for response in responses {
            println!("  {}", response);
        }
        println!();
    }
    Ok(())
}
