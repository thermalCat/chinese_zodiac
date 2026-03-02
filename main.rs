use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChineseZodiac {
    Rat,
    Ox,
    Tiger,
    Rabbit,
    Dragon,
    Snake,
    Horse,
    Goat,
    Monkey,
    Rooster,
    Dog,
    Pig,
}

impl ChineseZodiac {
    pub fn from_year(year: i32) -> Self {
        match (year - 2020).rem_euclid(12) {
            0 => Self::Rat,
            1 => Self::Ox,
            2 => Self::Tiger,
            3 => Self::Rabbit,
            4 => Self::Dragon,
            5 => Self::Snake,
            6 => Self::Horse,
            7 => Self::Goat,
            8 => Self::Monkey,
            9 => Self::Rooster,
            10 => Self::Dog,
            11 => Self::Pig,
            _ => unreachable!(),
        }
    }
}

fn main() {
    println!("What year were you born?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let year: i32 = input.trim().parse().expect("Please enter a valid number");

    let zodiac = ChineseZodiac::from_year(year);

    println!("You were born in the year of the {:?}.", zodiac);
}
