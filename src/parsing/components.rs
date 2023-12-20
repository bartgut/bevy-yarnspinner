#[derive(Clone, Debug)]
pub struct Condition {
    pub variable_name: String,
    pub condition: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct Tag {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct OptionPossibility {
    pub text: String,
    pub jump_to_node: String,
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
    },
    OptionLine {
        speaker: String,
        possibilities: Vec<OptionPossibility>,
    },
}

#[derive(Clone, Debug)]
pub struct YarnSpinnerNode {
    pub title: String,
    pub lines: Vec<LineType>,
}
