pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

struct Filter<L: Logger, P: Fn(u8, &str) -> bool> {
    inner: L,
    predicate: P
}

impl <L: Logger, P: Fn(u8, &str) -> bool> Filter<L, P> {
    fn new(inner: L, predicate: P) -> Self {
        Filter { inner, predicate }
    }
}

impl <L: Logger, P: Fn(u8, &str) -> bool> Logger for Filter<L, P> {
    fn log(&self, verbosity: u8, message: &str) {
        if (self.predicate)(verbosity, message) {
            self.inner.log(verbosity, message);
        }
    }
}

fn main() {
    let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
    logger.log(5, "FYI");
    logger.log(1, "yikes, something went wrong");
    logger.log(2, "uhoh");
    logger.log(100, "something went wrong, yikes!");
}