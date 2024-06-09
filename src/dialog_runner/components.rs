use std::fmt::{Display, Formatter};
use bevy::prelude::{Bundle, Component};
use bevy::time::Timer;
use crate::parsing::components::{Tag};

#[derive(Clone, Debug)]
pub enum DialogState {
    Start,
    Dialog,
    Waiting,
    End,
}

impl Display for DialogState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DialogState::Start => write!(f, "Start"),
            DialogState::Dialog => write!(f, "Dialog"),
            DialogState::Waiting => write!(f, "Waiting"),
            DialogState::End => write!(f, "End")
        }
    }
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

#[derive(Component)]
pub struct CurrentDialogEvent;
