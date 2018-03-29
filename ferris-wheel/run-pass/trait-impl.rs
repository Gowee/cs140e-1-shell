// FIXME: Make me pass! Diff budget: 25 lines.

#[derive(Clone, Copy, Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

impl Duration {
    fn as_milliseconds(&self) -> u64 { 
        // enum variants are not first-class type yet
        // so have to return duration, see RFC 1450
        match match *self {
            Duration::Minutes(m) => Duration::Seconds(m as u32 * 60),
            // cannot use Self here, see issues #26264 and #31168
            other => other
        } {
            Duration::Seconds(s) => s as u64 * 1000,
            Duration::MilliSeconds(ms) => ms,
            _ => panic!("Compiler error!") // So far, the compiler cannot understand this sort of type collapsion / shrinking .
        }
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Self) -> bool {
        self.as_milliseconds() == other.as_milliseconds()
    }
}

use Duration::*;
fn main() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
