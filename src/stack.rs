//! Simple backtrace printer that removes irrelevant frames.

use backtrace::{Backtrace, BacktraceFrame, Frame};

#[cfg(unix)]
const BATMAN_SENTINEL: &str = "<unknown>";

#[cfg(windows)]
const BATMAN_SENTINEL: &str = "KiUserExceptionDispatcher";

// XXX: This is probably unstable. It is used by `#[track_caller]` to remove irrelevant frames.
const RUST_BACKTRACE_SENTINEL: &str = "__rust_begin_short_backtrace";

pub(crate) fn print(frames: Vec<Frame>) {
    eprintln!("\nFloating point exception occurred.");

    let (frames, note) = match std::env::var("RUST_BACKTRACE").as_deref() {
        Ok("full") => (frames, false),
        Ok(_) => {
            let mut iter = frames.iter().enumerate();
            let start = iter
                .find_map(|(i, frame)| resolve(frame, |name| name == BATMAN_SENTINEL).then_some(i))
                .map(|index| index + 1)
                .unwrap_or_default();
            let end = iter
                .find_map(|(i, frame)| {
                    resolve(frame, |name| name.contains(RUST_BACKTRACE_SENTINEL)).then_some(i)
                })
                .unwrap_or(usize::MAX);
            let frames = frames.into_iter().skip(start).take(end - start).collect();

            (frames, true)
        }
        Err(_) => {
            eprintln!(
                "note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace."
            );
            return;
        }
    };

    let frames = frames
        .into_iter()
        .map(BacktraceFrame::from)
        .collect::<Vec<_>>();
    let mut trace = Backtrace::from(frames);
    trace.resolve();

    eprintln!("{trace:?}");
    if note {
        eprintln!(
            "note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace."
        );
    }
}

fn resolve<F>(frame: &Frame, predicate: F) -> bool
where
    F: Fn(&str) -> bool + Copy,
{
    let mut name = None;

    backtrace::resolve_frame(frame, |symbol| {
        name = symbol
            .name()
            .and_then(|symbol| symbol.as_str())
            .map(predicate);
    });

    name.unwrap_or(true)
}
