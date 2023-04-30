use std::collections::HashMap;
use crate::types::primary::{BasicEvent, BeatmapMeta, Bomb, BPMEvent, BurstSlider, ColorBoost, DifficultySet, Event, BeatmapSetMeta, Note, Obstacle, Rotation, Slider};
use crate::types::{primary, schema};
use crate::Beatmap;
use crate::types::common::{BoxFilterKind, NoteColor, read_string_from_file};
use crate::error::Result;
use crate::types::lightning::{BoxFilter, BoxFilterSettings, LightColorEvent, LightEventBox, LightEventLane, LightEvents, LightRotationEvent, LightTranslationEvent};

impl BeatmapSetMeta {
    pub fn read_from_str(data: &str) -> Result<Self> {
        Ok(schema::Info::read_from_str(data)?.into())
    }

    pub fn read_from_file(path: &str) -> Result<Self> {
        Self::read_from_str(&read_string_from_file(path)?)
    }
}

impl From<schema::Info> for BeatmapSetMeta {
    fn from(info: schema::Info) -> Self {
        Self {
            version: info._version,
            song_name: info._songName,
            song_subname: info._songSubName,
            song_author: info._songAuthorName,
            map_author: info._levelAuthorName,
            bpm: info._beatsPerMinute,
            shuffle: info._shuffle,
            shuffle_period: info._shufflePeriod,
            preview_start: info._previewStartTime,
            preview_duration: info._previewDuration,
            song_filename: info._songFilename,
            cover_image_filename: info._coverImageFilename,
            environment_name: info._environmentName,
            all_directions_environment_name: info._allDirectionsEnvironmentName,
            song_offset: info._songTimeOffset,
            custom_data: info._customData,
            difficulty_sets: info._difficultyBeatmapSets.into_iter().map(DifficultySet::from).collect()
        }
    }
}

impl From<schema::DifficultyBeatmapSet> for DifficultySet {
    fn from(set: schema::DifficultyBeatmapSet) -> Self {
        Self {
            game_mode: set._beatmapCharacteristicName,
            beatmaps: set._difficultyBeatmaps.into_iter().map(BeatmapMeta::from).collect()
        }
    }
}

impl From<schema::DifficultyBeatmap> for BeatmapMeta {
    fn from(beatmap: schema::DifficultyBeatmap) -> Self {
        Self {
            difficulty: beatmap._difficulty,
            rank: beatmap._difficultyRank,
            filename: beatmap._beatmapFilename,
            note_jump_speed: beatmap._noteJumpMovementSpeed,
            note_jump_start_beat_offset: beatmap._noteJumpStartBeatOffset,
            custom_data: beatmap._customData
        }
    }
}

impl Into<Option<Event>> for schema::OldNote {
    fn into(self) -> Option<Event> {
        use schema::OldNoteKind::*;
        match self._type {
            Red | Blue => Some(Event::Note(Box::new(Note {
                beat: self._time,
                x: self._lineIndex,
                y: self._lineLayer,
                color: if self._type == Red { NoteColor::Red } else { NoteColor::Blue },
                direction: self._cutDirection,
                angle_offset: 0.0,
            }))),
            Bomb => Some(Event::Bomb(Box::new(primary::Bomb {
                beat: self._time,
                x: self._lineIndex,
                y: self._lineLayer,
            }))),
            Unused => None
        }
    }
}

impl Into<Event> for schema::OldSlider {
    fn into(self) -> Event {
        Event::Slider(Box::new(Slider {
            head_beat: self._headTime,
            color: self._colorType,
            head_x: self._headLineIndex,
            head_y: self._headLineLayer,
            head_direction: self._headCutDirection,
            head_bulge: self._headControlPointLengthMultiplier,
            tail_beat: self._tailTime,
            tail_x: self._tailLineIndex,
            tail_y: self._tailLineLayer,
            tail_direction: self._tailCutDirection,
            tail_bulge: self._tailControlPointLengthMultiplier,
            special_curving: self._sliderMidAnchorMode,
        }))
    }
}

impl Into<Event> for schema::OldObstacle {
    fn into(self) -> Event {
        Event::Obstacle(Box::new(Obstacle {
            beat: self._time,
            x: self._lineIndex,
            y: if self._type == schema::OldObstacleKind::Full { 0 } else { 2 },
            duration: self._duration,
            width: self._width as f64,
            height: if self._type == schema::OldObstacleKind::Full { 5.0 } else { 2.0 },
        }))
    }
}

impl Into<Event> for schema::OldEvent {
    fn into(self) -> Event {
        Event::BasicEvent(Box::new(BasicEvent {
            beat: self._time,
            kind: self._type,
            value: self._value,
            float_value: self._floatValue,
            custom_data: self._customData,
        }))
    }
}

