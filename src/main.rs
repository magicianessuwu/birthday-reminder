use chrono::{Local, Datelike, Duration};
use std::fs::File;
use std::io::{self, BufRead};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

struct CalendarEntry {
    year: Option<u32>,
    month: Option<u32>,
    day: Option<u32>,
    name: String,
    color: String,
    show_before: i64,
}

impl CalendarEntry {
    fn from_string(line: &str) -> Option<Self> {
        let mut year = None;
        let mut month = None;
        let mut day = None;
        let mut name = String::new();
        let mut color = String::new();
        let mut show_before = 0;

        for field in line.split(';') {
            let parts: Vec<&str> = field.split(':').collect();
            if parts.len() != 2 {
                return None;
            }
            match parts[0] {
                "Year" => year = parts[1].parse().ok(),
                "Month" => month = parts[1].parse().ok(),
                "Day" => day = parts[1].parse().ok(),
                "Name" => name = parts[1].trim_matches('"').to_string(),
                "Color" => color = parts[1].trim_matches('"').to_string(),
                "ShowBefore" => show_before = parts[1].parse().unwrap_or(0),
                _ => return None,
            }
        }

        Some(CalendarEntry {
            year,
            month,
            day,
            name,
            color,
            show_before,
        })
    }

    fn is_happening_today(&self) -> bool {
        let now = Local::today().naive_local();
        let show_date = now.checked_sub_signed(Duration::days(self.show_before)).unwrap_or(now);
        match (self.year, self.month, self.day) {
            (Some(year), Some(month), Some(day)) => {
                if year as i32 > show_date.year() {
                    return false;
                } else if year as i32 == show_date.year() && month > &show_date.month() {
                    return false;
                } else if year as i32 == show_date.year() && month == &show_date.month() && day > &show_date.day() {
                    return false;
                }
                true
            }
            _ => false,
        }
    }

    fn get_color(color: &str) -> Option<Color> {
        match color {
            "red" => Some(Color::Red),
            "green" => Some(Color::Green),
            "blue" => Some(Color::Blue),
            "yellow" => Some(Color::Yellow),
            "pink" => Some(Color::Magenta),
            "purple" => Some(Color::Ansi256(129)),
            "orange" => Some(Color::Ansi256(202)),
            _ => None,
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("./assets/calendar.txt").expect("You forgot to include the file ./assets/calendar.txt\nHow do I use that? -> https://github.com/magicianessuwu/birthday-reminder");
    let reader = io::BufReader::new(file);

    let entries: Vec<CalendarEntry> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| CalendarEntry::from_string(&line))
        .collect();
    let today_entries: Vec<&CalendarEntry> =
        entries.iter().filter(|e| e.is_happening_today()).collect();

    if !today_entries.is_empty() {
        println!("Events happening today:");
        for entry in today_entries {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            if let Some(color) = CalendarEntry::get_color(&entry.color) {
                stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
            }
            writeln!(&mut stdout, "{}", entry.name)?;
        }
    } else {
        println!("No events are happening today.");
    }

    println!("Press ENTER key to close!");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    Ok(())
}
