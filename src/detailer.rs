use std::{
    fmt::{Arguments, Write},
    sync::{atomic::AtomicUsize, Arc},
    time::Instant,
};

/// An event or workflow detail logger.
///
/// When dropped or flush()ed it will output its accumulated input.
pub struct Detailer {
    level: log::LevelFilter,
    accumulated: String,
    current_indentation: Arc<AtomicUsize>,
    start: Option<Instant>,
}

/// Configure the time logging prefix of detail lines
pub enum TimingSetting {
    /// Include timing info in line prefixes
    WithTiming,
    /// Do not include timing info in line prefixes
    WithoutTiming,
}

/// Create a new root detailer. It will log as 1 expression upon
/// being dropped or flushed.
///
/// ```rust
/// use detailer::{Detailer, detail, new_detailer};
/// let mut detailer = new_detailer!();
///
/// let mut detailer_at_debug = new_detailer!(Debug);
///
/// let mut detailer_at_debug_without_timing = new_detailer!(Debug, WithoutTiming);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! new_detailer {
    // scope!(detailer, "scope {}", "log")
    () => {
        detailer::Detailer::new(log::LevelFilter::Info, detailer::TimingSetting::WithTiming)
    };
    // scope!(detailer, "scope {}", "log")
    ($level:tt) => {
        detailer::Detailer::new(
            log::LevelFilter::$level,
            detailer::TimingSetting::WithTiming,
        )
    };
    // scope!(detailer, "scope {}", "log")
    ($level:tt, $timing_setting:tt) => {
        detailer::Detailer::new(
            log::LevelFilter::$level,
            detailer::TimingSetting::$timing_setting,
        )
    };
}

/// Add a detail line at info
/// ```rust
/// use detailer::{Detailer, detail, new_detailer};
/// let mut detailer = new_detailer!();
///
/// detail!(detailer, "some info {}", 24);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! detail {
    // detail!(detailer, "a {} event", "log")
    ($detail_tracker:expr, $($arg:tt)+) => {
        ($detail_tracker.log(
            log::Level::Info,
            core::format_args!($($arg)+))
        );
    };
}

/// Add a detail line at a specified level
/// ```rust
/// use detailer::{Detailer, detail_at, new_detailer};
/// let mut detailer = new_detailer!();
///
/// detail_at!(detailer, Error, "yikes {}", 25);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! detail_at {
    // detail!(detailer, Debug, "a {} event", "log")
    ($detail_tracker:expr, $log_level:tt, $($arg:tt)+) => {
        ($detail_tracker.log(
            log::Level::$log_level,
            core::format_args!($($arg)+))
        );
    };
}

/// Add a lexical scope indentation to the detail
///
/// You can go in multiple levels, but be aware that scopes
/// bypass log level (other than Off) and will always show
/// up in the output. Use them for clarity, but don't
/// overuse them or your output might get hard to read.
/// ```rust
/// use detailer::{Detailer, scope, new_detailer, detail};
/// let mut detailer = new_detailer!();
///
/// detail!(detailer, "not indented");
/// {
///     let _scope_1 = scope!(detailer, "expensive {} under this scope", "work");
///     detail!(detailer, "indented");
///     detail!(detailer, "also indented");
/// }
/// detail!(detailer, "not indented");
/// ```
#[macro_export(local_inner_macros)]
macro_rules! scope {
    // scope!(detailer, "scope {}", "log")
    ($detail_tracker:expr, $($arg:tt)+) => {
        ($detail_tracker.scope(
            core::format_args!($($arg)+))
        );
    };
}

impl Detailer {
    /// Create a new event Detailer logger.
    ///
    /// When dropped or flush()ed it will output its accumulated input.
    pub fn new(level: log::LevelFilter, timing_setting: TimingSetting) -> Detailer {
        Detailer {
            level,
            accumulated: Default::default(),
            current_indentation: Default::default(),
            start: match timing_setting {
                TimingSetting::WithTiming => Some(Instant::now()),
                TimingSetting::WithoutTiming => None,
            },
        }
    }

    /// See what's currently accumulated
    pub fn peek(&self) -> &str {
        &self.accumulated
    }

    /// Remove the contents and reset the timer (if enabled)
    pub fn reset(&mut self) {
        self.accumulated.clear();
        if self.start.is_some() {
            self.start = Some(Instant::now());
        }
    }

