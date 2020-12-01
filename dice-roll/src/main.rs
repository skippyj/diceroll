fn roll_dice(inp: String) {
    use regex::Regex;
    use rand::Rng;
    // "r" prefix denotes raw string, basically automatic backslash escaping
    let val = &inp.trim();
    let rx = Regex::new(r"(?P<amount>\d+)(d)(?P<sides>\d+)").unwrap();
    let grp = rx.captures(val).unwrap();
    let amount = &grp["amount"];
    let sides = &grp["sides"];
    let mut rng = rand::thread_rng(); // Initialize rng obj
    
    println!("\nRolled {}", &val);
    for i in 0..amount.parse::<i32>().unwrap() {
        println!("Dice {}: {}", i, rng.gen_range(0, sides.parse::<i32>().unwrap()));
    }
}

fn main() {
    use std::io::{stdin,stdout,Write};
    let mut input = String::new(); // Mut is a mutatable value
    print!("Select your dice (2d6, 1d20, etc)\n");
    let _ = stdout().flush(); // Flush buffer, immediate print
    stdin().read_line(&mut input).expect("Invalid text");
    roll_dice(input)
}
