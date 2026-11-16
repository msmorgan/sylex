//! Parses the `spelling` notation (`docs/design/spelling.md` §7).
//!
//! This module checks the notation's syntax alone: Holes, `~` ties, `@Form`
//! labels, `*` sentence-run marks, `#<…>` list joins, `< … >` optional parts,
//! `%` Tails, `#` and `##` own-name tokens, and the `^` Gap. Load rules that
//! inspect a declaration's parameters, the lexicon or the English grammar live
//! outside the notation parser.

/// A byte range within a `spelling` string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// The slice of `source` this span covers.
    #[must_use]
    pub fn text<'a>(&self, source: &'a str) -> &'a str {
        &source[self.start..self.end]
    }
}

/// A parsed `spelling` (`docs/design/spelling.md` §7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Spelling {
    /// `^`: the whole `spelling` of a word that stands for a Gap.
    Gap(Span),
    /// `#` or `##`: the whole `spelling` of an own-name word (`#` full, `##`
    /// Shortened Name).
    OwnName(OwnName, Span),
    /// `part* [tail]`: fixed text, Holes and optional parts, with an optional
    /// trailing Tail.
    Parts {
        parts: Vec<Part>,
        tail: Option<Tail>,
        span: Span,
    },
}

/// Which form of the enclosing object's own name a [`Spelling::OwnName`]
/// spells (`docs/design/spelling.md` §7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OwnName {
    /// `#`: the full name (`[CR#201.5]`).
    Full,
    /// `##`: the Shortened Name (`[CR#201.5c]`).
    Shortened,
}

impl Spelling {
    #[must_use]
    pub const fn span(&self) -> Span {
        match self {
            Self::Gap(span) | Self::OwnName(_, span) | Self::Parts { span, .. } => *span,
        }
    }
}

/// One in-place piece of a `spelling`, optional part or Tail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Part {
    Text(Text),
    Hole(Hole),
    Optional(Optional),
}

impl Part {
    #[must_use]
    pub const fn span(&self) -> Span {
        match self {
            Self::Text(text) => text.span,
            Self::Hole(hole) => hole.span,
            Self::Optional(optional) => optional.span,
        }
    }
}

/// Fixed characters containing no notation markup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    pub text: String,
    pub span: Span,
}

/// A Hole filled by a parameter's argument or list of arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hole {
    pub name: String,
    pub name_span: Span,
    pub kind: HoleKind,
    pub span: Span,
}

/// What kind of parameter a [`Hole`] reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HoleKind {
    /// `[affix "~"] "$" name [mark] ["~" affix]`: a single argument.
    Argument {
        prefix: Option<Affix>,
        mark: Option<Mark>,
        suffix: Option<Affix>,
    },
    /// `"$" name "#<" join ">"`: a `List` parameter.
    List { join: Join, join_span: Span },
}

/// Bound letters tied to a [`Hole`] by `~`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Affix {
    pub text: String,
    pub span: Span,
}

/// The optional mark after `$name` on an argument [`Hole`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mark {
    /// `@Form`: the printed form of the Hole's head word.
    Form(Form),
    /// `*`: a Hole that a run of sentences may fill.
    Sentences,
}

macro_rules! string_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $(
                $(#[$vmeta:meta])*
                $variant:ident = $text:literal,
            )+
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                $(#[$vmeta])*
                $variant,
            )+
        }

        impl $name {
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $text,)+
                }
            }

            fn from_token(token: &str) -> Option<Self> {
                match token {
                    $($text => Some(Self::$variant),)+
                    _ => None,
                }
            }
        }
    };
}

string_enum! {
    /// The closed set of `@Form` labels (`docs/design/spelling.md` §7).
    pub enum Form {
        Cardinal = "Cardinal",
        Arabic = "Arabic",
        Ordinal = "Ordinal",
        Roman = "Roman",
        Counting = "Counting",
        Present = "Present",
        Preterite = "Preterite",
        PastParticiple = "PastParticiple",
        GerundParticiple = "GerundParticiple",
    }
}

string_enum! {
    /// How a list [`Hole`] joins its elements (`docs/design/spelling.md` §7).
    pub enum Join {
        /// `#<>`: juxtaposes the elements.
        Juxtaposed = "",
        /// `#<,>`: separates the elements with commas and no coordinator.
        Comma = ",",
        /// `#<.>`: prints each element as a run of sentences after the one before.
        Period = ".",
        /// `#<and>`.
        And = "and",
        /// `#<or>`.
        Or = "or",
        /// `#<and/or>`.
        AndOr = "and/or",
        /// `#<then>`.
        Then = "then",
        /// `#<\n>`: puts each element on a line of its own.
        LineBreak = "\n",
    }
}

