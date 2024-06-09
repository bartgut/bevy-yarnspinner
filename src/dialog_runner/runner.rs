use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, RwLock, Weak};

use bevy::prelude::*;
use lazy_static::lazy_static;

use crate::dialog_runner::components::{DialogEvent, DialogOption, DialogState};
use crate::dialog_runner::context::StateContext;
use crate::dialog_runner::dialog_runner_error::DialogRunnerError;
use crate::dialog_runner::dialog_runner_error::DialogRunnerError::{StartingNodeNotFound, UnknownNodeChosen, WrongState};
use crate::parsing::components::{ConditionType, LineType, OptionPossibility, YarnSpinnerNode};

pub type CommandFn = Box<dyn Fn(&mut Commands, &mut dyn Iterator<Item = String>) + Send + Sync>;
lazy_static! {
    pub static ref COMMAND_REGISTRY: Mutex<HashMap<String, CommandFn>> = Mutex::new(HashMap::new());
}

pub struct DialogRunner<T: StateContext> {
    nodes: Vec<Arc<RwLock<YarnSpinnerNode>>>,
    current_node: Weak<RwLock<YarnSpinnerNode>>,
    current_line_index: usize,
    dialog_state: DialogState,
    _phantom: PhantomData<T>,
}

impl<T: StateContext> DialogRunner<T> {
    pub fn create_from_nodes(nodes: Vec<Arc<RwLock<YarnSpinnerNode>>>, start_node_title: &str) -> Result<Self, DialogRunnerError> {
        let current_node = nodes
            .iter()
            .find(|node| node.read().unwrap().title == start_node_title)
            .ok_or(StartingNodeNotFound { node_name: start_node_title.to_string() })?;

        let weak_current_node = Arc::downgrade(current_node);

        Ok(Self {
            nodes,
            current_node: weak_current_node,
            current_line_index: 0,
            dialog_state: DialogState::Start,
            _phantom: PhantomData,
        })
    }

    pub fn next_event(&mut self, context: &mut T, commands: &mut Commands) -> DialogEvent {
        match self.dialog_state {
            DialogState::Start | DialogState::Dialog => self.handle_dialog(context, commands),
            DialogState::Waiting => DialogEvent::Waiting,
            DialogState::End => DialogEvent::End,
        }
    }

    pub fn make_decision(&mut self, decision: &str) -> Result<(), DialogRunnerError> {
        if let DialogState::Waiting = self.dialog_state {
            self.update_used(decision);
            self.current_node = Arc::downgrade(self
                .nodes
                .iter()
                .find(|node| node.read().unwrap().title == decision)
                .ok_or(UnknownNodeChosen { node_name: decision.to_string() })?);
            self.current_line_index = 0;
            self.dialog_state = DialogState::Start;
            Ok(())
        } else {
            Err(WrongState { current: self.dialog_state.clone(), expected: DialogState::Waiting })
        }
    }

    pub fn reset_to(&mut self, node_title: &str) -> Result<(), String> {
        self.current_node = Arc::downgrade(self
            .nodes
            .iter()
            .find(|node| node.read().unwrap().title == node_title)
            .ok_or("Could not reset to selected node(wrong node title?")?);
        self.current_line_index = 0;
        self.dialog_state = DialogState::Start;
        Ok(())
    }

    fn handle_dialog(&mut self, context: &mut T, commands: &mut Commands) -> DialogEvent {
        self.update_context(context);
        self.perform_jump();
        self.execute_command(commands);
        self.process_event(context, commands)
    }

    fn update_used(&mut self, decision: &str) {
        let node_arc = self.current_node.upgrade().unwrap();
        let mut node = node_arc.write().unwrap();
        let current_line_mut = &mut node.lines[self.current_line_index];
        if let LineType::OptionLine {
            speaker: _speaker,
            possibilities,
        } = current_line_mut
        {
            let possibility = possibilities
                .iter_mut()
                .find(|possibility| possibility.jump_to_node_title == decision)
                .expect("No possibility found");
            possibility.used = true;
        }
    }

    fn process_event(&mut self, context: &mut T, commands: &mut Commands) -> DialogEvent {
        let event = self.line_to_event(&self.current_line(), context);
        self.dialog_state = Self::event_to_dialog_state(&event);
        self.move_pointer();
        match self.dialog_state {
            DialogState::Start => self.next_event(context, commands),
            DialogState::End => DialogEvent::End,
            _ => event.unwrap(),
        }
    }

    fn current_line(&self) -> LineType {
        let current_node = self.current_node.upgrade().unwrap();
        let x = current_node.read().unwrap().lines[self.current_line_index].clone();
        x
    }

    fn line_to_event(&self, line: &LineType, context: &T) -> Option<DialogEvent> {
        match line {
            LineType::DialogLine {
                speaker,
                text,
                tags,
            } => Some(DialogEvent::Dialog {
                speaker: speaker.clone(),
                text: text.clone(),
                tags: tags.clone(),
            }),
            LineType::OptionLine {
                speaker,
                possibilities,
            } => {
                let options = possibilities
                    .iter()
                    .filter(|&x| self.passes_condition(x, context))
                    .map(|possibility| DialogOption {
                        text: possibility.text.clone(),
                        node: possibility.jump_to_node_title.clone(),
                        used: possibility.used.clone(),
                    })
                    .collect();
                Some(DialogEvent::Options {
                    speaker: speaker.clone(),
                    options,
                })
            }
            _ => None,
        }
    }

    fn passes_condition(&self, possibility: &OptionPossibility, context: &T) -> bool {
        match &possibility.condition {
            Some(condition) => match context.get_value(&condition.variable_name) {
                Some(value) => match condition.condition {
                    ConditionType::Equal => value == &condition.value,
                    ConditionType::NotEqual => value != &condition.value,
                },
                None => false,
            },
            None => true,
        }
    }

    fn update_context(&mut self, context: &mut T) {
        if let LineType::SetLine {
            variable_name,
            value,
        } = &self.current_line()
        {
            context.set_value(variable_name, value);
        }
    }

    fn perform_jump(&mut self) {
        if let LineType::JumpLine { node, node_title} = &self.current_line() {
            self.current_node = node.clone();
            self.current_line_index = 0;
        }
    }

    fn execute_command(&mut self, commands: &mut Commands) {
        if let LineType::CommandLine { func_name, args } = &self.current_line() {
            COMMAND_REGISTRY.lock().unwrap().get(func_name).unwrap()(
                commands,
                &mut args.clone().into_iter(),
            );
        }
    }

    fn move_pointer(&mut self) {
        match self.dialog_state {
            DialogState::Waiting => {}
            _ => {
                self.current_line_index += 1;
                if self.current_line_index >= self.current_node.upgrade().unwrap().read().unwrap().lines.len() {
                    self.dialog_state = DialogState::End;
                }
            }
        }
    }

    fn event_to_dialog_state(event: &Option<DialogEvent>) -> DialogState {
        match event {
            Some(DialogEvent::Dialog {
                speaker: _speaker,
                text: _text,
                tags: _tags,
            }) => DialogState::Dialog,
            Some(DialogEvent::Options {
                speaker: _speaker,
                options: _options,
            }) => DialogState::Waiting,
            Some(DialogEvent::Waiting) => DialogState::Waiting,
            Some(DialogEvent::End) => DialogState::End,
            None => DialogState::Start, // TODO INNY StATE?
        }
    }

}
