extern crate chrono;
extern crate pancurses;

use chrono::Local;
use crate::chrono::Timelike;
use pancurses::*;
use std::time::Duration;
use std::thread::sleep;

const SECONDS_IN_MINUTE: u32 = 60;
const SECONDS_IN_HOUR: u32 = 3600;

fn main() {
    let window = initscr();
    let max_x: i32 = window.get_max_x();
    let time_progress_window = newwin(2, max_x - 3, 2, 1);

    if has_colors() {
        start_color();
        init_pair(0, COLOR_BLUE, COLOR_BLUE);
        init_pair(1, COLOR_WHITE, COLOR_BLUE);
        init_pair(2, COLOR_BLUE, COLOR_BLACK);
        init_pair(3, COLOR_WHITE, COLOR_BLACK);
    }

    // https://en.wikipedia.org/wiki/Geometric_Shapes
    // https://en.wikipedia.org/wiki/Box_Drawing_(Unicode_block)
    loop {
        let date = Local::now();
        window.color_set(1);

        let top_border = date.format("%H:%M:%S - %A %B %d, %Y").to_string();
        let horizontal_border = "━".repeat((max_x -2) as usize);
        let border = format!("{: ^1$}", top_border.to_string(), max_x as usize);


        window.printw(&border);
        window.mv(1, 0);
        window.color_set(2);

        window.printw("┏");
        window.printw(&horizontal_border);
        window.printw("┓");

        window.printw("┃");
        window.printw(" ".repeat((max_x -2) as usize));
        window.printw("┃");

        window.printw("┃");
        window.printw(" ".repeat((max_x -2) as usize));
        window.printw("┃");

        window.printw("┗");
        window.printw("━".repeat((max_x -2) as usize));
        window.printw("┛");

        time_progress_window.color_set(3);

        time_progress_window.mv(0, 0);
        time_progress_window.printw(" M ");
        let progress_width = time_progress_window.get_max_x() - 15;
        let milli_seconds = (date.timestamp_subsec_millis() as f64/ 1000.0) as f64;
        let seconds = date.second() as f64 + milli_seconds;

        let minute_progress_percentage_complete : f64 = seconds / SECONDS_IN_MINUTE as f64 * 100.00;
        for n in 1..progress_width {
            if (n as f64 / progress_width as f64 * 100.0) < minute_progress_percentage_complete as f64 {
                time_progress_window.printw("█");
            } else {
                time_progress_window.printw("░");
            }
        }
        let formatted_number = format!("{:.*}", 2, minute_progress_percentage_complete);
        time_progress_window.printw(" ");
        time_progress_window.printw(formatted_number.to_string());


        time_progress_window.mv(1, 0);
        time_progress_window.printw(" H ");
        let progress_width = time_progress_window.get_max_x() - 15;
        let seconds = seconds;
        let minutes = (date.minute() as f64 * 60.0) + seconds;

        let hour_progress_percentage_complete = minutes / SECONDS_IN_HOUR as f64 * 100.00;
        for n in 1..progress_width {
            if (n as f64 / progress_width as f64 * 100.0) < hour_progress_percentage_complete as f64 {
                time_progress_window.printw("█");
            } else {
                time_progress_window.printw("░");
            }
        }
        let formatted_number = format!("{:.*}", 2, hour_progress_percentage_complete);
        time_progress_window.printw(" ");
        time_progress_window.printw(formatted_number.to_string());

        window.refresh();
        window.clear();
        time_progress_window.refresh();
        time_progress_window.clear();

        sleep(Duration::from_millis(50));
   }
  endwin();
}
