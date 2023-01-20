# Logger
A logger represents one module which may log information. A logger contains the modules name and sends it along with every event. It also holds an `Arc` to the log controller so that it may be used anywhere, anytime, without any problems.

## Creating a logger
Creating a logger is done using the function `Logger::new(module_name: &'static str, log_controller: Arc<LogController>)`.

## Using a logger
To log an event using the logger, you can use the function `Logger::log(&self, level: Level, message: String)`