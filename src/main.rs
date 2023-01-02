use std::env;
use std::thread;
use std::time::Duration;

const NUMBER: [[u8; 15]; 10] = [
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
const COLON: [u8; 15] = [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0];

#[derive(Debug)]
struct Displaytime {
    second: i32,
    minute: i32,
    hour: i32,
    ten_second: i32,
    ten_minute: i32,
    ten_hour: i32,
    one_second: i32,
    one_minute: i32,
    one_hour: i32,
}

impl Displaytime {
    fn display(&self) {
        draw_number(
            self.ten_hour,
            self.one_hour,
            self.ten_minute,
            self.one_minute,
            self.ten_second,
            self.one_second,
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

    fn shaping_time(&mut self) {
        // second string shaping
        if self.second <= 9 {
            self.ten_second = 0;
            self.one_second = self.second % 10;
        } else {
            self.ten_second = self.second / 10;
            self.one_second = self.second % 10;
        }
        // minute string shaping
        if self.minute <= 9 {
            self.ten_minute = 0;
            self.one_minute = self.minute % 10;
        } else {
            self.ten_minute = self.minute / 10;
            self.one_minute = self.minute % 10;
        }

        // hour string shaping
        if self.hour <= 9 {
            self.ten_hour = 0;
            self.one_hour = self.hour % 10;
        } else {
            self.ten_hour = self.hour / 10;
            self.one_hour = self.hour % 10;
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
        ten_second: 0,
        ten_minute: 0,
        ten_hour: 0,
        one_second: 0,
        one_minute: 0,
        one_hour: 0,
    };
    let du = Duration::new(1, 0);
    loop {
        timedisp.display();
        thread::sleep(du);
        timedisp.inclement_time();
        timedisp.shaping_time();
    }
}

fn timer_disp(mut m: u32) {
    let h: u32 = m / 60;
    m = m % 60;

    let mut timedisp = Displaytime {
        second: 0,
        minute: m as i32,
        hour: h as i32,
        ten_second: 0,
        ten_minute: 0,
        ten_hour: 0,
        one_second: 0,
        one_minute: 0,
        one_hour: 0,
    };
    let du = Duration::new(1, 0);
    loop {
        timedisp.shaping_time();
        timedisp.display();
        thread::sleep(du);
        if timedisp.declement_time() {
            break;
        }
    }
    println!("!!! finish !!!");
}

fn create_line(start: usize, end: usize, mut line: String, target_num: usize) -> String {
    for i in start..end {
        let s: String;
        if NUMBER[target_num][i] == 1 {
            s = format!("\x1b[42m\x1b[32m{}\x1b[0m", NUMBER[target_num][i])
        } else {
            s = format!("\x1b[40m\x1b[30m{}\x1b[0m", NUMBER[target_num][i])
        }
        line = line + &s;
    }
    line = line + " ";
    line
}

fn create_colon(start: usize, end: usize, mut line: String) -> String {
    for i in start..end {
        let s: String;
        if COLON[i] == 1 {
            s = format!("\x1b[42m\x1b[32m{}\x1b[0m", COLON[i])
        } else {
            s = format!("\x1b[40m\x1b[30m{}\x1b[0m", COLON[i])
        }
        line = line + &s;
    }
    line = line + " ";
    line
}

fn draw_number(
    ten_hour: i32,
    one_hour: i32,
    ten_minute: i32,
    one_minute: i32,
    ten_second: i32,
    one_second: i32,
) {
    let mut line = String::new();
    line = line + " ";

    // line1
    line = create_line(0, 3, line, ten_hour as usize);
    line = create_line(0, 3, line, one_hour as usize);
    line = create_colon(0, 3, line);
    line = create_line(0, 3, line, ten_minute as usize);
    line = create_line(0, 3, line, one_minute as usize);
    line = create_colon(0, 3, line);
    line = create_line(0, 3, line, ten_second as usize);
    line = create_line(0, 3, line, one_second as usize);
    line = line + "\n ";
    // line2
    line = create_line(3, 6, line, ten_hour as usize);
    line = create_line(3, 6, line, one_hour as usize);
    line = create_colon(3, 6, line);
    line = create_line(3, 6, line, ten_minute as usize);
    line = create_line(3, 6, line, one_minute as usize);
    line = create_colon(3, 6, line);
    line = create_line(3, 6, line, ten_second as usize);
    line = create_line(3, 6, line, one_second as usize);
    line = line + "\n ";
    // line3
    line = create_line(6, 9, line, ten_hour as usize);
    line = create_line(6, 9, line, one_hour as usize);
    line = create_colon(6, 9, line);
    line = create_line(6, 9, line, ten_minute as usize);
    line = create_line(6, 9, line, one_minute as usize);
    line = create_colon(6, 9, line);
    line = create_line(6, 9, line, ten_second as usize);
    line = create_line(6, 9, line, one_second as usize);
    line = line + "\n ";
    // line4
    line = create_line(9, 12, line, ten_hour as usize);
    line = create_line(9, 12, line, one_hour as usize);
    line = create_colon(9, 12, line);
    line = create_line(9, 12, line, ten_minute as usize);
    line = create_line(9, 12, line, one_minute as usize);
    line = create_colon(9, 12, line);
    line = create_line(9, 12, line, ten_second as usize);
    line = create_line(9, 12, line, one_second as usize);
    line = line + "\n ";
    // line5
    line = create_line(12, 15, line, ten_hour as usize);
    line = create_line(12, 15, line, one_hour as usize);
    line = create_colon(12, 15, line);
    line = create_line(12, 15, line, ten_minute as usize);
    line = create_line(12, 15, line, one_minute as usize);
    line = create_colon(12, 15, line);
    line = create_line(12, 15, line, ten_second as usize);
    line = create_line(12, 15, line, one_second as usize);
    line = line + "\n ";

    eprint!("{}", line);
    eprint!("\x1b[16D\x1b[5A");
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
