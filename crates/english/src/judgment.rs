//! The judgment set: hand-written linguistic judgments, one line per entry
//! (`judgments/README.md`), and their evaluation against a bound grammar.

use std::fmt;
use std::path::Path;

use dpsi::Ident;
use sylex_lexicon::CardScope;
use sylex_lexicon::LexemeId;
use sylex_lexicon::Lexicon;
use sylex_syntax::CompiledGrammar;
use sylex_syntax::Constituent;
use sylex_syntax::Gap;
use sylex_syntax::Parse;
use sylex_syntax::Reading;

/// A grammatical function, as CGEL names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Function {
    Subject,
    Object,
    IndirectObject,
    Complement,
    PredicativeComplement,
    Adjunct,
    Modifier,
    Determiner,
}

impl Function {
    /// Every function, in the order the grammar lists them.
    pub const ALL: [Self; 8] = [
        Self::Subject,
        Self::Object,
        Self::IndirectObject,
        Self::Complement,
        Self::PredicativeComplement,
        Self::Adjunct,
        Self::Modifier,
        Self::Determiner,
    ];

    /// The child name the grammar and the syntax engine give a dependent in
    /// this function.
    #[must_use]
    pub fn child_name(self) -> &'static str {
        match self {
            Self::Subject => "subject",
            Self::Object => "object",
            Self::IndirectObject => "indirect_object",
            Self::Complement => "complement",
            Self::PredicativeComplement => "predicative_complement",
            Self::Adjunct => "adjunct",
            Self::Modifier => "modifier",
            Self::Determiner => "determiner",
        }
    }

    fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|f| format!("{f:?}") == name)
    }
}

/// One assertion about a Constituent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assertion {
    pub category: Ident,
    pub surface: String,
    /// The 1-based occurrence of `surface` in the text, or any occurrence.
    pub occurrence: Option<usize>,
    /// The lemma or lexeme id of its head word.
    pub head: Option<String>,
    /// The function it bears, and the lemma or lexeme id heading the phrase
    /// it bears it in.
    pub function: Option<(Function, String)>,
    /// The surface of the Antecedent of its Gap.
    pub gap: Option<String>,
}

impl fmt::Display for Assertion {
    /// The assertion as an entry writes it (`judgments/README.md`), which
    /// [`parse_file`] reads back as this assertion.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.category, quote(&self.surface))?;
        if let Some(occurrence) = self.occurrence {
            write!(f, "#{occurrence}")?;
        }
        if let Some(head) = &self.head {
            write!(f, " head {}", word(head))?;
        }
        if let Some((function, of)) = &self.function {
            write!(f, " as {function:?} of {}", word(of))?;
        }
        if let Some(antecedent) = &self.gap {
            write!(f, " gap {}", quote(antecedent))?;
        }
        Ok(())
    }
}

/// `text` between double quotes, `"` escaped as `\"`.
fn quote(text: &str) -> String {
    format!("\"{}\"", text.replace('"', "\\\""))
}

/// A lemma or lexeme id as an assertion's clause takes it: bare
/// when it is one run of word characters, else quoted.
fn word(text: &str) -> String {
    let bare = !text.is_empty()
        && text
            .chars()
            .all(|c| c.is_alphanumeric() || matches!(c, '-' | '_' | ':' | '\''));
    if bare { text.to_owned() } else { quote(text) }
}

/// Where `surface` occurs in `text` as whole words, by start: the `1` of
/// `-10` is none. An assertion's 1-based occurrence indexes this list.
#[must_use]
pub fn occurrences(text: &str, surface: &str) -> Vec<usize> {
    text.match_indices(surface)
        .filter(|(at, found)| {
            !text[..*at]
                .chars()
                .next_back()
                .is_some_and(char::is_alphanumeric)
                && !text[at + found.len()..]
                    .chars()
                    .next()
                    .is_some_and(char::is_alphanumeric)
        })
        .map(|(at, _)| at)
        .collect()
}

