use crate::{
    formatter::{Formatter, ParsedFormatString},
    Event, Level, LogOutput,
};
use alloc::sync::Arc;

pub struct LogController {
    output: Option<Arc<dyn LogOutput>>,
    minimum_level: Level,
    // TODO: Change to a RwLock<Formatter>
    formatter: Formatter,
}

#[cfg(debug_assertions)]
const DEFAULT_MINIMUM_LEVEL: Level = Level::Debug;

#[cfg(not(debug_assertions))]
const DEFAULT_MINIMUM_LEVEL: Level = Level::Warning;

impl LogController {
    pub fn new() -> Arc<Self> {
        // TODO: Return an Arc<RWLock<Self>>
        Arc::new(LogController {
            output: None,
            minimum_level: DEFAULT_MINIMUM_LEVEL,
            formatter: Formatter::new(),
        })
    }

    pub fn log(&self, event: Event) {
        if event.level() >= self.minimum_level {
            self.output
                .as_ref()
                .map(|output| output.write(&self.formatter.format(event)));
        }
    }

    pub fn set_output<O: LogOutput>(&mut self, output: Arc<O>) {
        self.output = Some(output);
    }

    pub fn clear_output(&mut self) {
        self.output = None;
    }

    pub fn set_minimum_level(&mut self, minimum_level: Level) {
        self.minimum_level = minimum_level;
    }

    pub fn set_format_string(&self, format_string: &str) {
        let parse_format_string = ParsedFormatString::parse(format_string);
        //self.formatter.write().set_format_string(parsed_format_string);
    }
}
