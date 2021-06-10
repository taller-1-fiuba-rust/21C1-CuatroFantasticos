#[derive(Clone, Copy)]
pub struct Verbose {
    active: bool,
}

impl Verbose {
    pub fn new(status: &str) -> Verbose {
        let mut active = false;
        if status == "1" {
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
