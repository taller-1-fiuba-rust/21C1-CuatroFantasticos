pub struct Verbose {
    active: bool,
}

impl Verbose {
    pub fn new(asd: &str) -> Verbose {
        let mut active = false;
        if asd == "1" {
            active = true;
        }
        Verbose { active }
    }

    pub fn print(&self, text: &str) {
        if self.active {
            println!("{}", text);
        }
    }
}