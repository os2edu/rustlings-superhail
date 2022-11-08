// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a hint.

use std::{num::ParseIntError};

fn main() -> Result<(), String> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input);

    match cost {
        Ok(total_cost) => {
            if total_cost > tokens {
                Err("You can't afford that many!".into())
            } else {
                tokens -= total_cost;
                Ok(())
            }
        },
        Err(error) => Err("ParseIntError".into())
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();

    match qty {
        Ok(num) => Ok(num * cost_per_item + processing_fee),
        Err(error) => Err(error)
    }
}
