mod eight;
mod eleven;
mod fifteen;
mod five;
mod four;
mod fourteen;
mod nine;
mod one;
mod seven;
mod six;
mod ten;
mod thirteen;
mod three;
mod twelve;
mod two;

pub trait Day {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

pub fn get_day(day: &u8) -> Result<Box<dyn Day>, String> {
    match day {
        1 => Ok(Box::new(one::DayOne::default())),
        2 => Ok(Box::new(two::DayTwo::default())),
        3 => Ok(Box::new(three::DayThree::default())),
        4 => Ok(Box::new(four::DayFour::default())),
        5 => Ok(Box::new(five::DayFive::default())),
        6 => Ok(Box::new(six::DaySix::default())),
        7 => Ok(Box::new(seven::DaySeven::default())),
        8 => Ok(Box::new(eight::DayEight::default())),
        9 => Ok(Box::new(nine::DayNine::default())),
        10 => Ok(Box::new(ten::DayTen::default())),
        11 => Ok(Box::new(eleven::DayEleven::default())),
        12 => Ok(Box::new(twelve::DayTwelve::default())),
        13 => Ok(Box::new(thirteen::DayThirteen::default())),
        14 => Ok(Box::new(fourteen::DayFourteen::default())),
        15 => Ok(Box::new(fifteen::DayFifteen::default())),

        _ => Err(format!("Day {} not supported.", day)),
    }
}