/// One judgment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Judgment {
    /// `file:line`, for reports.
    pub location: String,
    /// `+` (true) or `*` (false).
    pub grammatical: bool,
    /// The Category to parse as, or the grammar's root.
    pub category: Option<Ident>,
    /// The name of the card the text is read as if on (`<card=NAME>`).
    pub card: Option<String>,
    /// That card is stated to be a legendary permanent (`<legendary>`), so
    /// its name has its Shortened form [CR#201.5c].
    pub legendary: bool,
    /// The Anchor Words the text is read with in scope (`<anchor=WORD>`,
    /// once per word) [CR#614.12c].
    pub anchors: Vec<String>,
    pub text: String,
    /// Alternative groups; each must be satisfied by some one Reading.
    pub groups: Vec<Vec<Assertion>>,
    /// Assertions no Reading may satisfy.
    pub negated: Vec<Assertion>,
    pub readings: Option<usize>,
}

/// A malformed entry.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("{location}: {message}")]
pub struct SyntaxError {
    pub location: String,
    pub message: String,
}

/// Parses one judgment file. Blank lines and `#` comments are skipped.
///
/// # Errors
/// The first malformed entry.
pub fn parse_file(name: &str, text: &str) -> Result<Vec<Judgment>, SyntaxError> {
    let mut entries = Vec::new();
    for (number, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let location = format!("{name}:{}", number + 1);
        let entry = parse_entry(line, &location).map_err(|message| SyntaxError {
            location: location.clone(),
            message,
        })?;
        entries.push(entry);
    }
    Ok(entries)
}

/// Every `*.txt` judgment file in a directory, by file name.
///
/// # Errors
/// An unreadable directory or file, or a malformed entry.
///
/// # Panics
/// When a directory entry has no file name, which `read_dir` never yields.
pub fn load_dir(dir: &Path) -> Result<Vec<(String, Vec<Judgment>)>, SyntaxError> {
    let io = |e: std::io::Error| SyntaxError {
        location: dir.display().to_string(),
        message: e.to_string(),
    };
    let mut paths = Vec::new();
    for entry in std::fs::read_dir(dir).map_err(io)? {
        let path = entry.map_err(io)?.path();
        if path.extension().is_some_and(|x| x == "txt") {
            paths.push(path);
        }
    }
    paths.sort();
    paths
        .into_iter()
        .map(|path| {
            let name = path
                .file_name()
                .expect("a directory entry names a file")
                .to_string_lossy()
                .into_owned();
            let text = std::fs::read_to_string(&path).map_err(io)?;
            Ok((name.clone(), parse_file(&name, &text)?))
        })
        .collect()
}

fn parse_entry(line: &str, location: &str) -> Result<Judgment, String> {
    let mut chars = line.chars();
    let grammatical = match chars.next() {
        Some('+') => true,
        Some('*') => false,
        _ => return Err("an entry starts with `+` or `*`".to_owned()),
    };
    let rest = chars.as_str();
    let category_end = rest
        .find(|c: char| !c.is_ascii_alphabetic())
        .unwrap_or(rest.len());
    let category = (category_end > 0).then(|| Ident::new(&rest[..category_end]));
    let mut rest = rest[category_end..].trim_start();
    let mut card = None;
    if let Some(after) = rest.strip_prefix("<card=") {
        let end = after.find('>').ok_or("unclosed `<card=`")?;
        card = Some(after[..end].to_owned());
        rest = after[end + 1..].trim_start();
    }
    let mut legendary = false;
    if let Some(after) = rest.strip_prefix("<legendary>") {
        if card.is_none() {
            return Err("`<legendary>` follows the `<card=NAME>` it is about".to_owned());
        }
        legendary = true;
        rest = after.trim_start();
    }
    let mut anchors = Vec::new();
    while let Some(after) = rest.strip_prefix("<anchor=") {
        let end = after.find('>').ok_or("unclosed `<anchor=`")?;
        let word = &after[..end];
        if !sylex_lexicon::valid_anchor_word(word) {
            return Err(format!(
                "`<anchor={word}>` states no word an ability may be headed with"
            ));
        }
        anchors.push(word.to_owned());
        rest = after[end + 1..].trim_start();
    }
    let (text, assertions) = match rest.find(" => ") {
        Some(at) => (rest[..at].trim(), Some(rest[at + 4..].trim())),
        None => (rest.trim_end_matches("=>").trim(), None),
    };
    if text.is_empty() {
        return Err("no text".to_owned());
    }
    let mut entry = Judgment {
        location: location.to_owned(),
        grammatical,
        category,
        card,
        legendary,
        anchors,
        text: text.to_owned(),
        groups: Vec::new(),
        negated: Vec::new(),
        readings: None,
    };
    let Some(assertions) = assertions else {
        return Ok(entry);
    };
    if !grammatical {
        return Err("an ungrammatical entry takes no assertions".to_owned());
    }
    for group in split_outside_quotes(assertions, "||") {
        let mut positive = Vec::new();
        for item in split_outside_quotes(&group, ";") {
            let item = item.trim();
            if item.is_empty() {
                continue;
            }
            if let Some(count) = item.strip_prefix("readings ") {
                let count = count
                    .trim()
                    .parse()
                    .map_err(|_| format!("`readings` needs a number: {item}"))?;
                if entry.readings.replace(count).is_some_and(|c| c != count) {
                    return Err("two different `readings` counts".to_owned());
                }
            } else if let Some(negated) = item.strip_prefix('!') {
                entry.negated.push(parse_assertion(negated)?);
            } else {
                positive.push(parse_assertion(item)?);
            }
        }
        if !positive.is_empty() {
            entry.groups.push(positive);
        }
    }
    Ok(entry)
}

