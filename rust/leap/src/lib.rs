pub fn is_leap_year(year: u64) -> bool {
    matches!(year % 400, 0) ||  ( matches!(year % 4, 0) && !matches!(year % 100, 0))
}
