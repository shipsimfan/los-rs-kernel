use crate::Event;
use alloc::{string::String, vec::Vec};

enum FormatBlock {
    String(String),
    ModuleName,
    Message,
    Level,
    LevelCaps,
    LevelNoCaps,
}

pub(crate) struct Formatter {
    blocks: Vec<FormatBlock>,
    base_length: usize,
    module_name_multiplier: usize,
    message_multiplier: usize,
    level_multiplier: usize,
}

pub(crate) struct ParsedFormatString {
    blocks: Vec<FormatBlock>,
    base_length: usize,
    module_name_multiplier: usize,
    message_multiplier: usize,
    level_multiplier: usize,
}

impl Formatter {
    pub(crate) fn new() -> Self {
        let parsed_format_string = ParsedFormatString::parse("[{{Le}}][{{Mo}}] {{Me}}\n");
        Formatter {
            blocks: parsed_format_string.blocks,
            base_length: parsed_format_string.base_length,
            module_name_multiplier: parsed_format_string.module_name_multiplier,
            message_multiplier: parsed_format_string.message_multiplier,
            level_multiplier: parsed_format_string.level_multiplier,
        }
    }

    pub(crate) fn format(&self, event: Event) -> String {
        // Precalculate string length
        let mut length = self.base_length;
        length += self.module_name_multiplier * event.module().len();
        length += self.message_multiplier * event.message().len();
        length += self.level_multiplier * event.level().display().len();

        // Allocate the string
        let mut output = String::with_capacity(length);

        // Build the string
        for block in &self.blocks {
            block.add_output(&mut output, &event);
        }

        output
    }

    pub(crate) fn set_format_string(&mut self, parsed_format_string: ParsedFormatString) {
        self.blocks = parsed_format_string.blocks;
    }
}

impl ParsedFormatString {
    pub(crate) fn parse<S: AsRef<str>>(str: S) -> Self {
        let mut chars = str.as_ref().chars();
        let mut blocks = Vec::new();
        let mut base_length = 0;
        let mut module_name_multiplier = 0;
        let mut message_multiplier = 0;
        let mut level_multiplier = 0;
        let mut current_string = String::new();
        while let Some(c) = chars.next() {
            if c != '{' {
                current_string.push(c);
                continue;
            }

            let c = match chars.next() {
                Some(c) => c,
                None => {
                    current_string.push('{');
                    break;
                }
            };

            if c != '{' {
                current_string.push('{');
                current_string.push(c);
                continue;
            }

            let c1 = match chars.next() {
                Some(c) => c,
                None => {
                    current_string.push_str("{{");
                    break;
                }
            };

            let c2 = match chars.next() {
                Some(c) => c,
                None => {
                    current_string.push_str("{{");
                    current_string.push(c1);
                    break;
                }
            };

            let c = match chars.next() {
                Some(c) => c,
                None => {
                    current_string.push_str("{{");
                    current_string.push(c1);
                    current_string.push(c2);
                    break;
                }
            };

            if c != '{' {
                current_string.push_str("{{");
                current_string.push(c1);
                current_string.push(c2);
                current_string.push(c);
                continue;
            }

            let c = match chars.next() {
                Some(c) => c,
                None => {
                    current_string.push_str("{{");
                    current_string.push(c1);
                    current_string.push(c2);
                    current_string.push('}');
                    break;
                }
            };

            if c != '{' {
                current_string.push_str("{{");
                current_string.push(c1);
                current_string.push(c2);
                current_string.push('}');
                current_string.push(c);
                continue;
            }

            match &[c1.to_ascii_lowercase(), c2.to_ascii_lowercase()] {
                &['m', 'e'] | &['m', 'o'] | &['l', 'e'] => {
                    if current_string.len() > 0 {
                        base_length += current_string.len();
                        blocks.push(FormatBlock::String(current_string));
                        current_string = String::new();
                    }

                    blocks.push(if c1.to_ascii_lowercase() == 'm' {
                        if c2.to_ascii_lowercase() == 'e' {
                            message_multiplier += 1;
                            FormatBlock::Message
                        } else {
                            module_name_multiplier += 1;
                            FormatBlock::ModuleName
                        }
                    } else {
                        level_multiplier += 1;
                        if c1 == 'L' {
                            if c2 == 'E' {
                                FormatBlock::LevelCaps
                            } else {
                                FormatBlock::Level
                            }
                        } else {
                            FormatBlock::LevelNoCaps
                        }
                    })
                }
                _ => {
                    current_string.push_str("{{");
                    current_string.push(c1);
                    current_string.push(c2);
                    current_string.push_str("}}");
                }
            }
        }

        if current_string.len() > 0 {
            base_length += current_string.len();
            blocks.push(FormatBlock::String(current_string));
        }

        ParsedFormatString {
            blocks,
            base_length,
            module_name_multiplier,
            message_multiplier,
            level_multiplier,
        }
    }
}

impl FormatBlock {
    pub(self) fn add_output(&self, output: &mut String, event: &Event) {
        output.push_str(match self {
            FormatBlock::String(string) => string,
            FormatBlock::ModuleName => event.module(),
            FormatBlock::Message => event.message(),
            FormatBlock::Level => event.level().display(),
            FormatBlock::LevelCaps => event.level().display_upper(),
            FormatBlock::LevelNoCaps => event.level().display_lower(),
        })
    }
}
