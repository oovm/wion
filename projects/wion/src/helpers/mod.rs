use std::fmt::Write;

pub(crate) trait IndentDebug {
    fn debug_indent<W: Write>(&self, f: &mut IndentFormatter<'_, W>, level: usize) -> std::fmt::Result;
}

pub(crate) struct IndentFormatter<'a, W> {
    writer: W,
    indent_text: &'a str,
    level: usize,
}

impl<'a, W: Write> Write for IndentFormatter<'a, W> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if cfg!(debug_assertions) {
            for c in s.chars() {
                if c == '\n' {
                    eprint!("text contains newline: {}", s);
                }
            }
        }
        self.writer.write_str(s)
    }
}

impl<'a, W: Write> IndentFormatter<'a, W> {
    pub fn indent(&mut self) {
        self.level += 1;
    }
    pub fn dedent(&mut self) {
        self.level -= 1;
    }
    /// Write new line and indent text
    pub fn newline(&mut self) -> std::fmt::Result {
        self.writer.write_char('\n')?;
        for _ in 0..self.level {
            self.writer.write_str(self.indent_text)?;
        }
        Ok(())
    }
}
