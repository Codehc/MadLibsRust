use std::io;
use print_templates::{print_title};

mod print_templates;

// Alternate madlibs, commented since theyre not used and will be stored in memory if uncommented which is a waste
/*static UNDER_THE_SEA: &'static str = "{name}, listen to me
The {noun} world, it's a {adjective}
Life under the {noun}
Is better than anything they got up there";*/

static NEVA_GONNA_GIVE: &'static str = "Neva gonna {verb} you {noun}
Neva gonna {verb} you {noun}
Neva gonna {verb} around and {verb} you!";

fn main() {
    // Print the MadLibs title
    print_title();

    println!("\nWelcome to MadLibs by Rezq! Made for APCSA Project 3");

    // Chose the madlib to do
    let mut madlib: String = NEVA_GONNA_GIVE.to_owned();

    let mut index_of_opening_bracket: Option<usize> = madlib.find("{");

    // Infinite loop, will be broken from the inside when no more queries are left to be made
    loop {
        // Pattern match the index_of_opening_bracket option. If it doesn't have a value break the loop
        match index_of_opening_bracket {
            None => {
                break;
            },
            Some(index) => {
                // Get the index of the space after the query
                let index_of_closing_bracket: Option<usize> = madlib[index..].find("}");
                let index_of_closing_bracket: usize = match index_of_closing_bracket {
                    None => madlib.len() - 1,
                    Some(space_index) => space_index + index,
                };

                // Query the player
                let query: &str = &madlib[index + 1..index_of_closing_bracket];
                println!("\nPlease enter a {}:", query);

                let mut input: String = String::new();
                io::stdin().read_line(&mut input).expect("Please enter a valid string.");
                input = input.trim().to_owned();

                // Replace the query with the users input
                madlib = format!("{}{}{}", &madlib[..index], input, &madlib[index_of_closing_bracket + 1..]);

                // Look for next query
                index_of_opening_bracket = madlib.find("{");
            }
        }
    }

    // Print the completed madlib
    println!("\n-----------------------------\nHere is your finished MadLib:\n\n{}", madlib);
}