use std::fs;
use backtrace::{Backtrace, BacktraceSymbol};
use regex::Regex;

pub(crate) fn get_caller_name() -> Option<String> {
    let path_regex = Regex::new(r#"should(-[a-zA-Z0-9\.]+)?(\\|/)src(\\|/)impls"#).unwrap();
    let backtrace = Backtrace::new();
    let frames = backtrace.frames();
    let mut caller_symbol: Option<&BacktraceSymbol> = None;

    for i in (0..frames.len() - 1).rev() {
        if let Some(symbol) = frames[i].symbols().first() {
            if let Some(filename) = symbol.filename() {
                if path_regex.is_match(&filename.to_string_lossy()) {
                    caller_symbol = frames[i + 1].symbols().first();
                    break;
                }
            }
        }
    }

    println!("{:?}", backtrace);

    if let Some(symbol) = caller_symbol {
        if let (Some(filename), Some(line)) = (symbol.filename(), symbol.lineno()) {
            if let Ok(source) = fs::read_to_string(filename) {
                let source_line = source.lines().skip(line as usize - 1).next().unwrap();

                if let Some(end_index) = source_line.find(".should_") {
                    let caller = source_line[..end_index].trim();
                    return Some(caller.to_owned())
                }
            }
        }
    }

    None
}