/// Splits on `separator` outside double-quoted surfaces (`\"` escapes a
/// quote inside one).
fn split_outside_quotes(text: &str, separator: &str) -> Vec<String> {
    let mut parts = vec![String::new()];
    let mut quoted = false;
    let mut rest = text;
    while let Some(c) = rest.chars().next() {
        if c == '\\' && quoted && rest[1..].starts_with('"') {
            parts.last_mut().unwrap().push_str("\\\"");
            rest = &rest[2..];
            continue;
        }
        if c == '"' {
            quoted = !quoted;
        }
        if !quoted && rest.starts_with(separator) {
            parts.push(String::new());
            rest = &rest[separator.len()..];
            continue;
        }
        parts.last_mut().unwrap().push(c);
        rest = &rest[c.len_utf8()..];
    }
    parts
}

/// Splits the clauses after a surface into words; a quoted string (a
/// surface, or a multiword lemma such as `"mana value"`) is one word.
fn words(text: &str) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    let mut rest = text.trim_start();
    while !rest.is_empty() {
        if rest.starts_with('"') {
            let (word, after) = quoted(rest)?;
            out.push(word);
            rest = after.trim_start();
        } else {
            let end = rest.find(char::is_whitespace).unwrap_or(rest.len());
            out.push(rest[..end].to_owned());
            rest = rest[end..].trim_start();
        }
    }
    Ok(out)
}

/// Reads a quoted surface at the start of `text`; returns it and the rest.
fn quoted(text: &str) -> Result<(String, &str), String> {
    let text = text.trim_start();
    let body = text
        .strip_prefix('"')
        .ok_or_else(|| format!("expected a quoted surface: {text}"))?;
    let mut surface = String::new();
    let mut chars = body.char_indices();
    while let Some((at, c)) = chars.next() {
        match c {
            '\\' if body[at + 1..].starts_with('"') => {
                surface.push('"');
                chars.next();
            }
            '"' => return Ok((surface, &body[at + 1..])),
            _ => surface.push(c),
        }
    }
    Err(format!("unclosed surface: {text}"))
}

fn parse_assertion(text: &str) -> Result<Assertion, String> {
    let text = text.trim();
    let category_end = text
        .find(|c: char| !c.is_ascii_alphabetic())
        .unwrap_or(text.len());
    if category_end == 0 {
        return Err(format!("an assertion starts with a Category: {text}"));
    }
    let category = Ident::new(&text[..category_end]);
    let (surface, mut rest) = quoted(&text[category_end..])?;
    let mut assertion = Assertion {
        category,
        surface,
        occurrence: None,
        head: None,
        function: None,
        gap: None,
    };
    if let Some(after) = rest.strip_prefix('#') {
        let end = after
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(after.len());
        assertion.occurrence = Some(
            after[..end]
                .parse()
                .map_err(|_| format!("bad occurrence: {text}"))?,
        );
        rest = &after[end..];
    }
    let mut words = words(rest)?.into_iter();
    while let Some(word) = words.next() {
        match word.as_str() {
            "head" => assertion.head = Some(words.next().ok_or("`head` needs a lemma")?),
            "as" => {
                let function = words.next().ok_or("`as` needs a function")?;
                let function = Function::parse(&function)
                    .ok_or_else(|| format!("unknown function `{function}`"))?;
                if words.next().as_deref() != Some("of") {
                    return Err(format!("expected `as FUNCTION of LEMMA`: {text}"));
                }
                assertion.function = Some((function, words.next().ok_or("`of` needs a lemma")?));
            }
            "gap" => assertion.gap = Some(words.next().ok_or("`gap` needs a surface")?),
            other => return Err(format!("unexpected `{other}` in: {text}")),
        }
    }
    Ok(assertion)
}