/// `< … >`: a non-empty optional part.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Optional {
    pub parts: Vec<Part>,
    pub tail: Option<Tail>,
    pub span: Span,
}

/// Where a [`Tail`] attaches among its host phrase's trailing dependents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TailPlacement {
    /// `%Category: …`: outside the host's other trailing dependents.
    Outer,
    /// `%inner Category: …`: at the edge of the host itself, before its other
    /// trailing dependents.
    Inner,
}

/// `%Category: …` or `%inner Category: …`, printed at the edge of an enclosing
/// phrase of the named `Category`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tail {
    pub placement: TailPlacement,
    pub category: String,
    pub category_span: Span,
    pub parts: Vec<Part>,
    pub span: Span,
}

/// A syntax error in a `spelling` string, located by its byte [`Span`].
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{kind}")]
pub struct Error {
    pub span: Span,
    pub kind: ErrorKind,
}

impl Error {
    const fn new(span: Span, kind: ErrorKind) -> Self {
        Self { span, kind }
    }
}

/// Why a `spelling` string failed to parse.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ErrorKind {
    #[error("`^` stands alone as the whole `spelling` of a Gap")]
    GapNotWholeSpelling,
    #[error("Hole abuts a letter or digit with no `~` tie")]
    AbuttingHole,
    #[error("`$` must be directly followed by a parameter name")]
    MissingHoleName,
    #[error("`~` tie requires an affix of letters")]
    MissingAffix,
    #[error("`@` must be directly followed by a form label")]
    MissingForm,
    #[error("unknown form label `{0}`")]
    UnknownForm(String),
    #[error("a list Hole takes `#<join>` and no `~` tie")]
    TieOnListHole,
    #[error("a list Hole takes `#<join>` and no `@` form or `*` mark")]
    MarkedListHole,
    #[error("unclosed `#<` join")]
    UnclosedJoin,
    #[error("join `{0}`: no parameter chooses a join")]
    ParameterJoin(String),
    #[error("unknown join `{0}`")]
    UnknownJoin(String),
    #[error("empty optional part `<>`")]
    EmptyOptional,
    #[error("unclosed `<` optional part")]
    UnclosedOptional,
    #[error("unmatched `>`")]
    UnmatchedCloseAngle,
    #[error("a `spelling` or optional part holds at most one `%` Tail")]
    SecondTail,
    #[error("a `%` Tail holds no Tail, at any depth")]
    NestedTail,
    #[error("a `%` Tail's Category must be one capitalized word, found `{0}`")]
    InvalidTailCategory(String),
    #[error("expected `: ` after `%` Tail Category")]
    MissingTailColon,
    #[error("a `%` Tail cannot be empty")]
    EmptyTail,
    #[error("unexpected markup character `{0}`")]
    UnexpectedMark(char),
}

/// Parses `input` as a `spelling` in the notation of `docs/design/spelling.md` §7.
///
/// # Errors
///
/// The first notation syntax error in `input`, located by its byte [`Span`].
pub fn parse(input: &str) -> Result<Spelling, Error> {
    if input == "^" {
        return Ok(Spelling::Gap(Span::new(0, 1)));
    }
    if input == "#" {
        return Ok(Spelling::OwnName(OwnName::Full, Span::new(0, 1)));
    }
    if input == "##" {
        return Ok(Spelling::OwnName(OwnName::Shortened, Span::new(0, 2)));
    }
    let mut parser = Parser { input, pos: 0 };
    let (parts, tail) = parser.sequence(false, None)?;
    debug_assert_eq!(parser.pos, input.len());
    Ok(Spelling::Parts {
        parts,
        tail,
        span: Span::new(0, input.len()),
    })
}

fn is_markup(c: char) -> bool {
    matches!(c, '$' | '~' | '@' | '*' | '^' | '#' | '%' | '<' | '>')
}

fn is_letter(c: char) -> bool {
    c.is_alphabetic()
}