impl Into<Event> for schema::BpmEvent {
    fn into(self) -> Event {
        Event::BPM(Box::new(BPMEvent {
            beat: self.b,
            value: self.m,
        }))
    }
}

impl Into<Event> for schema::RotationEvent {
    fn into(self) -> Event {
        Event::Rotation(Box::new(Rotation {
            beat: self.b,
            is_late: self.e,
            value: self.r,
        }))
    }
}

impl Into<Event> for schema::ColorNote {
    fn into(self) -> Event {
        Event::Note(Box::new(Note {
            beat: self.b,
            x: self.x,
            y: self.y,
            color: self.c,
            direction: self.d,
            angle_offset: self.a as f64,
        }))
    }
}

impl Into<Event> for schema::BombNote {
    fn into(self) -> Event {
        Event::Bomb(Box::new(Bomb {
            beat: self.b,
            x: self.x,
            y: self.y,
        }))
    }
}

impl Into<Event> for schema::Obstacle {
    fn into(self) -> Event {
        Event::Obstacle(Box::new(Obstacle {
            beat: self.b,
            x: self.x,
            y: self.y,
            duration: self.d,
            width: self.w,
            height: self.h,
        }))
    }
}

impl Into<Event> for schema::Slider {
    fn into(self) -> Event {
        Event::Slider(Box::new(Slider {
            head_beat: self.b,
            color: self.c,
            head_x: self.x,
            head_y: self.y,
            head_direction: self.d,
            head_bulge: self.mu,
            tail_beat: self.tb,
            tail_x: self.tx,
            tail_y: self.ty,
            tail_direction: self.tc,
            tail_bulge: self.tmu,
            special_curving: self.m,
        }))
    }
}

impl Into<Event> for schema::BurstSlider {
    fn into(self) -> Event {
        Event::BurstSlider(Box::new(BurstSlider {
            head_beat: self.b,
            color: self.c,
            head_x: self.x,
            head_y: self.y,
            head_direction: self.d,
            tail_beat: self.tb,
            tail_x: self.tx,
            tail_y: self.ty,
            segment_count: self.sc,
            squish: self.s,
        }))
    }
}

impl Into<Event> for schema::BasicBeatmapEvent {
    fn into(self) -> Event {
        Event::BasicEvent(Box::new(BasicEvent {
            beat: self.b,
            kind: self.et,
            value: self.i,
            float_value: self.f,
            custom_data: Default::default(),
        }))
    }
}

impl Into<Event> for schema::ColorBoostBeatmapEvent {
    fn into(self) -> Event {
        Event::ColorBoost(Box::new(ColorBoost {
            beat: self.b,
            enable: self.o,
        }))
    }
}

impl Into<Event> for schema::LightColorEventBoxGroup {
    fn into(self) -> Event {
        Event::LightEventBox(Box::new(LightEventBox {
            beat: self.b,
            group: self.g,
            lanes: self.e.into_iter().map(|x| Into::<LightEventLane>::into(x)).collect(),
        }))
    }
}

impl Into<Event> for schema::LightRotationEventBoxGroup {
    fn into(self) -> Event {
        Event::LightEventBox(Box::new(LightEventBox {
            beat: self.b,
            group: self.g,
            lanes: self.e.into_iter().map(|x| Into::<LightEventLane>::into(x)).collect(),
        }))
    }
}

impl Into<Event> for schema::LightTranslationEventBoxGroup {
    fn into(self) -> Event {
        Event::LightEventBox(Box::new(LightEventBox {
            beat: self.b,
            group: self.g,
            lanes: self.e.into_iter().map(|x| Into::<LightEventLane>::into(x)).collect(),
        }))
    }
}

impl Into<LightEventLane> for schema::LightColorEventBoxGroupLane {
    fn into(self) -> LightEventLane {
        LightEventLane {
            filter: self.f.into(),
            beat_dist: self.w,
            beat_dist_kind: self.d,
            dist: self.r,
            dist_kind: self.t,
            dist_affects_first_event: self.b,
            dist_easing: self.i,
            axis: None,
            reverse: None,
            events: LightEvents::Color(self.e.into_iter().map(|x| Into::<LightColorEvent>::into(x)).collect())
        }
    }
}

impl Into<LightEventLane> for schema::LightRotationEventBoxGroupLane {
    fn into(self) -> LightEventLane {
        LightEventLane {
            filter: self.f.into(),
            beat_dist: self.w,
            beat_dist_kind: self.d,
            dist: self.s,
            dist_kind: self.t,
            dist_affects_first_event: self.b,
            dist_easing: self.i,
            axis: Some(self.a),
            reverse: Some(self.r),
            events: LightEvents::Rotation(self.e.into_iter().map(|x| Into::<LightRotationEvent>::into(x)).collect())
        }
    }
}

