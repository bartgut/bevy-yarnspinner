use std::str::FromStr;
use std::sync::{RwLock, Weak};
use vec1::Vec1;

#[derive(Clone, Debug)]
pub struct Condition {
    pub variable_name: String,
    pub condition: ConditionType,
    pub value: bool,
}

#[derive(Clone, Debug)]
pub enum ConditionType {
    Equal,
    NotEqual
}

impl FromStr for ConditionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "==" => Ok(ConditionType::Equal),
            "!=" => Ok(ConditionType::NotEqual),
            _ => Err(())
        }
    }
}



#[derive(Clone, Debug)]
pub struct Tag {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct OptionPossibility {
    pub text: String,
    pub jump_to_node_title: String,
    pub jump_to_node: Weak<RwLock<YarnSpinnerNode>>,
    pub condition: Option<Condition>,
    pub used: bool,
}

#[derive(Clone, Debug)]
pub enum LineType {
    SetLine {
        variable_name: String,
        value: bool,
    },
    CommandLine {
        func_name: String,
        args: Vec<String>,
    },
    DialogLine {
        speaker: String,
        text: String,
        tags: Vec<Tag>,
    },
    JumpLine {
        node_title: String,
        node: Weak<RwLock<YarnSpinnerNode>>
    },
    OptionLine {
        speaker: String,
        possibilities: Vec1<OptionPossibility>,
    },
}

#[derive(Clone, Debug)]
pub struct YarnSpinnerNode {
    pub title: String,
    pub lines: Vec1<LineType>,
}
