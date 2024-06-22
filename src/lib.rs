use anyhow::Result;
use console::Term;

pub trait WriteLineBreak {
    /// Write an empty line to the terminal
    ///
    /// ```
    /// terminal.write_line_break()?;
    /// ```
    fn write_line_break(&self) -> Result<()>;
}

impl WriteLineBreak for Term {
    fn write_line_break(&self) -> Result<()> {
        self.write_str("\n")?;
        Ok(())
    }
}

pub trait Clear {
    /// Clear the terminal
    ///
    /// ```
    /// terminal.clear()?;
    /// ```
    fn clear(&self) -> Result<()>;
}

impl Clear for Term {
    fn clear(&self) -> Result<()> {
        self.clear_last_lines(999)?;
        Ok(())
    }
}