impl Into<LightEventLane> for schema::LightTranslationEventBoxGroupLane {
    fn into(self) -> LightEventLane {
        LightEventLane {
            filter: self.f.into(),
            beat_dist: self.w,
            beat_dist_kind: self.d,
            dist: self.s,
            dist_kind: self.t,
            dist_affects_first_event: self.b,
            dist_easing: self.i,
            axis: Some(self.a),
            reverse: Some(self.r),
            events: LightEvents::Translation(self.l.into_iter().map(|x| Into::<LightTranslationEvent>::into(x)).collect())
        }
    }
}

impl Into<LightColorEvent> for schema::LightColorEventData {
    fn into(self) -> LightColorEvent {
        LightColorEvent {
            relative_beat: self.b,
            transition_kind: self.i,
            color: self.c,
            brightness: self.s,
            frequency: self.f,
        }
    }
}

impl Into<LightRotationEvent> for schema::LightRotationEventData {
    fn into(self) -> LightRotationEvent {
        LightRotationEvent {
            relative_beat: self.b,
            behaviour: self.p,
            easing: self.e,
            loops: self.l,
            amount: self.r,
            direction: self.o,
        }
    }
}

impl Into<LightTranslationEvent> for schema::LightTranslationEventData {
    fn into(self) -> LightTranslationEvent {
        LightTranslationEvent {
            relative_beat: self.b,
            rotation_behaviour: self.p,
            easing: self.e,
            amount: self.t,
        }
    }
}

impl Into<BoxFilter> for schema::FilterObject {
    fn into(self) -> BoxFilter {
        BoxFilter {
            chunks: self.c,
            settings: match self.f {
                BoxFilterKind::Sections =>
                    BoxFilterSettings::Sections {
                        count: self.p,
                        index: self.t,
                    },
                BoxFilterKind::StepAndOffset =>
                    BoxFilterSettings::StepAndOffset {
                        start: self.p,
                        skip: self.t,
                    }
            },
            reverse: self.r,
            ordering: self.n,
            random_seed: self.s,
            limit: self.l,
            limit_kind: self.d,
        }
    }
}


impl Beatmap {
    pub fn read_from_str(data: &str) -> Result<Self> {
        Ok(schema::BeatmapFile::read_from_str(data)?.into())
    }

    pub fn read_from_file(path: &str) -> Result<Self> {
        Self::read_from_str(&read_string_from_file(path)?)
    }
}

impl From<schema::BeatmapFile> for Beatmap {
    fn from(file: schema::BeatmapFile) -> Self {
        match file {
            schema::BeatmapFile::Old(file) => {
                let mut events: Vec<Event> = vec![];
                events.extend(file._notes.into_iter().filter_map(|e| Into::<Option<Event>>::into(e)));
                events.extend(file._sliders.into_iter().map(|x| Into::<Event>::into(x)));
                events.extend(file._obstacles.into_iter().map(|x| Into::<Event>::into(x)));
                events.extend(file._events.into_iter().map(|x| Into::<Event>::into(x)));

                Self {
                    version: file._version,
                    events,
                    waypoints: file._waypoints,
                    basic_event_types_with_keywords: HashMap::new(),
                    use_normal_events_as_compatible_events: true,
                    custom_data: file.customData,
                }
            },
            schema::BeatmapFile::New(file) => {
                let mut events: Vec<Event> = vec![];
                events.extend(file.bpmEvents.into_iter().map(|x| Into::<Event>::into(x)));
                events.extend(file.rotationEvents.into_iter().map(|x| Into::<Event>::into(x)));
                events.extend(file.colorNotes.into_iter().map(|x| Into::<Event>::into(x)));
                events.extend(file.bombNotes.into_iter().map(|x| Into::<Event>::into(x)));
                events.extend(file.sliders.into_iter().map(|x| Into::<Event>::into(x)));
                events.extend(file.obstacles.into_iter().map(|x| Into::<Event>::into(x)));
                events.extend(file.burstSliders.into_iter().map(|x| Into::<Event>::into(x)));
                events.extend(file.basicBeatmapEvents.into_iter().map(|x| Into::<Event>::into(x)));
                events.extend(file.colorBoostBeatmapEvents.into_iter().map(|x| Into::<Event>::into(x)));
                events.extend(file.lightColorEventBoxGroups.into_iter().map(|x| Into::<Event>::into(x)));
                events.extend(file.lightRotationEventBoxGroups.into_iter().map(|x| Into::<Event>::into(x)));
                events.extend(file.lightTranslationEventBoxGroups.into_iter().map(|x| Into::<Event>::into(x)));
                Self {
                    version: file.version,
                    events,
                    waypoints: file.waypoints,
                    basic_event_types_with_keywords: file.basicEventTypesWithKeywords,
                    use_normal_events_as_compatible_events: file.useNormalEventsAsCompatibleEvents,
                    custom_data: file.customData,
                }
            }
        }

    }
}
