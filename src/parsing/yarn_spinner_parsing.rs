use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use super::components::*;

#[derive(Parser)]
#[grammar = "assets/grammar/yarnspinner.pest"]
pub struct YarnSpinnerParser;

pub fn load_from_file(dialog: &str) -> Vec<YarnSpinnerNode> {
    let parsed = YarnSpinnerParser::parse(Rule::yarnspinner, dialog).expect("unsuccessful parse");
    parsed.into_iter().map(parse_section).collect()
}

fn parse_section(section: Pair<Rule>) -> YarnSpinnerNode {
    let mut node = YarnSpinnerNode {
        title: String::new(),
        lines: vec![],
    };

    if section.as_rule() == Rule::section {
        for field in section.into_inner() {
            match field.as_rule() {
                Rule::title => node.title = field.as_str().to_string(),
                Rule::section_content => {
                    for content in field.into_inner() {
                        node.lines.push(parse_content(content));
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    node
}

fn parse_content(content: Pair<Rule>) -> LineType {
    match content.as_rule() {
        Rule::set_line => parse_set_line(content),
        Rule::command_line => parse_command_line(content),
        Rule::dialog_line => parse_dialog_line(content),
        Rule::option_lines => parse_option_lines(content),
        Rule::jump_line => parse_jump_line(content),
        _ => unreachable!(),
    }
}

fn parse_set_line(content: Pair<Rule>) -> LineType {
    let mut variable_name = String::new();
    let mut value = false;

    for set_line_field in content.into_inner() {
        match set_line_field.as_rule() {
            Rule::variable_name => variable_name = set_line_field.as_str().to_string(),
            Rule::boolean_value => value = set_line_field.as_str().parse::<bool>().unwrap(),
            _ => unreachable!(),
        }
    }

    LineType::SetLine {
        variable_name,
        value,
    }
}

fn parse_command_line(content: Pair<Rule>) -> LineType {
    let mut func_name = String::new();
    let mut args: Vec<String> = vec![];

    for command_line_field in content.into_inner() {
        match command_line_field.as_rule() {
            Rule::function_name => func_name = command_line_field.as_str().to_string(),
            Rule::args => {
                for command_arg_field in command_line_field.into_inner() {
                    match command_arg_field.as_rule() {
                        Rule::arg => args.push(command_arg_field.as_str().to_string()),
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    LineType::CommandLine { func_name, args }
}

fn parse_dialog_line(content: Pair<Rule>) -> LineType {
    let mut speaker = String::new();
    let mut text = String::new();
    let mut tags: Vec<Tag> = vec![];

    for dialog_line_field in content.into_inner() {
        match dialog_line_field.as_rule() {
            Rule::speaker => speaker = dialog_line_field.as_str().to_string(),
            Rule::dialog => text = dialog_line_field.as_str().to_string(),
            Rule::tags => tags.push(parse_tag(dialog_line_field)),
            _ => unreachable!(),
        }
    }

    LineType::DialogLine {
        speaker,
        text,
        tags,
    }
}

fn parse_tag(content: Pair<Rule>) -> Tag {
    let mut name = String::new();
    let mut value = String::new();

    for tag_field in content.into_inner() {
        match tag_field.as_rule() {
            Rule::tag_name => name = tag_field.as_str().to_string(),
            Rule::tag_value => value = tag_field.as_str().to_string(),
            _ => unreachable!(),
        }
    }

    Tag { name, value }
}

fn parse_option_lines(content: Pair<Rule>) -> LineType {
    let mut option_possibilities: Vec<OptionPossibility> = vec![];
    let speaker = String::new();

    for option_lines_field in content.into_inner() {
        match option_lines_field.as_rule() {
            Rule::option_line => {
                let mut text = String::new();
                let mut node_title = String::new();
                let mut condition: Option<Condition> = None;

                for option_line_field in option_lines_field.into_inner() {
                    match option_line_field.as_rule() {
                        Rule::option_dialog_line => {
                            for dialog_line_field in option_line_field.into_inner() {
                                match dialog_line_field.as_rule() {
                                    Rule::speaker => {}
                                    Rule::dialog => text = dialog_line_field.as_str().to_string(),
                                    Rule::if_statement => {
                                        condition = Some(parse_if_statement(dialog_line_field))
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                        Rule::jump_line => {
                            for jump_line_field in option_line_field.into_inner() {
                                match jump_line_field.as_rule() {
                                    Rule::title => {
                                        node_title = jump_line_field.as_str().to_string()
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                        _ => unreachable!(),
                    }
                }

                option_possibilities.push(OptionPossibility {
                    text,
                    jump_to_node: node_title,
                    condition,
                    used: false,
                });
            }
            _ => unreachable!(),
        }
    }

    LineType::OptionLine {
        speaker,
        possibilities: option_possibilities,
    }
}

fn parse_if_statement(dialog_line_field: Pair<Rule>) -> Condition {
    let mut variable_name = String::new();
    let mut condition_sign = String::new();
    let mut value = String::new();

    for if_statement_field in dialog_line_field.into_inner() {
        match if_statement_field.as_rule() {
            Rule::variable_name => variable_name = if_statement_field.as_str().to_string(),
            Rule::condition => condition_sign = if_statement_field.as_str().to_string(),
            Rule::boolean_value => value = if_statement_field.as_str().to_string(),
            _ => unreachable!(),
        }
    }
    Condition {
        variable_name,
        condition: condition_sign,
        value,
    }
}

fn parse_jump_line(content: Pair<Rule>) -> LineType {
    let node_title = content
        .into_inner()
        .find_map(|field| match field.as_rule() {
            Rule::title => Some(field.as_str().to_string()),
            _ => None,
        })
        .expect("Jump line missing title");

    LineType::JumpLine { node_title }
}
