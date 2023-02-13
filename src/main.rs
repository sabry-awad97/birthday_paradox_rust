use chrono::{Datelike, Duration, NaiveDate};
use rand::distributions::Uniform;

use rand::{thread_rng, Rng};

const MONTHS: &[&str] = &[
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

struct BirthdayParadox {
    number_of_birthdays: usize,
}

impl BirthdayParadox {
    fn new(number_of_birthdays: usize) -> Self {
        Self {
            number_of_birthdays,
        }
    }

    fn generate_birthdays(&self) -> Vec<NaiveDate> {
        let start_date = NaiveDate::from_ymd_opt(1997, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(1997, 12, 31).unwrap();

        let range = Uniform::new(0, (end_date - start_date).num_days());
        let mut rng = thread_rng();

        let birthdays: Vec<NaiveDate> = (0..self.number_of_birthdays)
            .map(|_| start_date + Duration::days(rng.sample(range)))
            .collect();

        birthdays
    }

    fn display_birthdays(&self, birthdays: &[NaiveDate]) {
        for (i, birthday) in birthdays.iter().enumerate() {
            if i != 0 {
                print!(", ");
            }

            let month_name = MONTHS[birthday.month() as usize - 1];
            let date_text = format!("{} {}", month_name, birthday.day());
            print!("{}", date_text);
        }

        println!("");
    }

    fn display_results(&self, matches: Option<NaiveDate>) {
        print!("In this simulation, ");
        if let Some(matches) = matches {
            let month_name = MONTHS[matches.month() as usize - 1];
            let date_text = format!("{} {}", month_name, matches.day());
            println!("multiple people have a birthday on {}", date_text);
        } else {
            println!("there are no matching birthdays.");
        }
    }

    fn get_match(&self, birthdays: &[NaiveDate]) -> Option<NaiveDate> {
        let unique = birthdays.iter().collect::<std::collections::HashSet<_>>();
        if unique.len() == self.number_of_birthdays {
            return None;
        }
        for birthday in birthdays.iter() {
            let count = birthdays.iter().filter(|&b| b == birthday).count();
            if count > 1 {
                return Some(*birthday);
            }
        }
        None
    }

    fn run_simulations(&self, times: usize) -> (usize, f64) {
        let mut match_count = 0;
        for i in 0..times {
            if i % 10000 == 0 {
                println!("{} simulations run...", i);
            }
            let birthdays = self.generate_birthdays();
            if self.get_match(&birthdays).is_some() {
                match_count += 1;
            }
        }
        println!("{} simulations run.", times);
        let probability = (match_count as f64) / (times as f64) * 100.0;
        (match_count, probability)
    }
}

fn main() {
    println!(
        "   Birthday Paradox
The birthday paradox shows us that in a group of N people, the odds
that two of them have matching birthdays is surprisingly large.
This program does a Monte Carlo simulation (that is, repeated random
simulations) to explore this concept.
(It's not actually a paradox, it's just a surprising result.)"
    );

    let number_of_birthdays = 23;
    let birthday_paradox = BirthdayParadox::new(number_of_birthdays);
    let birthdays = birthday_paradox.generate_birthdays();
    birthday_paradox.display_birthdays(&birthdays);
    let simulations = 100_000;

    birthday_paradox.display_results(birthday_paradox.get_match(&birthdays));
    let (sim_match, probability) = birthday_paradox.run_simulations(simulations);

    println!(
        "Out of {} simulations of {}, people, there was a",
        simulations, number_of_birthdays
    );
    println!(
        "matching birthday in that group {} times. This means",
        sim_match
    );
    println!(
        "that {} people have a {} % chance of",
        sim_match, probability
    );
    println!("having a matching birthday in their group.");
    println!("That\'s probably more than you would think!");
}
