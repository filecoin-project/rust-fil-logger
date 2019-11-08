use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

use log::Record;

use flexi_logger::writers::LogWriter;
use flexi_logger::{default_format, DeferredNow, FormatFunction};

/// A `LogWriter` that accepts an open `File`
pub struct SingleFileWriter {
    // The state needs to be mutable; since `Log.log()` requires an unmutable self,
    // which translates into a non-mutating `LogWriter::write()`,
    // we need internal mutability and thread-safety.
    state: Mutex<File>,
    // The format function to use
    format: FormatFunction,
}

impl SingleFileWriter {
    /// Create a a new instance that logs into the given file.
    pub fn new(file: File) -> Self {
        Self {
            state: Mutex::new(file),
            format: default_format,
        }
    }
}

impl LogWriter for SingleFileWriter {
    #[inline]
    fn write(&self, now: &mut DeferredNow, record: &Record) -> std::io::Result<()> {
        let mut file = self.state.lock().unwrap();
        (self.format)(&mut *file, now, record)?;
        // Add a newline as the line formatters don't add one
        writeln!(&mut file)
    }

    #[inline]
    fn flush(&self) -> std::io::Result<()> {
        let mut file = self.state.lock().unwrap();
        file.flush()
    }

    fn format(&mut self, format: FormatFunction) {
        self.format = format;
    }

    #[inline]
    fn max_log_level(&self) -> log::LevelFilter {
        log::LevelFilter::Trace
    }
}