fn is_letter_or_digit(c: char) -> bool {
    c.is_alphabetic() || c.is_numeric()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TailContext {
    /// Directly parsing the `part+` of a `%` Tail.
    Direct,
    /// Inside an `< … >` optional part nested within a `%` Tail.
    Nested,
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl Parser<'_> {
    fn rest(&self) -> &str {
        &self.input[self.pos..]
    }

    fn peek_char(&self) -> Option<char> {
        self.rest().chars().next()
    }

    fn consume_letters(&mut self) -> Span {
        let start = self.pos;
        while let Some(c) = self.peek_char() {
            if !is_letter(c) {
                break;
            }
            self.pos += c.len_utf8();
        }
        Span::new(start, self.pos)
    }

    /// Parses `part* [tail]`, stopping at `>` when `stop_at_gt` holds.
    fn sequence(
        &mut self,
        stop_at_gt: bool,
        tail_ctx: Option<TailContext>,
    ) -> Result<(Vec<Part>, Option<Tail>), Error> {
        let mut parts: Vec<Part> = Vec::new();
        while let Some(c) = self.peek_char() {
            match c {
                '>' => {
                    if stop_at_gt {
                        return Ok((parts, None));
                    }
                    return Err(Error::new(
                        Span::new(self.pos, self.pos + 1),
                        ErrorKind::UnmatchedCloseAngle,
                    ));
                }
                '<' => {
                    let child_ctx = tail_ctx.map(|_| TailContext::Nested);
                    let optional = self.optional(child_ctx)?;
                    parts.push(Part::Optional(optional));
                }
                '%' => match tail_ctx {
                    Some(TailContext::Direct) => {
                        return Err(Error::new(
                            Span::new(self.pos, self.pos + 1),
                            ErrorKind::SecondTail,
                        ));
                    }
                    Some(TailContext::Nested) => {
                        return Err(Error::new(
                            Span::new(self.pos, self.pos + 1),
                            ErrorKind::NestedTail,
                        ));
                    }
                    None => {
                        let tail = self.tail(stop_at_gt)?;
                        return Ok((parts, Some(tail)));
                    }
                },
                '^' => {
                    return Err(Error::new(
                        Span::new(self.pos, self.pos + 1),
                        ErrorKind::GapNotWholeSpelling,
                    ));
                }
                '@' | '*' | '#' => {
                    return Err(Error::new(
                        Span::new(self.pos, self.pos + 1),
                        ErrorKind::UnexpectedMark(c),
                    ));
                }
                '~' => {
                    let kind = if self.rest().starts_with("~$") {
                        ErrorKind::MissingAffix
                    } else {
                        ErrorKind::UnexpectedMark('~')
                    };
                    return Err(Error::new(Span::new(self.pos, self.pos + 1), kind));
                }
                '$' => {
                    if let Some(Part::Hole(prev)) = parts.last()
                        && matches!(
                            &prev.kind,
                            HoleKind::Argument {
                                suffix: Some(_),
                                ..
                            }
                        )
                    {
                        let hole_end = self.hole_end_preview(self.pos);
                        return Err(Error::new(
                            Span::new(prev.span.start, hole_end),
                            ErrorKind::AbuttingHole,
                        ));
                    }
                    let hole = self.hole(None)?;
                    parts.push(Part::Hole(hole));
                }
                _ => self.text_and_maybe_tied_hole(&mut parts)?,
            }
        }
        Ok((parts, None))
    }

    /// Scans a run of non-markup characters, checking adjacency to a preceding
    /// or following [`Hole`] and peeling any `affix "~"` before `$`.
    fn text_and_maybe_tied_hole(&mut self, parts: &mut Vec<Part>) -> Result<(), Error> {
        let text_start = self.pos;
        while let Some(c) = self.peek_char() {
            if is_markup(c) {
                break;
            }
            self.pos += c.len_utf8();
        }
        let text_end = self.pos;
        let slice = &self.input[text_start..text_end];
        let first_char = slice
            .chars()
            .next()
            .expect("called only when peek_char returned a non-markup character");
        if let Some(Part::Hole(prev)) = parts.last()
            && is_letter_or_digit(first_char)
        {
            return Err(Error::new(
                Span::new(prev.span.start, text_start + first_char.len_utf8()),
                ErrorKind::AbuttingHole,
            ));
        }

        match self.peek_char() {
            Some('~') => {
                if !self.rest().starts_with("~$") {
                    return Err(Error::new(
                        Span::new(text_end, text_end + 1),
                        ErrorKind::UnexpectedMark('~'),
                    ));
                }
                let mut affix_start = text_end;
                while let Some(c) = self.input[text_start..affix_start].chars().next_back() {
                    if !is_letter(c) {
                        break;
                    }
                    affix_start -= c.len_utf8();
                }
                if affix_start == text_end {
                    return Err(Error::new(
                        Span::new(text_end, text_end + 1),
                        ErrorKind::MissingAffix,
                    ));
                }
                if let Some(before_affix) = self.input[text_start..affix_start].chars().next_back()
                    && is_letter_or_digit(before_affix)
                {
                    let hole_end = self.hole_end_preview(text_end + 1);
                    return Err(Error::new(
                        Span::new(affix_start - before_affix.len_utf8(), hole_end),
                        ErrorKind::AbuttingHole,
                    ));
                }
                if affix_start > text_start {
                    parts.push(Part::Text(Text {
                        text: self.input[text_start..affix_start].to_owned(),
                        span: Span::new(text_start, affix_start),
                    }));
                }
                let prefix = Affix {
                    text: self.input[affix_start..text_end].to_owned(),
                    span: Span::new(affix_start, text_end),
                };
                self.pos = text_end + 1;
                let hole = self.hole(Some(prefix))?;
                parts.push(Part::Hole(hole));
                Ok(())
            }
            Some('$') => {
                let last_char = slice.chars().next_back().expect("slice is non-empty");
                if is_letter_or_digit(last_char) {
                    let hole_end = self.hole_end_preview(text_end);
                    return Err(Error::new(
                        Span::new(text_end - last_char.len_utf8(), hole_end),
                        ErrorKind::AbuttingHole,
                    ));
                }
                parts.push(Part::Text(Text {
                    text: slice.to_owned(),
                    span: Span::new(text_start, text_end),
                }));
                Ok(())
            }
            Some('<' | '>' | '%' | '^' | '@' | '*' | '#') | None => {
                parts.push(Part::Text(Text {
                    text: slice.to_owned(),
                    span: Span::new(text_start, text_end),
                }));
                Ok(())
            }
            Some(_) => unreachable!("text scan stops only at markup or end of input"),
        }
    }

    /// End offset of `$name` starting at `dollar_pos`, for error spans.
    fn hole_end_preview(&self, dollar_pos: usize) -> usize {
        let mut end = dollar_pos + 1;
        for c in self.input[end..].chars() {
            if !is_letter(c) {
                break;
            }
            end += c.len_utf8();
        }
        end
    }

    /// Parses a [`Hole`] whose `$` is at `self.pos`.
    fn hole(&mut self, prefix: Option<Affix>) -> Result<Hole, Error> {
        let dollar_pos = self.pos;
        let hole_start = prefix.as_ref().map_or(dollar_pos, |affix| affix.span.start);
        self.pos += 1;

        let name_span = self.consume_letters();
        if name_span.start == name_span.end {
            return Err(Error::new(
                Span::new(dollar_pos, self.pos),
                ErrorKind::MissingHoleName,
            ));
        }
        let name = name_span.text(self.input).to_owned();

        if self.rest().starts_with('#') {
            if !self.rest().starts_with("#<") {
                return Err(Error::new(
                    Span::new(self.pos, self.pos + 1),
                    ErrorKind::UnexpectedMark('#'),
                ));
            }
            if prefix.is_some() {
                return Err(Error::new(
                    Span::new(hole_start, self.pos + 2),
                    ErrorKind::TieOnListHole,
                ));
            }
            let join_open = self.pos;
            self.pos += 2;
            let join_inner_start = self.pos;
            let Some(close_rel) = self.rest().find('>') else {
                return Err(Error::new(
                    Span::new(join_open, self.input.len()),
                    ErrorKind::UnclosedJoin,
                ));
            };
            let join_inner_end = self.pos + close_rel;
            let raw_join = &self.input[join_inner_start..join_inner_end];
            self.pos = join_inner_end + 1;
            let join_span = Span::new(join_open, self.pos);
            let Some(join) = Join::from_token(raw_join) else {
                let kind = if raw_join.starts_with('$') {
                    ErrorKind::ParameterJoin(raw_join.to_owned())
                } else if raw_join.contains('^') {
                    ErrorKind::GapNotWholeSpelling
                } else {
                    ErrorKind::UnknownJoin(raw_join.to_owned())
                };
                return Err(Error::new(join_span, kind));
            };
            if self.rest().starts_with('~') {
                return Err(Error::new(
                    Span::new(hole_start, self.pos + 1),
                    ErrorKind::TieOnListHole,
                ));
            }
            if self.rest().starts_with('@') || self.rest().starts_with('*') {
                return Err(Error::new(
                    Span::new(hole_start, self.pos + 1),
                    ErrorKind::MarkedListHole,
                ));
            }
            return Ok(Hole {
                name,
                name_span,
                kind: HoleKind::List { join, join_span },
                span: Span::new(hole_start, self.pos),
            });
        }

        let mark = if self.rest().starts_with('@') {
            let at_pos = self.pos;
            self.pos += 1;
            let form_span = self.consume_letters();
            if form_span.start == form_span.end {
                return Err(Error::new(
                    Span::new(at_pos, self.pos),
                    ErrorKind::MissingForm,
                ));
            }
            let form_str = form_span.text(self.input);
            let Some(form) = Form::from_token(form_str) else {
                return Err(Error::new(
                    form_span,
                    ErrorKind::UnknownForm(form_str.to_owned()),
                ));
            };
            Some(Mark::Form(form))
        } else if self.rest().starts_with('*') {
            self.pos += 1;
            Some(Mark::Sentences)
        } else {
            None
        };

        if self.rest().starts_with("#<") {
            return Err(Error::new(
                Span::new(hole_start, self.pos + 2),
                ErrorKind::MarkedListHole,
            ));
        }

        let suffix = if self.rest().starts_with('~') {
            let tilde_pos = self.pos;
            self.pos += 1;
            let affix_span = self.consume_letters();
            if affix_span.start == affix_span.end {
                return Err(Error::new(
                    Span::new(tilde_pos, self.pos),
                    ErrorKind::MissingAffix,
                ));
            }
            Some(Affix {
                text: affix_span.text(self.input).to_owned(),
                span: affix_span,
            })
        } else {
            None
        };

        Ok(Hole {
            name,
            name_span,
            kind: HoleKind::Argument {
                prefix,
                mark,
                suffix,
            },
            span: Span::new(hole_start, self.pos),
        })
    }

    /// Parses `< part* [tail] >` starting at `<`.
    fn optional(&mut self, tail_ctx: Option<TailContext>) -> Result<Optional, Error> {
        let start = self.pos;
        self.pos += 1;
        let (parts, tail) = self.sequence(true, tail_ctx)?;
        if self.peek_char() != Some('>') {
            return Err(Error::new(
                Span::new(start, self.pos),
                ErrorKind::UnclosedOptional,
            ));
        }
        self.pos += 1;
        let span = Span::new(start, self.pos);
        if parts.is_empty() && tail.is_none() {
            return Err(Error::new(span, ErrorKind::EmptyOptional));
        }
        Ok(Optional { parts, tail, span })
    }

    /// Parses `"%" ["inner "] Category ": " part+` starting at `%`.
    fn tail(&mut self, stop_at_gt: bool) -> Result<Tail, Error> {
        let start = self.pos;
        self.pos += 1;
        let placement = if self.rest().starts_with("inner ") {
            self.pos += "inner ".len();
            TailPlacement::Inner
        } else {
            TailPlacement::Outer
        };

        let cat_start = self.pos;
        while let Some(c) = self.peek_char() {
            if !(is_letter_or_digit(c) || c == '_') {
                break;
            }
            self.pos += c.len_utf8();
        }
        let category = &self.input[cat_start..self.pos];
        let valid_category = category.chars().next().is_some_and(char::is_uppercase)
            && category.chars().all(is_letter);
        if !valid_category {
            let end = if self.pos > cat_start {
                self.pos
            } else {
                self.peek_char()
                    .map_or(cat_start, |c| cat_start + c.len_utf8())
            };
            return Err(Error::new(
                Span::new(cat_start, end),
                ErrorKind::InvalidTailCategory(category.to_owned()),
            ));
        }
        let category_span = Span::new(cat_start, self.pos);

        if !self.rest().starts_with(": ") {
            let end = self
                .peek_char()
                .map_or(self.pos, |c| self.pos + c.len_utf8());
            return Err(Error::new(
                Span::new(self.pos, end),
                ErrorKind::MissingTailColon,
            ));
        }
        self.pos += 2;

        let (parts, sub_tail) = self.sequence(stop_at_gt, Some(TailContext::Direct))?;
        debug_assert!(sub_tail.is_none());
        if parts.is_empty() {
            return Err(Error::new(Span::new(start, self.pos), ErrorKind::EmptyTail));
        }

        Ok(Tail {
            placement,
            category: category.to_owned(),
            category_span,
            parts,
            span: Span::new(start, self.pos),
        })
    }
}
