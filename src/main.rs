use std::thread;
use std::time::Duration;

const MARK: [&str; 4] = ["-", "\\", "|", "/"];

#[derive(Debug)]
struct Displaytime {
    second: u32,
    minute: u32,
    hour: u32,
    second_str: String,
    minute_str: String,
    hour_str: String,
}

impl Displaytime {
    fn display(&self) {
        let pos = self.second as usize % MARK.len();
        eprint!(
            "{} {}:{}:{}\r",
            MARK[pos], self.hour_str, self.minute_str, self.second_str
        );
    }

    fn inclement_time(&mut self) {
        self.second = self.second + 1;
        // second reset
        if self.second == 60 {
            self.minute = self.minute + 1;
            self.second = 0;
        }
        // minute reset
        if self.minute == 60 {
            self.hour = self.hour + 1;
            self.minute = 0;
        }
    }

    fn shaping_str(&mut self) {
        // second string shaping
        if self.second <= 9 {
            self.second_str = "0".to_string() + &self.second.to_string();
        } else {
            self.second_str = self.second.to_string();
        }

        // minute string shaping
        if self.minute <= 9 {
            self.minute_str = "0".to_string() + &self.minute.to_string();
        } else {
            self.minute_str = self.minute.to_string();
        }

        // hour string shaping
        if self.hour <= 9 {
            self.hour_str = "0".to_string() + &self.hour.to_string();
        } else {
            self.hour_str = self.hour.to_string();
        }
    }
}

fn main() {
    let mut timedisp = Displaytime {
        second: 0,
        minute: 0,
        hour: 0,
        second_str: "00".to_string(),
        minute_str: "00".to_string(),
        hour_str: "00".to_string(),
    };
    let du = Duration::new(1, 0);
    loop {
        timedisp.display();
        thread::sleep(du);
        timedisp.inclement_time();
        timedisp.shaping_str();
    }
}
