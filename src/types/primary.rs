use std::collections::HashMap;
use crate::types::common::{Difficulty, Direction, NoteColor, SliderMidAnchorMode};
use crate::types::lightning::LightEventBox;

/// Holds info, contained in `info.dat`. That's the song info and the list of difficulties. Actual beatmaps are contained in [Beatmap]
#[derive(Debug, PartialEq)]
pub struct BeatmapSetMeta {
    pub version: String,
    pub song_name: String,
    pub song_subname: String,
    pub song_author: String,
    pub map_author: String,
    pub bpm: f64,
    pub shuffle: f64,
    pub shuffle_period: f64,
    pub preview_start: f64,
    pub preview_duration: f64,
    pub song_filename: String,
    pub cover_image_filename: String,
    pub environment_name: String,
    pub all_directions_environment_name: Option<String>,
    pub song_offset: f64,
    pub custom_data: HashMap<String, serde_json::Value>,
    pub difficulty_sets: Vec<DifficultySet>,
}

#[derive(Debug, PartialEq)]
pub struct DifficultySet {
    pub game_mode: String,
    pub beatmaps: Vec<BeatmapMeta>,
}

#[derive(Debug, PartialEq)]
pub struct BeatmapMeta {
    pub difficulty: Difficulty,
    pub rank: i32,
    pub filename: String,
    pub note_jump_speed: f64,
    pub note_jump_start_beat_offset: f64,
    pub custom_data: HashMap<String, serde_json::Value>,
}

/// Holds info about a particular beatmap (one difficulty of a map)
#[derive(Debug, PartialEq)]
pub struct Beatmap {
    pub version: String,
    pub events: Vec<Event>,
    pub waypoints: Vec<serde_json::Value>,
    pub basic_event_types_with_keywords: HashMap<String, serde_json::Value>,
    pub use_normal_events_as_compatible_events: bool,
    pub custom_data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, PartialEq)]
pub struct BPMEvent {
    pub beat: f64,
    pub value: f64,
}

#[derive(Debug, PartialEq)]
pub struct Rotation {
    pub beat: f64,
    pub is_late: bool,
    pub value: f64,
}

#[derive(Debug, PartialEq)]
pub struct Note {
    pub beat: f64,
    pub x: i32,
    pub y: i32,
    pub color: NoteColor,
    pub direction: Direction,
    pub angle_offset: f64,
}

#[derive(Debug, PartialEq)]
pub struct Bomb {
    pub beat: f64,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, PartialEq)]
pub struct Obstacle {
    pub beat: f64,
    pub x: i32,
    pub y: i32,
    pub duration: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, PartialEq)]
pub struct Slider {
    pub head_beat: f64,
    pub color: NoteColor,
    pub head_x: i32,
    pub head_y: i32,
    pub head_direction: Direction,
    pub head_bulge: f64,
    pub tail_beat: f64,
    pub tail_x: i32,
    pub tail_y: i32,
    pub tail_direction: Direction,
    pub tail_bulge: f64,
    pub special_curving: SliderMidAnchorMode,
}

#[derive(Debug, PartialEq)]
pub struct BurstSlider {
    pub head_beat: f64,
    pub color: NoteColor,
    pub head_x: i32,
    pub head_y: i32,
    pub head_direction: Direction,
    pub tail_beat: f64,
    pub tail_x: i32,
    pub tail_y: i32,
    pub segment_count: i32,
    pub squish: f64,
}

#[derive(Debug, PartialEq)]
pub struct BasicEvent {
    pub beat: f64,
    pub kind: i32,
    pub value: i32,
    pub float_value: Option<f64>,
    pub custom_data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, PartialEq)]
pub struct ColorBoost {
    pub beat: f64,
    pub enable: bool,
}

#[derive(Debug, PartialEq)]
pub enum Event {
    BPM(Box<BPMEvent>),
    Rotation(Box<Rotation>),
    Note(Box<Note>),
    Bomb(Box<Bomb>),
    Obstacle(Box<Obstacle>),
    Slider(Box<Slider>),
    BurstSlider(Box<BurstSlider>),
    BasicEvent(Box<BasicEvent>),
    ColorBoost(Box<ColorBoost>),
    LightEventBox(Box<LightEventBox>),
}
