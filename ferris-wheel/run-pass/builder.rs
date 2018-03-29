// FIXME: Make me pass! Diff budget: 30 lines.

struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    fn default() -> Self {
        Builder {
            string: None::<String>,
            number: None::<usize>
        }
    }

    fn string<T: Into<String>>(mut self, string: T) -> Self {
        self.string = Some(string.into());
        self
    }

    fn number(mut self, number: usize) -> Self {
        self.number = Some(number);
        self
    }

    fn to_string(&self) -> String {
        //format!("{} {}", self.string.unwrap(), self.number.unwrap()).to_owned()
        let mut ret = String::new();
        if let Some(ref string) = self.string {
            ret.push_str(string);
        }
        if let Some(ref number) = self.number {
            if !ret.is_empty() {
                ret.push_str(" ");
            }
            ret.push_str(number.to_string().as_ref());
        }
        ret
    }
}

// Do not modify this function.
fn main() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default()
        .string("heap!".to_owned())
        .to_string();

    assert_eq!(c, "heap!");
}