impl Judgment {
    /// The Card Scope the entry's text is read in
    /// (`sylex_vocabulary::stated_scope`): the name it states with
    /// `<card=NAME>` as the card's own, in its Full form and, when the entry
    /// also states `<legendary>` and the name has one, in its Shortened form
    /// [CR#201.5c]; and each Anchor Word it states with `<anchor=WORD>`
    /// [CR#614.12c]. The name, the flag and the words are inputs like the
    /// text: no card is looked up, the name need be no printed card's, and
    /// no other text confirms the words as a face's units do
    /// ([`crate::corpus::Face::scope`]).
    #[must_use]
    pub fn scope(&self, lexicon: &Lexicon) -> CardScope {
        let mut scope = match &self.card {
            Some(name) => sylex_vocabulary::stated_scope(name, self.legendary, lexicon),
            None => CardScope::default(),
        };
        for word in &self.anchors {
            scope.insert_anchor_word(word);
        }
        scope
    }
}

/// Round-trip law 2 for one Reading of `parse`: admitted, realized, and
/// found again in the parse of its realization, with the same names in
/// scope. `scope` is the scope `parse` was analyzed with.
///
/// A realization that is the parsed text itself would analyze and parse
/// exactly as `parse` did, so the Reading is looked up in `parse`; only
/// other text is parsed again, as the same Category.
///
/// # Errors
/// Which step failed.
pub fn law_two(
    grammar: &CompiledGrammar,
    lexicon: &Lexicon,
    parse: &Parse<'_>,
    reading: &Reading,
    scope: &CardScope,
) -> Result<(), String> {
    parse
        .admit(reading, lexicon)
        .map_err(|e| format!("admission: {e}"))?;
    let text = grammar
        .realize(reading, lexicon)
        .map_err(|e| format!("realization: {e}"))?;
    let found = if text == parse.text() {
        parse.contains(reading)
    } else {
        parse
            .reparse(&lexicon.analyze_source_scoped(&text, None, scope))
            .contains(reading)
    };
    if found {
        Ok(())
    } else {
        Err(format!(
            "round-trip law 2: a Reading of {text:?} does not reparse"
        ))
    }
}

/// What evaluating an entry found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Holds,
    Fails(String),
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Holds => f.write_str("holds"),
            Self::Fails(why) => f.write_str(why),
        }
    }
}

/// One Reading, located: its text and Constituents.
pub struct Located {
    pub text: String,
    pub constituents: Vec<Constituent>,
}

/// Evaluates entries against one bound grammar and its lexicon.
pub struct Judge<'g> {
    pub grammar: &'g CompiledGrammar,
    pub lexicon: &'g Lexicon,
}

