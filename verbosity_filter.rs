trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

/// Only log messages up to the given verbosity level.
struct VerbosityFilter<L: Logger> {
    max_verbosity: u8,
    inner: L,
}

impl <L:Logger> Logger for VerbosityFilter<L> {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}


fn main() {
    // let logger = StderrLogger;
    let logger = VerbosityFilter { max_verbosity: 3, inner: StderrLogger };
    
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}