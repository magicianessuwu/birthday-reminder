use std::fs::File;
use std::io::{self, BufRead, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

struct CalendarEntry {
    year: Option<u32>,
    month: Option<u32>,
    day: Option<u32>,
    name: String,
    color: String,
    show_early: i64,
}

impl CalendarEntry {
    fn from_string(line: &str) -> Option<Self> {
        let mut year = None;
        let mut month = None;
        let mut day = None;
        let mut name = String::new();
        let mut color = String::new();
        let mut show_early = -1;

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
                "showEarly" => show_early = parts[1].parse().unwrap_or(-1),
                _ => return None,
            }
        }

        Some(CalendarEntry {
            year,
            month,
            day,
            name,
            color,
            show_early,
        })
    }

    fn should_show_entry(&self) -> bool {
        let now = chrono::Local::now().naive_local();
        let current_date = now.date();

        let entry_date = match (self.year, self.month, self.day) {
            (Some(year), Some(month), Some(day)) => {
                chrono::NaiveDate::from_ymd_opt(year as i32, month, day)
            }
            _ => Some(current_date), // If any of year, month, or day is missing, consider today's date
        };

        if let Some(entry_date) = entry_date {
            // Check if the entry date is within the show_before days
            let show_before = chrono::Duration::days(self.show_early);
            let start_date = entry_date - show_before;
            let end_date = entry_date;

            // Check if the current date falls within the range of [start_date, end_date]
            return current_date >= start_date && current_date <= end_date;
        }

        false // Return false if the provided date components are invalid
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
        entries.iter().filter(|e| e.should_show_entry()).collect();

    if !today_entries.is_empty() {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Black)))?;
        writeln!(&mut stdout, "This is happening in the near future:")?;

        for entry in today_entries {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            if let Some(color) = CalendarEntry::get_color(&entry.color) {
                stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
            }
            
            let date_string = match (entry.year, entry.month, entry.day) {
                (Some(year), Some(month), Some(day)) => format!("{}/{}/{}", month, day, year),
                (Some(year), Some(month), None) => format!("{}/Everyday!/{}", month, year),
                (Some(year), None, Some(day)) => format!("Every month!/{}/{}", day, year),
                (Some(year), None, None) => format!("Every month!/Everyday!/{}", year),
                (None, Some(month), Some(day)) => format!("{}/{}/Every Year!", month, day),
                (None, Some(month), None) => format!("{}/Everyday!/Every Year!", month),
                (None, None, Some(day)) => format!("Every month!/{}/Every Year!", day),
                (None, None, None) => format!("Everyday!"),
            };
            
            writeln!(&mut stdout, "{} - Date: {}", entry.name, date_string)?;
        }
        
    } else {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Black)))?;
        writeln!(
            &mut stdout,
            "It seems like there is nothing happening in the near future!"
        )?;
    }

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Black)))?;
    writeln!(&mut stdout, "Press ENTER to quit!")?;
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    Ok(())
}
