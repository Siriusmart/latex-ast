use std::fmt::Display;

/// Errors caused by incorrect usage of the crate
#[derive(Debug)]
#[cfg_attr(feature = "eq", derive(PartialEq, Eq))]
pub enum InternalError {
    /// Chunk containing unsanitised character
    UnsanitisedCharInString(char),
    /// Incorrect line number for a chunk
    IncorrectChunkLineNumber { expected: u32, got: u32 },
    /// ParagraphBreak does not contain enough line breaks
    ParagraphBreakTooShort,
    /// Nonwhitespace in ParagraphBreak
    ParagraghBreakNonwhitespace,
}

impl Display for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

impl std::error::Error for InternalError {}
