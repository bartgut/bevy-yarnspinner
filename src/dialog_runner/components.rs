use bevy::prelude::{Bundle, Component};
use bevy::time::Timer;
use crate::parsing::components::Tag;

#[derive(Clone, Debug)]
pub enum DialogState {
    Start,
    Dialog,
    Waiting,
    End,
}

#[derive(Clone, Debug)]
pub struct DialogOption {
    pub text: String,
    pub node: String,
    pub used: bool,
}

#[derive(Clone, Debug, Component)]
pub enum DialogEvent {
    Dialog {
        speaker: String,
        text: String,
        tags: Vec<Tag>,
    },
    Options {
        speaker: String,
        options: Vec<DialogOption>,
    },
    Waiting,
    End,
}

#[derive(Component)]
pub enum DialogEventOwnership {
    PARENT,
    TIMER(f32),
}

#[derive(Bundle)]
pub struct DialogEventBundle {
    pub event: DialogEvent,
    pub ownership: DialogEventOwnership,
}

#[derive(Component)]
pub struct DialogEventTimer(pub Timer);