    /// Output and clear the contents
    pub fn flush(&mut self) {
        let to_flush = self.accumulated.trim_end();
        if !to_flush.is_empty() {
            log::log!(
                self.level.to_level().unwrap_or(log::Level::Info),
                "{}",
                to_flush
            );
        }
        self.reset();
    }

    /// Indent output one more level as long as the scope guard exists
    pub fn scope(&mut self, scope_name: Arguments) -> DetailScopeGuard {
        if let Some(level) = self.level.to_level() {
            self.log(level, scope_name);
        }
        DetailScopeGuard::new(self.current_indentation.clone())
    }

    /// log a line, if the level is enabled.
    ///
    /// ```
    /// use detailer::{Detailer, new_detailer};
    ///
    /// let mut detailer = new_detailer!();
    ///
    /// detailer.log(log::Level::Warn, format_args!("yikes {}", 42));
    /// ```
    pub fn log(&mut self, level: log::Level, message: Arguments) {
        if level <= self.level {
            let current_indentation = self
                .current_indentation
                .load(std::sync::atomic::Ordering::Relaxed);
            if 0 < current_indentation {
                let message = message.to_string();
                let mut lines = message.split('\n');
                if let Some(first_line) = lines.next() {
                    if let Some(start) = &self.start {
                        let elapsed = start.elapsed().as_micros() as u64;
                        let _ = self.accumulated.write_fmt(format_args!("{elapsed:<6} "));
                    }
                    for _ in 0..current_indentation {
                        let _ = self.accumulated.write_str("  ");
                    }
                    let _ = self.accumulated.write_fmt(format_args!("{first_line}\n"));
                }
                for line in lines {
                    for _ in 0..current_indentation {
                        let _ = self.accumulated.write_str("  ");
                    }
                    let _ = self.accumulated.write_fmt(format_args!("{line}\n"));
                }
            } else {
                if let Some(start) = &self.start {
                    let elapsed = start.elapsed().as_micros() as u64;
                    let _ = self.accumulated.write_fmt(format_args!("{elapsed:<6} "));
                }
                let _ = self.accumulated.write_fmt(message);
                let _ = self.accumulated.write_char('\n');
            }
        }
    }

    /// log a line
    ///
    /// ```
    /// use detailer::{Detailer, new_detailer};
    ///
    /// let mut detailer = new_detailer!();
    ///
    /// detailer.info(format_args!("yikes {}", 42));
    /// ```
    pub fn info(&mut self, message: Arguments) {
        self.log(log::Level::Info, message)
    }

    /// log a line
    ///
    /// ```
    /// use detailer::{Detailer, new_detailer};
    ///
    /// let mut detailer = new_detailer!();
    ///
    /// detailer.trace(format_args!("yikes {}", 42));
    /// ```
    pub fn trace(&mut self, message: Arguments) {
        self.log(log::Level::Trace, message)
    }

    /// log a line
    ///
    /// ```
    /// use detailer::{Detailer, new_detailer};
    ///
    /// let mut detailer = new_detailer!();
    ///
    /// detailer.debug(format_args!("yikes {}", 42));
    /// ```
    pub fn debug(&mut self, message: Arguments) {
        self.log(log::Level::Debug, message)
    }

    /// log a line
    ///
    /// ```
    /// use detailer::{Detailer, new_detailer};
    /// use log::{Level, LevelFilter};
    ///
    /// let mut detailer = new_detailer!();
    ///
    /// detailer.warn(format_args!("yikes {}", 42));
    /// ```
    pub fn warn(&mut self, message: Arguments) {
        self.log(log::Level::Warn, message)
    }

    /// log a line
    ///
    /// ```
    /// use detailer::{Detailer, new_detailer};
    /// use log::{Level, LevelFilter};
    ///
    /// let mut detailer = new_detailer!();
    ///
    /// detailer.error(format_args!("yikes {}", 42));
    /// ```
    pub fn error(&mut self, message: Arguments) {
        self.log(log::Level::Error, message)
    }
}

impl Drop for Detailer {
    fn drop(&mut self) {
        self.flush()
    }
}

pub struct DetailScopeGuard {
    level: Arc<AtomicUsize>,
}

impl DetailScopeGuard {
    pub fn new(level: Arc<AtomicUsize>) -> Self {
        level.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self { level }
    }
}

impl Drop for DetailScopeGuard {
    fn drop(&mut self) {
        self.level
            .fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }
}
