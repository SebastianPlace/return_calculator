use std::io;

const HOURS_IN_DAY: f64 = 24 as f64;

// Get user inputs:
// - principal - Initial investment / principal: f64
// - compound_hrs - Compounding every x hours: f64
// - rate - Interest rate as a percentage: f64
// - term_days - Term i.e. length of investment in days: f64
fn main() {
    loop {
        let mut principal = String::new();
        let mut compound_hrs = String::new();
        let mut rate = String::new();
        let mut term_days = String::new();

        println!("Enter initial investment (principal):");
        match io::stdin().read_line(&mut principal) {
            Ok(_) => {},
            Err(_) => continue,
        }
        let principal: f64 = match principal.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("How often does it compound in hours?:");
        match io::stdin().read_line(&mut compound_hrs) {
            Ok(_) => {},
            Err(_) => continue,
        }
        let compound_hrs: f64 = match compound_hrs.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("What is the interest rate? (e.g for 6% put 6):");
        match io::stdin().read_line(&mut rate) {
            Ok(_) => {},
            Err(_) => continue,
        }
        let rate: f64 = match rate.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("What is the term of the investment in days:");
        match io::stdin().read_line(&mut term_days) {
            Ok(_) => {},
            Err(_) => continue,
        }
        let term_days: f64 = match term_days.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let estimated_apy = estimate_return(principal, compound_hrs, rate, term_days);

        println!("Your investment of £{} should yield a return of £{}.", principal, estimated_apy);
    }
}

// Calculates your return after the investment term specified
fn estimate_return(principal: f64, compound_hrs: f64, rate: f64, term_days: f64) -> f64 {
    let decimal_rate = percentage_rate_to_decimal(rate);
    let term_years = days_to_years(term_days);
    let compounds_in_term = compounds_per_term(compound_hrs, term_days);
    return principal * (1 as f64 + decimal_rate / compounds_in_term).powf(compounds_in_term * term_years);
}

fn percentage_rate_to_decimal(rate: f64) -> f64 { rate / 100 as f64}
fn days_to_years(days: f64) -> f64 { days / 365 as f64 }
fn compounds_per_day(compound_hrs: f64) -> f64 { HOURS_IN_DAY / compound_hrs }
fn compounds_per_term(compound_hrs: f64, term_days: f64) -> f64 {
    compounds_per_day(compound_hrs) * term_days
}