impl Judge<'_> {
    /// Parses an entry's text as it asks, with the name of the card it states
    /// in scope.
    ///
    /// # Errors
    /// A Category the grammar lacks.
    pub fn parse(&self, entry: &Judgment) -> Result<Parse<'_>, String> {
        self.parse_scoped(entry, &entry.scope(self.lexicon))
    }

    /// Parses an entry's text as it asks, with `scope` in scope.
    ///
    /// # Errors
    /// A Category the grammar lacks.
    pub fn parse_scoped(&self, entry: &Judgment, scope: &CardScope) -> Result<Parse<'_>, String> {
        let analyzed = self.lexicon.analyze_source_scoped(
            &sylex_upstream::oracle_text::strip_reminder_text(&entry.text),
            None,
            scope,
        );
        match &entry.category {
            Some(category) => self
                .grammar
                .parse_category(&analyzed, category)
                .ok_or_else(|| format!("the grammar has no Category `{category}`")),
            None => Ok(self.grammar.parse_analyzed(&analyzed)),
        }
    }

    /// Whether the entry holds, with both round-trip laws checked on every
    /// Reading.
    #[must_use]
    pub fn evaluate(&self, entry: &Judgment) -> Outcome {
        self.evaluate_scoped(entry, &entry.scope(self.lexicon))
    }

    /// [`Judge::evaluate`] with `scope` in scope.
    #[must_use]
    pub fn evaluate_scoped(&self, entry: &Judgment, scope: &CardScope) -> Outcome {
        let parse = match self.parse_scoped(entry, scope) {
            Ok(parse) => parse,
            Err(why) => return Outcome::Fails(why),
        };
        let count = parse.derivation_count();
        // An undeclared word (an off-card Shortened Name, a misspelled type)
        // leaves no Reading, which is what an ungrammatical entry claims.
        if !parse.vocabulary_gaps.is_empty() {
            return if entry.grammatical {
                Outcome::Fails(format!("vocabulary gaps at {:?}", parse.vocabulary_gaps))
            } else {
                Outcome::Holds
            };
        }
        if !entry.grammatical {
            return if count == 0 {
                Outcome::Holds
            } else {
                Outcome::Fails(format!("{count} derivations of an ungrammatical text"))
            };
        }
        if count == 0 {
            return Outcome::Fails("no Reading".to_owned());
        }
        if let Err(fault) = parse.verify_realization(self.lexicon) {
            return Outcome::Fails(format!("round-trip law 1: {fault}"));
        }
        let readings = parse.readings();
        let mut located = Vec::new();
        for reading in readings {
            if let Err(why) = law_two(self.grammar, self.lexicon, &parse, &reading, scope) {
                return Outcome::Fails(why);
            }
            match self.grammar.constituents(&reading, self.lexicon) {
                Ok((text, constituents)) => located.push(Located { text, constituents }),
                Err(e) => return Outcome::Fails(format!("realization: {e}")),
            }
        }
        if let Some(n) = entry.readings
            && n != located.len()
        {
            return Outcome::Fails(format!("{} Readings, not {n}", located.len()));
        }
        for group in &entry.groups {
            if !located
                .iter()
                .any(|r| group.iter().all(|a| self.holds(a, r)))
            {
                return Outcome::Fails(format!("no Reading satisfies {group:?}"));
            }
        }
        for assertion in &entry.negated {
            if located.iter().any(|r| self.holds(assertion, r)) {
                return Outcome::Fails(format!("a Reading satisfies !{assertion:?}"));
            }
        }
        Outcome::Holds
    }

    /// Whether some entry Reading is endorsed: it satisfies some group (or
    /// the entry has none) and no negated assertion.
    #[must_use]
    pub fn endorses(&self, entry: &Judgment, reading: &Located) -> bool {
        (entry.groups.is_empty()
            || entry
                .groups
                .iter()
                .any(|g| g.iter().all(|a| self.holds(a, reading))))
            && !entry.negated.iter().any(|a| self.holds(a, reading))
    }

    /// Whether some Constituent of the Reading satisfies the assertion.
    #[must_use]
    pub fn holds(&self, assertion: &Assertion, reading: &Located) -> bool {
        let text = &reading.text;
        let wanted_start = assertion.occurrence.map(|n| {
            occurrences(text, &assertion.surface)
                .get(n.saturating_sub(1))
                .copied()
        });
        reading.constituents.iter().enumerate().any(|(index, c)| {
            c.category == assertion.category
                && text.get(c.span.clone()) == Some(assertion.surface.as_str())
                && wanted_start.is_none_or(|start| start == Some(c.span.start))
                && assertion
                    .head
                    .as_ref()
                    .is_none_or(|h| self.names(c.head.as_ref(), h))
                && assertion.function.as_ref().is_none_or(|(function, of)| {
                    let in_phrase =
                        |p: usize| self.names(reading.constituents[p].head.as_ref(), of);
                    (bears(c.function, function.child_name()) && c.parent.is_some_and(in_phrase))
                        || c.also
                            .iter()
                            .any(|(f, p)| *f == function.child_name() && in_phrase(*p))
                })
                && assertion
                    .gap
                    .as_ref()
                    .is_none_or(|antecedent| has_antecedent(reading, index, antecedent))
        })
    }

    /// Whether `lexeme` is the word `name` (a lemma or a lexeme id).
    fn names(&self, lexeme: Option<&LexemeId>, name: &str) -> bool {
        lexeme.is_some_and(|id| {
            id.as_str() == name
                || self
                    .lexicon
                    .lexemes()
                    .get(id)
                    .is_some_and(|l| l.lemma.eq_ignore_ascii_case(name))
        })
    }
}

