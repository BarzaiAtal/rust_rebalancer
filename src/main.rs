// Input/output libraries
use std::io; 
// Vectors
use std::vec;

pub struct Ticker {
    symbol: String,
    desired_percent: u8,
    actual_percent: i32,
    dollar_value: i64,
}

// For getting intial data
fn initial_getter() {
    println!("Initial Getter, standing by.\n");
}

// For calculating the differences
fn difference_getter() {
    println!("Difference Getter, standing by.\n");
}

fn main() {
    // Vectors to hold variables
    let mut ticker_symbols: Vec<String> = Vec::new();       // Holds ticker symbols
    let mut ticker_percents: Vec<f64> = Vec::new();         // Holds actual ticker percentage
    let mut dollars_difference: Vec<f64> = Vec::new();      // Hodls the difference between the actual and desired
    let mut holds: Vec<u8> = Vec::new();    // Vector to hold holds
    let mut sells: Vec<u8> = Vec::new();    // Vector to hold sells
    let mut buys: Vec<u8> = Vec::new();     // Vector to hold buys
    // Position count
    let mut portfolio_count: u8 = 0;
    // Float to hold portfolio value
    let mut portfolio_value: f64 = 0.0;

    // Get position count
    // Query for number of positions and declare string to hold initial input string
    println!("How many positions are in this portfolio? ");
    let mut line: String = String::new();
    // Assemblage to read user input
    io::stdin()
        // Read the line
        .read_line(&mut line)
        // In case input is bad
        .expect("That is not valid input.");
    // Number that holds the counter
    // Trim string to get rid of anything extra
    let trimmed = line.trim();
    // See if string can be parsed to integer, assign or error
    match trimmed.parse::<u8>() {
        Ok(i) => portfolio_count = i,
        Err(..) => println!("This is not an integer: {}", trimmed)
    };

    // Get initial data
    initial_getter();
    // Calculate dollar differences
    difference_getter();

    // Debug stuff
    println!("{}", portfolio_count);
}