use std::env;
use std::thread;
use std::time::Duration;

const MARK: [&str; 4] = ["-", "\\", "|", "/"];
const NUMBER: [[i32; 15]; 10] = [
    [1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1],
    [0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
    [1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1],
    [1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1],
    [1, 0, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 1],
    [1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1],
    [1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1],
    [1, 1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
    [1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1],
    [1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1],
];

#[derive(Debug)]
struct Displaytime {
    second: i32,
    minute: i32,
    hour: i32,
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

    fn declement_time(&mut self) -> bool {
        let mut finish_flg = false;
        self.second = self.second - 1;
        if self.second == -1 {
            self.minute = self.minute - 1;
            self.second = 59;
        }

        if self.minute == -1 {
            self.hour = self.hour - 1;
            self.minute = 59;
        }

        if self.hour == -1 {
            finish_flg = true;
        }

        finish_flg
    }
}

fn option_select() -> (bool, bool, bool, u32) {
    let mut timer_flg: bool = false;
    let mut stopwatch_flg: bool = false;
    let mut end_flg: bool = false;
    let mut timer_minute: u32 = 0;
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        stopwatch_flg = true;
    } else if args.len() >= 2 {
        let option_part: String = args[1].clone();
        if option_part.chars().nth(0).unwrap() != '-' {
            println!("When using options, prefix the option with '-'.");
            end_flg = true;
        } else {
            if option_part.chars().nth(1).unwrap() == 't' {
                timer_flg = true;
            } else if option_part.chars().nth(1).unwrap() == 's' {
                stopwatch_flg = true;
            } else {
                println!("Only 't' or 's' can be used as an option.");
                end_flg = true;
            }
        }

        if timer_flg && args.len() == 3 {
            (timer_minute, end_flg) = match args[2].trim().parse() {
                Ok(num) => (num, false),
                Err(_) => (0, true),
            };

            if end_flg {
                println!("Please enter a number.");
            }
        }
    }
    (timer_flg, stopwatch_flg, end_flg, timer_minute)
}

fn stopwatch_disp() {
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

fn timer_disp(mut m: u32) {
    let h: u32 = m / 60;
    m = m % 60;

    let mut timedisp = Displaytime {
        second: 0,
        minute: m as i32,
        hour: h as i32,
        second_str: "00".to_string(),
        minute_str: "00".to_string(),
        hour_str: "00".to_string(),
    };
    let du = Duration::new(1, 0);
    loop {
        timedisp.shaping_str();
        timedisp.display();
        thread::sleep(du);
        if timedisp.declement_time() {
            break;
        }
    }
    println!("!!! finish !!!");
}

fn main() {
    let timer_flg: bool;
    let stopwatch_flg: bool;
    let end_flg: bool;
    let timer_minute: u32;

    (timer_flg, stopwatch_flg, end_flg, timer_minute) = option_select();

    if !end_flg {
        if timer_flg {
            timer_disp(timer_minute);
        } else if stopwatch_flg {
            stopwatch_disp();
        }
    }
}
