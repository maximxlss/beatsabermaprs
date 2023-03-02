#![allow(nonstandard_style)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use crate::error::{Error, Result};
use crate::types::common::{Axis, BoxFilterOrdering, Direction, DistributionKind, Easing, LightColor, RotationBehaviour, RotationDirection, LimitKind, NoteColor, SliderMidAnchorMode, TransitionKind, Difficulty, read_string_from_file, BoxFilterKind};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Info {
    pub _version: String,
    pub _songName: String,
    pub _songSubName: String,
    pub _songAuthorName: String,
    pub _levelAuthorName: String,
    pub _beatsPerMinute: f64,
    pub _shuffle: f64,
    pub _shufflePeriod: f64,
    pub _previewStartTime: f64,
    pub _previewDuration: f64,
    pub _songFilename: String,
    pub _coverImageFilename: String,
    pub _environmentName: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub _allDirectionsEnvironmentName : Option<String>,
    pub _songTimeOffset: f64,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub _customData: HashMap<String, serde_json::Value>,
    pub _difficultyBeatmapSets: Vec<DifficultyBeatmapSet>
}

impl Info {
    pub fn read_from_str(data: &str) -> Result<Self> {
        Ok(serde_json::from_str(data)?)
    }

    pub fn read_from_file(path: &str) -> Result<Self> {
        Self::read_from_str(&read_string_from_file(path)?)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DifficultyBeatmapSet {
    pub _beatmapCharacteristicName: String,
    pub _difficultyBeatmaps: Vec<DifficultyBeatmap>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct DifficultyBeatmap {
    pub _difficulty: Difficulty,
    pub _difficultyRank: i32,
    pub _beatmapFilename: String,
    pub _noteJumpMovementSpeed: f64,
    pub _noteJumpStartBeatOffset: f64,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub _customData: HashMap<String, serde_json::Value>,
}

#[derive(Debug, PartialEq)]
pub enum BeatmapFile {
    Old(OldBeatmapFile),
    New(NewBeatmapFile)
}

impl BeatmapFile {
    pub fn read_from_str(data: &str) -> Result<Self> {
        let new = serde_json::from_str(data);
        let old = serde_json::from_str(data);
        if let Ok(beatmap) = new {
            Ok(Self::New(beatmap))
        } else if let Ok(beatmap) = old {
            Ok(Self::Old(beatmap))
        } else {
            Err(Error::BeatmapParsingFailed { err_as_new: new.unwrap_err(), err_as_old: old.unwrap_err() })
        }
    }

    pub fn read_from_file(path: &str) -> Result<Self> {
        Self::read_from_str(&read_string_from_file(path)?)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NewBeatmapFile {
    pub version: String,
    pub bpmEvents: Vec<BpmEvent>,
    pub rotationEvents: Vec<RotationEvent>,
    pub colorNotes: Vec<ColorNote>,
    pub bombNotes: Vec<BombNote>,
    pub obstacles: Vec<Obstacle>,
    pub sliders: Vec<Slider>,
    pub burstSliders: Vec<BurstSlider>,
    pub waypoints: Vec<serde_json::Value>,
    pub basicBeatmapEvents: Vec<BasicBeatmapEvent>,
    pub colorBoostBeatmapEvents: Vec<ColorBoostBeatmapEvent>,
    pub lightColorEventBoxGroups: Vec<LightColorEventBoxGroup>,
    pub lightRotationEventBoxGroups: Vec<LightRotationEventBoxGroup>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub lightTranslationEventBoxGroups: Vec<LightTranslationEventBoxGroup>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub basicEventTypesWithKeywords: HashMap<String, serde_json::Value>,
    pub useNormalEventsAsCompatibleEvents: bool,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub customData: HashMap<String, serde_json::Value>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BpmEvent {
    pub b: f64,
    pub m: f64
}

fn bool_to_int<S>(value: &bool, serialized: S) -> core::result::Result<S::Ok, S::Error> where S: serde::Serializer {
    serialized.serialize_i32(*value as i32)
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RotationEvent {
    pub b: f64,
    #[serde(serialize_with = "bool_to_int")]
    pub e: bool,
    pub r: f64
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ColorNote {
    pub b: f64,
    pub x: i32,
    pub y: i32,
    pub c: NoteColor,
    pub d: Direction,
    pub a: i32
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BombNote {
    pub b: f64,
    pub x: i32,
    pub y: i32
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Obstacle {
    pub b: f64,
    pub x: i32,
    pub y: i32,
    pub d: f64,
    pub w: f64,
    pub h: f64
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Slider {
    pub b: f64,
    pub c: NoteColor,
    pub x: i32,
    pub y: i32,
    pub d: Direction,
    pub mu: f64,
    pub tb: f64,
    pub tx: i32,
    pub ty: i32,
    pub tc: Direction,
    pub tmu: f64,
    pub m: SliderMidAnchorMode
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BurstSlider {
    pub b: f64,
    pub c: NoteColor,
    pub x: i32,
    pub y: i32,
    pub d: Direction,
    pub tb: f64,
    pub tx: i32,
    pub ty: i32,
    pub sc: i32,
    pub s: f64
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BasicBeatmapEvent {
    pub b: f64,
    pub et: i32,
    pub i: i32,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub f: Option<f64>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ColorBoostBeatmapEvent {
    pub b: f64,
    pub o: bool
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LightColorEventBoxGroup {
    pub b: f64,
    pub g: i32,
    pub e: Vec<LightColorEventBoxGroupLane>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LightColorEventBoxGroupLane {
    pub f: FilterObject,
    pub w: f64,
    pub d: DistributionKind,
    pub r: f64,
    pub t: DistributionKind,
    #[serde(serialize_with = "bool_to_int")]
    pub b: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub i: Option<Easing>,
    pub e: Vec<LightColorEventData>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LightColorEventData {
    pub b: f64,
    pub i: TransitionKind,
    pub c: LightColor,
    pub s: f64,
    pub f: i32
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LightRotationEventBoxGroup {
    pub b: f64,
    pub g: i32,
    pub e: Vec<LightRotationEventBoxGroupLane>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LightRotationEventBoxGroupLane {
    pub f: FilterObject,
    pub w: f64,
    pub d: DistributionKind,
    pub s: f64,
    pub t: DistributionKind,
    #[serde(serialize_with = "bool_to_int")]
    pub b: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub i: Option<Easing>,
    pub a: Axis,
    #[serde(serialize_with = "bool_to_int")]
    pub r: bool,
    pub e: Vec<LightRotationEventData>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LightRotationEventData {
    pub b: f64,
    pub p: RotationBehaviour,
    pub l: i32,
    pub e: Easing,
    pub r: f64,
    pub o: RotationDirection
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LightTranslationEventBoxGroup {
    pub b: f64,
    pub g: i32,
    pub e: Vec<LightTranslationEventBoxGroupLane>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LightTranslationEventBoxGroupLane {
    pub f: FilterObject,
    pub w: f64,
    pub d: DistributionKind,
    pub s: f64,
    pub t: DistributionKind,
    #[serde(serialize_with = "bool_to_int")]
    pub b: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub i: Option<Easing>,
    pub a: Axis,
    #[serde(serialize_with = "bool_to_int")]
    pub r: bool,
    pub l: Vec<LightTranslationEventData>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct LightTranslationEventData {
    pub b: f64,
    pub p: RotationBehaviour,
    pub e: Easing,
    pub t: f64
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FilterObject {
    pub c: i32,
    pub f: BoxFilterKind,
    pub p: i32,
    pub t: i32,
    #[serde(serialize_with = "bool_to_int")]
    pub r: bool,
    pub n: BoxFilterOrdering,
    pub s: i32,
    pub l: f64,
    pub d: LimitKind
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OldBeatmapFile {
    pub _version: String,
    pub _notes: Vec<OldNote>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub _sliders: Vec<OldSlider>,
    pub _obstacles: Vec<OldObstacle>,
    pub _events: Vec<OldEvent>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub _waypoints: Vec<serde_json::Value>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub customData: HashMap<String, serde_json::Value>
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(i8)]
pub enum OldNoteKind {
    Red = 0,
    Blue = 1,
    Unused = 2,
    Bomb = 3
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OldNote {
    pub _time: f64,
    pub _lineIndex: i32,
    pub _lineLayer: i32,
    pub _type: OldNoteKind,
    pub _cutDirection: Direction,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub _customData: HashMap<String, serde_json::Value>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OldSlider {
    pub _colorType: NoteColor,
    pub _headTime: f64,
    pub _headLineIndex: i32,
    pub _headLineLayer: i32,
    pub _headControlPointLengthMultiplier: f64,
    pub _headCutDirection: Direction,
    pub _tailTime: f64,
    pub _tailLineIndex: i32,
    pub _tailLineLayer: i32,
    pub _tailControlPointLengthMultiplier: f64,
    pub _tailCutDirection: Direction,
    pub _sliderMidAnchorMode: SliderMidAnchorMode,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub _customData: HashMap<String, serde_json::Value>
}

#[derive(Serialize_repr, Deserialize_repr, Debug, PartialEq)]
#[repr(i8)]
pub enum OldObstacleKind {
    Full = 0,
    Crouch = 1
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OldObstacle {
    pub _time: f64,
    pub _lineIndex: i32,
    pub _type: OldObstacleKind,
    pub _duration: f64,
    pub _width: f64,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub _customData: HashMap<String, serde_json::Value>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OldEvent {
    pub _time: f64,
    pub _type: i32,
    pub _value: i32,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub _floatValue: Option<f64>,
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub _customData: HashMap<String, serde_json::Value>
}

