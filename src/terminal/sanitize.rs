/// Strip ESC control sequences so TUI rendering is not corrupted by stray ANSI bytes.
pub fn sanitize_for_tui(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip CSI / OSC sequences
            if let Some(&next) = chars.peek()
                && (next == '[' || next == ']')
            {
                chars.next();
                for nc in chars.by_ref() {
                    if (next == '[' && nc.is_ascii_alphabetic()) || (next == ']' && nc == '\x07') {
                        break;
                    }
                }
                continue;
            }
        }
        out.push(c);
    }
    out
}
