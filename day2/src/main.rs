fn main() {
  //match instead of if..else like switch
  let day= 2;
  let result = match day {
    1 | 2 | 3 | 4 | 5  => "WeekDays",
    6 | 7 => "Weekends",
    _ => "Invalid Day",
  };

  println!("{result}");
}