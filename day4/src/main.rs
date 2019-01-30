#[macro_use] extern crate text_io;

use std::collections::HashMap;
use std::default::Default;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::env::args;

type GuardId = u16;

#[derive(Debug, PartialOrd, PartialEq, Ord, Eq)]
enum LogEvent {
    BeginShift(GuardId),
    WakeUp,
    FallAsleep,
}

#[derive(PartialOrd, PartialEq, Ord, Eq)]
struct LogEntry {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    event: Option<LogEvent>,
}

impl Default for LogEntry {
    fn default() -> LogEntry {
        LogEntry {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            event: None,
        }
    }
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}-{:02}-{:02} {:02}:{:02}] {:?}",
               self.year, self.month, self.day, self.hour, self.minute,
               self.event)
    }
}

struct GuardStats {
    total_minutes_asleep: u64,
    sleep_minute_frequency: [u64; 60],
}

fn open_input_file(file_name: &str) -> File {
    File::open(&Path::new(file_name)).unwrap()
}

fn log_event_from_log_text(log_text: &str) -> Option<LogEvent> {
    match log_text {
        "wakes up" => Some(LogEvent::WakeUp),
        "falls asleep" => Some(LogEvent::FallAsleep),
        _ => Some(LogEvent::BeginShift(read!("Guard #{}", log_text.bytes()))),
    }
}

fn log_entries_from_file(file: &File) -> Vec<LogEntry> {
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    let mut log_entries = Vec::new();

    while reader.read_line(&mut buffer).unwrap_or_default() > 0 {
        let mut entry: LogEntry = Default::default();
        let log_text: String;
        scan!(buffer.bytes() => "[{}-{}-{} {}:{}] {}\n",
              entry.year, entry.month, entry.day,
              entry.hour, entry.minute, log_text);
        entry.event = log_event_from_log_text(&log_text);

        log_entries.push(entry);
        buffer.clear();
    }

    log_entries.sort_unstable();
    log_entries
}

fn append_minutes_to_guard_stats(stats: &mut GuardStats,
                                 start_minute: u8, end_minute: u8) {
    stats.total_minutes_asleep += (end_minute - start_minute) as u64;
    for minute in stats.sleep_minute_frequency[(start_minute as usize)..(end_minute as usize)].iter_mut() {
        *minute += 1;
    }
}

fn guard_stats_from_log_entries(log_entries: &Vec<LogEntry>)
    -> HashMap<GuardId, GuardStats> {
    let mut guard_stats_map: HashMap<GuardId, GuardStats>
        = HashMap::new();
    let mut guard_on_duty: GuardId = 0;
    let mut minute_fell_asleep: u8 = 0;
    for entry in log_entries {
        match entry.event.as_ref().unwrap() {
            LogEvent::BeginShift(guard_id) => { guard_on_duty = *guard_id; },
            LogEvent::WakeUp => {
                if !guard_stats_map.contains_key(&guard_on_duty) {
                    guard_stats_map.insert(guard_on_duty,
                        GuardStats { total_minutes_asleep: 0,
                                     sleep_minute_frequency: [0; 60], });
                }
                append_minutes_to_guard_stats(
                    guard_stats_map.get_mut(&guard_on_duty).unwrap(),
                    minute_fell_asleep, entry.minute);
            },
            LogEvent::FallAsleep => { minute_fell_asleep = entry.minute; },
        }
    }

    guard_stats_map
}

fn strategy_one(guard_stats: &HashMap<GuardId, GuardStats>) {
    println!("== Strategy 1 ==");
    let (max_guard_id, max_guard_stats) =
        guard_stats.iter().max_by_key(|&(_k, v)| v.total_minutes_asleep).unwrap();
    println!("Guard with maximum minutes asleep: ID #{}, {} minutes",
             max_guard_id, max_guard_stats.total_minutes_asleep);
    let (max_minute, max_minute_frequency) =
        max_guard_stats.sleep_minute_frequency
        .iter().enumerate().max_by_key(|&(_i, f)| f).unwrap();
    println!("Minute with most sleep: {}, {} times", max_minute, max_minute_frequency);

    println!("Sleepiest guard ID * sleepiest minute for that guard = {}", (*max_guard_id as usize) * max_minute);
}

fn strategy_two(guard_stats: &HashMap<GuardId, GuardStats>) {
    println!("== Strategy 2 ==");
    let (guard, min, freq) = guard_stats.iter()
        .map(|(k, v)| {
            let (min, freq) = v.sleep_minute_frequency.iter().enumerate().max_by_key(|&(_i, f)| f).unwrap();
            (k, min, freq)
        })
        .max_by_key(|&(_id, _min, freq)| freq)
        .unwrap();
    println!("Guard ID #{} spent minute {} asleep the most overall: {} times",
             guard, min, freq);
    println!("Guard ID #{} * minute {} = {}", guard, min, (*guard as usize) * min);
}

fn main() {
    let file_name = args().nth(1).unwrap();
    let log_entries = log_entries_from_file(&open_input_file(&file_name));
    println!("First five log entries:");
    log_entries.iter()
        .take(5)
        .for_each(|x| println!("{}", x));

    let guard_stats = guard_stats_from_log_entries(&log_entries);
    println!("Minutes asleep for guard #1201: {}", guard_stats.get(&1201)
             .unwrap().total_minutes_asleep);
    println!("Times asleep for guard #1201 in minute 16: {}",
             guard_stats.get(&1201).unwrap().sleep_minute_frequency[16]);

    strategy_one(&guard_stats);
    strategy_two(&guard_stats);
}
