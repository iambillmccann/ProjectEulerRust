pub mod problems;
pub mod math_library;

// mod premutations {
//    fn count() { }
// }

// mod really_big_int {
//     fn addition() { }
// }

pub mod utilities {

    pub fn format_milliseconds(total_milliseconds: i64) -> String {
        let milliseconds = total_milliseconds % 1000;
        let totalseconds = total_milliseconds / 1000;
        let seconds = totalseconds % 60;
        let totalminutes = totalseconds / 60;
        let minutes = totalminutes % 60;
        let hours = totalminutes / 60;

        let mut format_seconds = seconds.to_string();
        let mut format_minutes = minutes.to_string();
        let mut format_hours = hours.to_string();

        let format_milliseconds: String = if milliseconds < 10 {
            "00".to_owned() + &milliseconds.to_string()
        } else if milliseconds < 100 {
            "0".to_owned() + &milliseconds.to_string()
        } else { milliseconds.to_string() };

        if seconds < 10 { format_seconds = "0".to_owned() + &format_seconds; }
        if minutes < 10 { format_minutes = "0".to_owned() + &format_minutes; }
        if hours < 10 { format_hours = "0".to_owned() + &format_hours; }

        format_hours + ":" + 
        &format_minutes + ":" +
        &format_seconds + ":" +
        &format_milliseconds

    }

}
