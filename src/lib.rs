pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

// this funtion calculates the difference between two dates
// the second date is always later than the first
pub fn date_diff(first: Date, second: Date) -> i32 {
    // use a map to store the number of days in each month
    let days_in_month = [
        (1, 31),
        (2, 28),
        (3, 31),
        (4, 30),
        (5, 31),
        (6, 30),
        (7, 31),
        (8, 31),
        (9, 30),
        (10, 31),
        (11, 30),
        (12, 31),
    ];

    // calculate the difference in days
    let _days = 0;
    // calculate the difference in years
    let mut years = second.year - first.year;
    // calculate the difference in months
    let mut months = second.month - first.month;
    // calculate the difference in days
    let mut days = second.day - first.day;

    // if the difference in days is negative
    if days < 0 {
        // subtract one month from the difference in months
        months -= 1;
        // add the number of days in the previous month to the difference in days, using the map
        days += days_in_month[months as usize].1;
    }

    // if the difference in months is negative
    if months < 0 {
        // subtract one year from the difference in years
        years -= 1;
        // add 12 to the difference in months
        months += 12;
    }

    // return the difference of years, months, and days
    years * 365 + months * 30 + days
}
