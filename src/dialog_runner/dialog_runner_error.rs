use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::dialog_runner::components::DialogState;

#[derive(Debug)]
pub enum DialogRunnerError {
    StartingNodeNotFound { node_name: String },
    UnknownNodeChosen { node_name: String },
    WrongState { current: DialogState, expected: DialogState }
}

impl Display for DialogRunnerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DialogRunnerError::StartingNodeNotFound { node_name} =>
                write!(f, "Selected starting node does not exist in this dialog: {}", node_name),
            DialogRunnerError::UnknownNodeChosen { node_name} =>
                write!(f, "Unknown node chose: {}", node_name),
            DialogRunnerError::WrongState { current, expected} =>
                write!(f, "Current state: {}, expected to perform this operation: {}", current, expected)
        }
    }
}

impl Error for DialogRunnerError {}