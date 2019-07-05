mod jni_c_header;

use std::{
    cell::{RefCell},
    rc::Rc,
    time::SystemTime,
};

use log::{error};

use gilrs::*;
use gilrs::ff::*;
use gilrs::ev::*;
use gilrs::ev::state::*;

pub struct PowerInfoWrapper {
    pub status: String,
    pub value: u8
}

impl PowerInfoWrapper {
    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn from(incomming: PowerInfo) -> PowerInfoWrapper {
        match incomming {
            PowerInfo::Unknown => PowerInfoWrapper {
                status: "unknown".into(),
                value: 0
            },
            PowerInfo::Wired => PowerInfoWrapper {
                status: "wired".into(),
                value: 0
            },
            PowerInfo::Discharging(value) => PowerInfoWrapper {
                status: "discharging".into(),
                value
            },
            PowerInfo::Charging(value) => PowerInfoWrapper {
                status: "charging".into(),
                value
            },
            PowerInfo::Charged => PowerInfoWrapper {
                status: "charged".into(),
                value: 0
            },
        }
    }
}

pub struct EventWrapper {
    pub message: String,
    pub gamepad: GamepadId,
    pub button: Button,
    pub axis: Axis,
    pub code: Option<Code>,
    pub value: Option<f64>,
    pub time: SystemTime
} 

impl EventWrapper {
    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn gamepad(&self) -> GamepadId {
        self.gamepad
    }

    pub fn button(&self) -> Button {
        self.button
    }

    pub fn axis(&self) -> Axis {
        self.axis
    }

    pub fn code(&self) -> Option<Code> {
        self.code
    }

    pub fn value(&self) -> Option<f64> {
        self.value
    }

    pub fn time(&self) -> SystemTime {
        self.time
    }

    pub fn from(incoming: Event) -> EventWrapper {
        match incoming.event {
            EventType::ButtonPressed(button, code) => EventWrapper {
                message: "button-pressed".into(),
                gamepad: incoming.id,
                button: button,
                axis: Axis::Unknown,
                code: Some(code),
                value: None,
                time: incoming.time,
            },
            EventType::ButtonRepeated(button, code) => EventWrapper {
                message: "button-repeated".into(),
                gamepad: incoming.id,
                button: button,
                axis: Axis::Unknown,
                code: Some(code),
                value: None,
                time: incoming.time,
            },
            EventType::ButtonReleased(button, code) => EventWrapper {
                message: "button-released".into(),
                gamepad: incoming.id,
                button: button,
                axis: Axis::Unknown,
                code: Some(code),
                value: None,
                time: incoming.time,
            },
            EventType::ButtonChanged(button, value, code) => EventWrapper {
                message: "button-changed".into(),
                gamepad: incoming.id,
                button: button,
                axis: Axis::Unknown,
                code: Some(code),
                value: Some(value as f64),
                time: incoming.time,
            },
            EventType::AxisChanged(axis, value, code) => EventWrapper {
                message: "axis-changed".into(),
                gamepad: incoming.id,
                button: Button::Unknown,
                axis: axis,
                code: Some(code),
                value: Some(value as f64),
                time: incoming.time,
            },
            EventType::Connected => EventWrapper {
                message: "connected".into(),
                gamepad: incoming.id,
                button: Button::Unknown,
                axis: Axis::Unknown,
                code: None,
                value: None,
                time: incoming.time,
            },
            EventType::Disconnected => EventWrapper {
                message: "disconnected".into(),
                gamepad: incoming.id,
                button: Button::Unknown,
                axis: Axis::Unknown,
                code: None,
                value: None,
                time: incoming.time,
            },            
            EventType::Dropped => EventWrapper {
                message: "dropped".into(),
                gamepad: incoming.id,
                button: Button::Unknown,
                axis: Axis::Unknown,
                code: None,
                value: None,
                time: incoming.time,
            }
        }
    }
}

pub struct DistanceModelWrapper {
    name: String,
    ref_distance: f32,
    rolloff_factor: f32,
    max_distance: f32
}

impl DistanceModelWrapper {
    pub fn to(model: DistanceModelWrapper) -> DistanceModel {
        match model.name.as_ref() {
            "Linear" => DistanceModel::Linear{
                ref_distance: model.ref_distance, 
                rolloff_factor: model.rolloff_factor, 
                max_distance: model.max_distance
            },
            "LinearClamped" => DistanceModel::LinearClamped{
                ref_distance: model.ref_distance, 
                rolloff_factor: model.rolloff_factor,
                max_distance: model.max_distance
            },
            "Inverse" => DistanceModel::Inverse{
                ref_distance: model.ref_distance,
                rolloff_factor: model.rolloff_factor
            },
            "InverseClamped" => DistanceModel::InverseClamped{
                ref_distance: model.ref_distance,
                rolloff_factor: model.rolloff_factor,
                max_distance: model.max_distance
            },
            "Exponential" => DistanceModel::Exponential{
                ref_distance: model.ref_distance, 
                rolloff_factor: model.rolloff_factor
            },
            "ExponentialClamped" => DistanceModel::ExponentialClamped{
                ref_distance: model.ref_distance,
                rolloff_factor: model.rolloff_factor,
                max_distance: model.max_distance
            },
            _ => DistanceModel::None
        }
    }
}

pub enum BaseEffectTypeWrapper {
    Weak,
    Strong
}

