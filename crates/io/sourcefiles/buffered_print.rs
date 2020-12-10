use std::io::{self, BufWriter, StdoutLock, Write as _};

pub fn buf_print<F: FnOnce(&mut BufWriter<StdoutLock<'_>>)>(f: F) {
    let stdout = io::stdout();
    let mut stdout = BufWriter::new(stdout.lock());
    f(&mut stdout);
    stdout.flush().unwrap();
}
