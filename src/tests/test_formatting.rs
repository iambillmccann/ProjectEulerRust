
#[cfg(test)]
pub mod test_formatting {
    use super::super::super::euler_library::utilities;

    #[test]
    fn milliseconds_less_than_10() {
        assert_eq!(utilities::format_milliseconds(6),"00:00:00:006");
    }

    #[test]
    fn milliseconds_less_than_100() {
        assert_eq!(utilities::format_milliseconds(76),"00:00:00:076");
    }
    
    #[test]
    fn seconds_less_than_10() {
        assert_eq!(utilities::format_milliseconds(5000),"00:00:05:000");
    }
    
    #[test]
    fn minutes_less_than_10() {
        assert_eq!(utilities::format_milliseconds(180000),"00:03:00:000");
    }
    
    #[test]
    fn hours_less_than_10() {
        assert_eq!(utilities::format_milliseconds(7200000),"02:00:00:000");
    }
    
    #[test]
    fn milliseconds_not_more_than_999() {
        assert_eq!(utilities::format_milliseconds(1000),"00:00:01:000");
    }
    
    #[test]
    fn seconds_less_than_60() {
        assert_eq!(utilities::format_milliseconds(60000),"00:01:00:000");
    }
    
    #[test]
    fn minutess_less_than_60() {
        assert_eq!(utilities::format_milliseconds(3600000),"01:00:00:000");
    }
    
    #[test]
    fn hours_more_than_99() {
        assert_eq!(utilities::format_milliseconds(360000000),"100:00:00:000");
    }
    
}