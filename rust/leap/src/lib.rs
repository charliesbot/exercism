/*
 * Given a year, report if it is a leap year.
 * The tricky thing here is that a leap year in the Gregorian calendar occurs:
 * on every year that is evenly divisible by 4
 * except every year that is evenly divisible by 100
 * unless the year is also evenly divisible by 400
 * For example, 1997 is not a leap year, but 1996 is. 1900 is not a leap year, but 2000 is.
*/

pub fn is_leap_year(year: u64) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            return year % 400 == 0;
        }
        return true;
    }

    return false;
}
