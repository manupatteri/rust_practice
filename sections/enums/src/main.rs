enum DayOfTheWeek {
	SUNDAY,
	MONDAY,
	TUESDAY,
	WEDNESDAY,
	THURSDAY,
	FRIDAY,
	SATURDAY,
}
fn main() {
    println!("Hello, world! {}", is_weekend(DayOfTheWeek::SUNDAY));
    println!("Hello, world! {}", is_weekend(DayOfTheWeek::TUESDAY));

}
fn is_weekend(day_of_the_week: DayOfTheWeek) -> bool {
	if matches!(day_of_the_week,DayOfTheWeek::SUNDAY) || matches!(day_of_the_week,DayOfTheWeek::SUNDAY) {
		true
	} else {
		false
	}
	
}
	