pub fn base_effect(kind: BaseEffectTypeWrapper, magnitude: u16, scheduling: Replay, envelope: Envelope) -> BaseEffect {
    match kind {
        BaseEffectTypeWrapper::Weak => BaseEffect {
            kind: BaseEffectType::Weak{magnitude},
            scheduling,
            envelope
        },
        BaseEffectTypeWrapper::Strong => BaseEffect {
            kind: BaseEffectType::Strong{magnitude},
            scheduling,
            envelope
        }
    }
}

pub fn base_effect_kind(effect: &BaseEffect) -> BaseEffectTypeWrapper {
    match effect.kind {
        BaseEffectType::Weak{magnitude: _} => BaseEffectTypeWrapper::Weak,
        BaseEffectType::Strong{magnitude: _} => BaseEffectTypeWrapper::Strong,
        _ => BaseEffectTypeWrapper::Strong
    }
}

pub fn base_effect_scheduling(effect: &BaseEffect) -> Replay {
    effect.scheduling
}

pub fn base_effect_envelope(effect: &BaseEffect) -> Envelope {
    effect.envelope
}

#[derive(Debug)]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for Error {}

pub fn ticks(ms: u32) -> Ticks {
    Ticks::from_ms(ms)
}

pub fn replay(after: u32, play_for: u32, with_delay: u32) -> Replay {
    Replay {
        after: ticks(after), 
        play_for: ticks(play_for), 
        with_delay: ticks(with_delay)
    }
}

pub fn replay_after(replay: &Replay) -> Ticks {
    replay.after
}

pub fn replay_play_for(replay: &Replay) -> Ticks {
    replay.play_for
}

pub fn replay_with_delay(replay: &Replay) -> Ticks {
    replay.with_delay
}

pub fn envelope(attack_length: u32, attack_level: f32, fade_length: u32, fade_level: f32) -> Envelope {
    Envelope {
        attack_length: ticks(attack_length),
        attack_level,
        fade_length: ticks(fade_length),
        fade_level
    }
}

pub fn next_event(gilrs: &mut Gilrs) -> Option<EventWrapper> {
    gilrs.next_event().map(EventWrapper::from)
}

pub fn new_gilrs() -> Result<Gilrs, String> {
    Gilrs::new().map_err(|err| err.to_string())
}

pub fn gilrs_gamepads(gilrs: &Gilrs) -> Vec<Gamepad<'_>> {
    gilrs.gamepads().map(|(_id, gamepad)| gamepad).collect()
}

pub fn gamepad_state_button_data(state: &GamepadState, code: Code) -> Option<ButtonData> {
    state.button_data(code).map(|x| *x)
}

pub fn gamepad_state_axis_data(state: &GamepadState, code: Code) -> Option<AxisData> {
    state.axis_data(code).map(|x| *x)
}

pub fn gamepad_state(gamepad: &Gamepad) -> GamepadState {
    gamepad.state().clone() // Prob not the most idiomatic
}

pub fn gamepad_map_name(gamepad: &Gamepad) -> String {
    gamepad.map_name().unwrap_or_default().into()
}

pub fn gamepad_button_data(gamepad: &Gamepad, btn: Button) -> Option<ButtonData> {
    gamepad.button_data(btn).map(|x| *x)
}

pub fn gamepad_axis_data(gamepad: &Gamepad, axis: Axis) -> Option<AxisData> {
    gamepad.axis_data(axis).map(|x| *x)
}

pub fn gamepad_deadzone(gamepad: &Gamepad, axis: Code) -> Option<f64> {
    gamepad.deadzone(axis).map(|x| x as f64)
}

pub fn gamepad_power_info(gamepad: &Gamepad) -> PowerInfoWrapper {
    PowerInfoWrapper::from(gamepad.power_info())
}

pub fn gamepad_id(gamepad: &Gamepad) -> GamepadId {
    gamepad.id().clone()
}

pub fn effect_play(effect: &Effect) -> Result<(), String> {
    effect.play().map_err(|err| err.to_string())
}

pub fn effect_add_gamepad(effect: &Effect, gamepad: &Gamepad) -> Result<(), String> {
    effect.add_gamepad(gamepad).map_err(|err| err.to_string())
}

pub fn effect_set_gain(effect: &Effect, gain: f32) -> Result<(), String> {
    effect.set_gain(gain).map_err(|err| err.to_string())
}

struct EffectBuilderWrapper(Rc<RefCell<EffectBuilder>>);

impl EffectBuilderWrapper {
    fn new() -> Self {
        EffectBuilderWrapper(Rc::new(RefCell::new(EffectBuilder::new())))
    }
    fn add_gamepad(&self, gamepad: &Gamepad) -> EffectBuilderWrapper {
        self.0.borrow_mut().add_gamepad(gamepad);
        EffectBuilderWrapper(self.0.clone())
    }
    fn add_effect(&self, effect: BaseEffect) -> EffectBuilderWrapper {
        self.0.borrow_mut().add_effect(effect);
        EffectBuilderWrapper(self.0.clone())
    }
    fn finish(&self, gilrs: &mut Gilrs) -> Result<Effect, String> {
        self.0.borrow_mut().finish(gilrs).map_err(|err| err.to_string())
    }
}

include!(concat!(env!("OUT_DIR"), "/lib.rs"));