use std::fs::File;
use std::io::Read;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use crate::error::Result;

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(i8)]
pub enum NoteColor {
    Red = 0,
    Blue = 1
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(i8)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    UpLeft = 4,
    UpRight = 5,
    DownLeft = 6,
    DownRight = 7,
    Any = 8
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(i8)]
pub enum SliderMidAnchorMode {
    Straight = 0,
    Clockwise = 1,
    CounterClockwise = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(i8)]
pub enum RotationBehaviour {
    Transition = 0,
    Extend = 1,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(i8)]
pub enum RotationDirection {
    Automatic = 0,
    Clockwise = 1,
    CounterClockwise = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(i8)]
pub enum DistributionKind {
    Wave = 1,
    Step = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(i8)]
pub enum Easing {
    None = -1,
    Linear = 0,
    EaseInQuad = 1,
    EaseOutQuad = 2,
    EaseInOutQuad = 3,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(i8)]
pub enum Axis {
    X = 0,
    Y = 1,
    Z = 2
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(i8)]
pub enum TransitionKind {
    Instant = 0,
    Transition = 1,
    Extend = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(i8)]
pub enum LightColor {
    Red = 0,
    Blue = 1,
    White = 2,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(i8)]
pub enum BoxFilterOrdering {
    Standard1 = 0,
    Standard2 = 1,
    Random = 2,
    RandomStartingIndex = 3,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(i8)]
pub enum LimitKind {
    Sections = 0,
    SectionsDuration = 1,
    SectionsBrightness = 2,
    SectionsDurationBrightness = 3,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(i8)]
pub enum BoxFilterKind {
    Sections = 1,
    StepAndOffset = 2
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    Expert,
    ExpertPlus
}

impl Difficulty {
    pub fn rank(&self) -> i32 {
        match self {
            Difficulty::Easy => 1,
            Difficulty::Normal => 3,
            Difficulty::Hard => 5,
            Difficulty::Expert => 7,
            Difficulty::ExpertPlus => 9
        }
    }
}

pub(crate) fn read_string_from_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}