/// Whether Constituent `index` contains a Gap whose Antecedent is spelled
/// `antecedent`: a Gap inside it or one it binds (a relative clause binds the
/// Gap of its clause). The Gap is bound at its lowest gapless ancestor `A`:
/// when the gapped child of `A` is its head (Right Node Raising), the
/// Antecedent is a sibling of that child; otherwise `A` modifies a
/// head, which is the Antecedent (a relative clause and its nominal), or a
/// Verb Phrase whose Object's nominal is the Antecedent (a postposed relative
/// clause).
fn has_antecedent(reading: &Located, index: usize, antecedent: &str) -> bool {
    let cs = &reading.constituents;
    let mut child = index;
    let binder = if cs[index].gap == Gap::None {
        let Some(gapped) =
            (0..cs.len()).find(|&c| cs[c].parent == Some(index) && cs[c].gap != Gap::None)
        else {
            return false;
        };
        child = gapped;
        index
    } else {
        let Some(binder) = std::iter::from_fn(|| {
            let parent = cs[child].parent?;
            if cs[parent].gap == Gap::None {
                return Some(Some(parent));
            }
            child = parent;
            Some(None)
        })
        .flatten()
        .next() else {
            return false;
        };
        binder
    };
    let spelled = |c: &Constituent| reading.text.get(c.span.clone()) == Some(antecedent);
    let children = |of: usize| cs.iter().filter(move |c| c.parent == Some(of));
    let child_of = |of: usize, function: &str| {
        (0..cs.len()).find(|&c| cs[c].parent == Some(of) && bears(cs[c].function, function))
    };
    if bears(cs[child].function, "head") {
        return children(binder).any(|c| !bears(c.function, "head") && spelled(c));
    }
    let Some(host) = cs[binder].parent else {
        return false;
    };
    if children(host).any(|c| bears(c.function, "head") && spelled(c)) {
        return true;
    }
    if cs[host].category != "VerbPhrase" {
        return false;
    }
    // The Object of the host's head Verb Phrase, down to its head nominal.
    let mut verb = host;
    let object = loop {
        if let Some(object) = child_of(verb, "object") {
            break object;
        }
        let Some(head) = child_of(verb, "head") else {
            return false;
        };
        verb = head;
    };
    let mut nominal = object;
    while cs[nominal].category == "NounPhrase" {
        let Some(head) = child_of(nominal, "head") else {
            return false;
        };
        nominal = head;
    }
    spelled(&cs[nominal])
}

/// Whether a Constituent bearing `function` (none at the root) bears `name`.
fn bears(function: Option<Ident>, name: &str) -> bool {
    function.is_some_and(|f| f == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An assertion displays as an entry writes it: each form the README
    /// lists (a quoted multiword lemma, a lexeme id, an escaped quotation
    /// mark, an occurrence, a gap) reads back as the assertion displayed.
    #[test]
    fn an_assertion_displays_as_the_entry_writes_it() {
        for written in [
            "NounPhrase \"target creature\"",
            "NounPhrase \"target creature\" as Object of destroy",
            "PrepositionPhrase \"for a card\"#2 head search",
            "NounPhrase \"mana value\" head \"mana value\" as Modifier of action:counter",
            "Clause \"you control\" gap \"permanents\"",
            "NounPhrase \"say \\\"a\\\"\" as Subject of \"mana value\"",
        ] {
            let entries = parse_file("test", &format!("+ text => {written}")).unwrap();
            let assertion = &entries[0].groups[0][0];
            assert_eq!(assertion.to_string(), written);
        }
    }
}
