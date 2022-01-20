//! Simple backtrace wrapper that removes irrelevant frames.

// XXX: This is probably unstable. It is used by `#[track_caller]` to remove irrelevant frames.
const RUST_BACKTRACE_SENTINEL: &str = "__rust_begin_short_backtrace";

pub(crate) struct Backtrace {
    inner: std::backtrace::Backtrace,
}

impl Backtrace {
    /// Create a new `Backtrace` wrapper.
    pub(crate) fn new() -> Self {
        Self {
            inner: std::backtrace::Backtrace::force_capture(),
        }
    }

    /// Print this backtrace to a string, stripping irrelevant frames.
    ///
    /// TODO: Patch frame numbers so they start at 0.
    pub(crate) fn as_string(&self) -> String {
        let bt = format!("{}", self.inner);

        let mut found_batman = false;
        let mut start = 0;
        let mut end = 0;

        // Trim irrelevant frames
        // Note: Don't use str::lines() here, because we want to count any `\r` characters.
        for line in bt.split('\n') {
            if !found_batman {
                if !line.contains("batman") {
                    start += line.len() + 1;
                    continue;
                }

                found_batman = true;
            } else if line.contains(RUST_BACKTRACE_SENTINEL) {
                break;
            }

            end += line.len() + 1;
        }

        bt.split_at(start).1.split_at(end).0.to_string()
    }
}
