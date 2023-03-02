use crate::types::common::{BoxFilterOrdering, DistributionKind, Easing, LightColor, RotationBehaviour, RotationDirection, LimitKind, TransitionKind, Axis};

#[derive(Debug, PartialEq)]
pub enum BoxFilterSettings {
    Sections { count: i32, index: i32 },
    StepAndOffset { start: i32, skip: i32 },
}

#[derive(Debug, PartialEq)]
pub struct BoxFilter {
    pub chunks: i32,
    pub settings: BoxFilterSettings,
    pub reverse: bool,
    pub ordering: BoxFilterOrdering,
    pub random_seed: i32,
    pub limit: f64,
    pub limit_kind: LimitKind,
}

#[derive(Debug, PartialEq)]
pub struct LightColorEvent {
    pub relative_beat: f64,
    pub transition_kind: TransitionKind,
    pub color: LightColor,
    pub brightness: f64,
    pub frequency: i32,
}

#[derive(Debug, PartialEq)]
pub struct LightRotationEvent {
    pub relative_beat: f64,
    pub behaviour: RotationBehaviour,
    pub easing: Easing,
    pub loops: i32,
    pub amount: f64,
    pub direction: RotationDirection,
}

#[derive(Debug, PartialEq)]
pub struct LightTranslationEvent {
    pub relative_beat: f64,
    pub rotation_behaviour: RotationBehaviour,
    pub easing: Easing,
    pub amount: f64,
}

#[derive(Debug, PartialEq)]
pub enum LightEvents {
    Color(Vec<LightColorEvent>),
    Rotation(Vec<LightRotationEvent>),
    Translation(Vec<LightTranslationEvent>),
}

#[derive(Debug, PartialEq)]
pub struct LightEventLane {
    pub filter: BoxFilter,
    pub beat_dist: f64,
    pub beat_dist_kind: DistributionKind,
    pub dist: f64,
    pub dist_kind: DistributionKind,
    pub dist_affects_first_event: bool,
    pub dist_easing: Option<Easing>,
    pub axis: Option<Axis>,
    pub reverse: Option<bool>,
    pub events: LightEvents,
}

#[derive(Debug, PartialEq)]
pub struct LightEventBox {
    pub beat: f64,
    pub group: i32,
    pub lanes: Vec<LightEventLane>,
}
