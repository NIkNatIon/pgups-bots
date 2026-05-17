use crate::i18n::I18n;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Lesson {
    #[serde(rename = "pairNum")]
    pub pair_num: i32,
    pub discipline: String,
    #[serde(rename = "typeOfDiscipline")]
    pub type_of_discipline: String,
    pub teacher: Option<Teacher>,
    pub weekday: String,
    pub parity: String,
    pub room: Option<Room>,
}

#[derive(Debug, Deserialize)]
pub struct Teacher {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Room {
    pub name: String,
}

const PAIR_TIMES: [&str; 7] = [
    "9:00 - 10:30",
    "10:45 - 12:15",
    "13:15 - 14:45",
    "15:00 - 16:30",
    "16:45 - 18:15",
    "18:25 - 19:55",
    "20:05 - 21:35",
];

pub fn format_lessons(lessons: &[Lesson], weekday: &str, parity: &str, i18n: &I18n) -> String {
    let weekday_key = format!("schedule-weekday-{}", weekday.to_lowercase());
    let parity_key = format!("schedule-parity-{}", parity.to_lowercase());
    let weekday_name = i18n.get(&weekday_key);
    let parity_name = i18n.get(&parity_key);

    if lessons.is_empty() {
        return format!(
            "{} ({})\n{}\n",
            weekday_name,
            parity_name,
            i18n.get("schedule-no-lessons")
        );
    }

    let mut sorted: Vec<&Lesson> = lessons.iter().collect();
    sorted.sort_by_key(|l| l.pair_num);
    sorted.dedup_by_key(|l| l.pair_num);

    let mut result = format!("{}\n", weekday_name.to_uppercase());
    result.push_str(&format!("({})\n", parity_name));

    let pair_word = i18n.get("schedule-pair");
    let room_word = i18n.get("schedule-room");

    for lesson in sorted {
        let teacher = lesson
            .teacher
            .as_ref()
            .map(|t| t.name.as_str())
            .unwrap_or("");
        let room = lesson.room.as_ref().map(|r| r.name.as_str()).unwrap_or("");
        let time = PAIR_TIMES
            .get((lesson.pair_num - 1) as usize)
            .unwrap_or(&"");

        result.push_str("———————————————\n");
        result.push_str(&format!("{}\n", time));
        result.push_str(&format!("{} {}\n", lesson.pair_num, pair_word));
        result.push_str(&format!("{}\n", lesson.discipline));
        result.push_str(&format!("{}\n", lesson.type_of_discipline));
        if !room.is_empty() {
            result.push_str(&format!("{}: {}\n", room_word, room));
        }
        if !teacher.is_empty() {
            result.push_str(&format!("{}\n", teacher));
        }
    }
    result.push_str("———————————————\n");

    result
}

pub fn current_weekday() -> &'static str {
    let days = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days_since_epoch = now_secs / 86400;
    let day_index = ((days_since_epoch + 3) % 7) as usize;
    days[day_index]
}

pub fn tomorrow_weekday() -> &'static str {
    let days = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days_since_epoch = now_secs / 86400;
    let day_index = ((days_since_epoch + 3 + 1) % 7) as usize;
    days[day_index]
}

pub fn current_parity() -> &'static str {
    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days_since_epoch = now_secs / 86400;
    let week_number = days_since_epoch / 7;
    if week_number.is_multiple_of(2) {
        "Even"
    } else {
        "Odd"
    }
}

pub fn next_week_parity() -> &'static str {
    if current_parity() == "Even" {
        "Odd"
    } else {
        "Even"
    }
}

pub const WEEKDAYS: [&str; 6] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

pub fn urlencode(s: &str) -> String {
    let mut result = String::new();
    for byte in s.as_bytes() {
        match *byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(*byte as char);
            }
            _ => {
                result.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    result
}
