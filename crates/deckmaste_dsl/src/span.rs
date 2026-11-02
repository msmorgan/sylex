//! Source locations.

/// Identifies one source file among those a caller parses.
///
/// The parser only copies it into every [`Span`]; the caller decides what
/// each id refers to.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct FileId(pub u32);

/// A byte range in one source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub file: FileId,
    /// Byte offset of the first byte.
    pub start: usize,
    /// Byte offset just past the last byte.
    pub end: usize,
}

impl Span {
    #[must_use]
    pub fn new(file: FileId, start: usize, end: usize) -> Self {
        Self { file, start, end }
    }

    /// The smallest span covering both `self` and `other`.
    #[must_use]
    pub fn to(self, other: Span) -> Span {
        Span {
            file: self.file,
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// The text this span covers in `source`.
    ///
    /// # Panics
    ///
    /// If the span is not a valid range of `source`.
    #[must_use]
    pub fn text(self, source: &str) -> &str {
        &source[self.start..self.end]
    }

    /// The line and column of the span's start in `source`.
    #[must_use]
    pub fn start_line_col(self, source: &str) -> LineCol {
        LineCol::of(source, self.start)
    }

    /// The line and column of the span's end in `source`.
    #[must_use]
    pub fn end_line_col(self, source: &str) -> LineCol {
        LineCol::of(source, self.end)
    }
}

/// A 1-based line and column; the column counts characters, not bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct LineCol {
    pub line: usize,
    pub column: usize,
}

impl LineCol {
    /// The line and column of byte `offset` in `source`.
    ///
    /// An offset past the end, or inside a multi-byte character, is clamped
    /// back to the nearest character boundary.
    #[must_use]
    pub fn of(source: &str, offset: usize) -> LineCol {
        let mut offset = offset.min(source.len());
        while !source.is_char_boundary(offset) {
            offset -= 1;
        }
        let before = &source[..offset];
        let line_start = before.rfind('\n').map_or(0, |i| i + 1);
        LineCol {
            line: before.matches('\n').count() + 1,
            column: before[line_start..].chars().count() + 1,
        }
    }
}

impl std::fmt::Display for LineCol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
