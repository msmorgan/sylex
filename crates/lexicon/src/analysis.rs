use std::collections::BTreeMap;
use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

use crate::Binding;
use crate::Capitalization;
use crate::Category;
use crate::LetterCase;
use crate::Lexeme;
use crate::LexicalReading;
use crate::LexicalValue;
use crate::SurfaceStructure;
use crate::WordForm;
use crate::morphology;
use crate::numeral::Numeral;
use crate::numeral::NumeralCodec;

#[derive(Debug, thiserror::Error)]
pub enum LexicalError {
    #[error("invalid lexical declaration {lexeme}: {reason}")]
    Declaration {
        lexeme: crate::LexemeId,
        reason: String,
    },
    #[error("lexical value is not licensed by this lexicon")]
    UnlicensedValue,
}

/// What an italic run written at the head of an ability analyses as
/// [CR#207.2c,207.2d].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItalicHead {
    /// The run is spelled by a declared Ability Word [CR#207.2c].
    AbilityWord(LexicalValue),
    /// A declared Keyword Ability spells the run [CR#702.1]. It is neither an
    /// Ability Word nor a Flavor Word.
    Keyword(LexicalValue),
    /// No Ability Word or keyword declaration spells the run. Flavor words are
    /// listed nowhere [CR#207.2d] — each is tailored to the one ability it
    /// heads — so there is no inventory to consult and the run itself is
    /// the label.
    FlavorWord { label: String },
}

/// Half-open occurrences in the lossless Unicode-scalar token sequence.
/// Scalar boundaries allow bound lexical forms without preselecting a word
/// split.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct LexicalMatch {
    pub start: usize,
    pub end: usize,
    pub reading: LexicalReading,
    /// The case its first letter is written in, where its reading takes an
    /// initial capital that its declared spelling lacks; `None` where the
    /// two spellings are one, as they are for a Lexeme spelled exactly.
    pub case: Option<LetterCase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyzedText {
    pub raw: String,
    /// None means the raw input was analyzed directly.
    pub normalized: Option<String>,
    pub tokens: Vec<char>,
    pub matches: Vec<LexicalMatch>,
    /// The open words of the text: each spelling that has the shape of an
    /// Anchor Word or a Vote Option, which the scope does not introduce.
    /// They are no lexical matches. A text introduces a word of either kind
    /// at any word (*votes for sickness or psychosis*), so every spelling of
    /// the shape is offered to the parse, which reads one only at a
    /// position that introduces it or, in the same Reading, uses it
    /// (`docs/design/lexicon.md` §2); one no parse reads is nothing.
    pub open_words: Vec<LexicalMatch>,
    /// The Card Scope the text was analyzed in: the names of the face whose
    /// text it is, and the words its other lines introduce and use.
    pub scope: crate::CardScope,
}

impl AnalyzedText {
    #[must_use]
    pub fn text(&self) -> &str {
        self.normalized.as_deref().unwrap_or(&self.raw)
    }

    /// Byte locations are derived only when a consumer requests them.
    #[must_use]
    pub fn byte_range(&self, start: usize, end: usize) -> Option<std::ops::Range<usize>> {
        if start > end || end > self.tokens.len() {
            return None;
        }
        let begin: usize = self.tokens[..start].iter().map(|c| c.len_utf8()).sum();
        let length: usize = self.tokens[start..end].iter().map(|c| c.len_utf8()).sum();
        Some(begin..begin + length)
    }

    /// Orthographic words are an inventory view, not a context-sensitive
    /// tokenizer.
    #[must_use]
    pub fn words(&self) -> Vec<std::ops::Range<usize>> {
        let mut result = Vec::new();
        let mut start = 0;
        while start < self.tokens.len() {
            if !word_character(self.tokens[start]) {
                start += 1;
                continue;
            }
            let mut end = start + 1;
            while end < self.tokens.len()
                && (word_character(self.tokens[end])
                    || self.tokens[end] == '\''
                        && end + 1 < self.tokens.len()
                        && word_character(self.tokens[end + 1]))
            {
                end += 1;
            }
            result.push(start..end);
            start = end;
        }
        result
    }

    /// The words no declared Lexeme spells. A title run or an open word
    /// alone covers none: it is an Introduced Name, an Anchor Word or a Vote
    /// Option only where a parse takes it as one
    /// ([`AnalyzedText::unknown_words_given`]).
    #[must_use]
    pub fn unknown_words(&self) -> Vec<std::ops::Range<usize>> {
        self.unknown_words_given(|_| false)
    }

    /// The words that neither a declared Lexeme spells nor an Introduced
    /// Name, Anchor Word or Vote Option that `introduced` holds covers.
    #[must_use]
    pub fn unknown_words_given(
        &self,
        introduced: impl Fn(&LexicalMatch) -> bool,
    ) -> Vec<std::ops::Range<usize>> {
        let known = |found: &LexicalMatch| match found.reading {
            LexicalReading::IntroducedName { .. } => introduced(found),
            // A match is a word the scope introduces; an open word is one
            // only where the parse takes it.
            LexicalReading::AnchorWord { .. } | LexicalReading::VoteOption { .. } => {
                self.scope.introduces(&found.reading) || introduced(found)
            }
            LexicalReading::Word(_)
            | LexicalReading::OwnName { .. }
            | LexicalReading::FlavorWord { .. }
            | LexicalReading::SetName { .. }
            | LexicalReading::KindName { .. }
            | LexicalReading::Numeral { .. } => true,
        };
        self.words()
            .into_iter()
            .filter(|word| !self.covered_by(word.clone(), known))
            .collect()
    }

    /// Whether adjacent lexical occurrences cover a word, allowing a containing
    /// multiword lexeme. Crossing overlaps alone do not constitute coverage.
    #[must_use]
    pub fn is_covered(
        &self,
        range: std::ops::Range<usize>,
        include: impl Fn(&LexicalReading) -> bool,
    ) -> bool {
        self.covered_by(range, |found| include(&found.reading))
    }

    fn covered_by(
        &self,
        range: std::ops::Range<usize>,
        include: impl Fn(&LexicalMatch) -> bool,
    ) -> bool {
        if range.start >= range.end || range.end > self.tokens.len() {
            return false;
        }
        let mut edges = vec![Vec::new(); range.len()];
        for found in self.matches.iter().chain(&self.open_words) {
            if found.start < range.end && found.end > range.start && include(found) {
                let start = found.start.max(range.start) - range.start;
                let end = found.end.min(range.end) - range.start;
                edges[start].push(end);
            }
        }
        let mut reachable = vec![false; range.len() + 1];
        reachable[0] = true;
        for (start, targets) in edges.iter().enumerate() {
            if reachable[start] {
                for end in targets {
                    reachable[*end] = true;
                }
            }
        }
        reachable[range.len()]
    }
}

#[derive(Debug, Default)]
struct TrieNode {
    children: Vec<(char, usize)>,
    values: Vec<(LexicalValue, Binding, Option<LetterCase>)>,
}

impl TrieNode {
    fn child(&self, ch: char) -> Option<usize> {
        let at = self.children.binary_search_by_key(&ch, |&(c, _)| c).ok()?;
        Some(self.children[at].1)
    }
}

/// An immutable index built from declared forms and shared realization rules.
/// Loading and inspecting it needs no construction grammar or compiler.
#[derive(Debug)]
pub struct Lexicon {
    lexemes: BTreeMap<crate::LexemeId, Lexeme>,
    surfaces: BTreeMap<LexicalValue, String>,
    /// The capitalized spelling of each value whose Lexeme takes an initial
    /// capital its declared spelling lacks.
    capitals: BTreeMap<LexicalValue, String>,
    edges: BTreeMap<LexicalValue, crate::Edges>,
    trie: Vec<TrieNode>,
}

impl Lexicon {
    /// Validate declarations and freeze both analysis and realization indexes.
    ///
    /// # Errors
    /// Rejects duplicate identities and slots, empty forms, inapplicable
    /// features and incomplete agreement paradigms.
    pub fn new(lexemes: impl IntoIterator<Item = Lexeme>) -> Result<Self, LexicalError> {
        let mut result = Self {
            lexemes: BTreeMap::new(),
            surfaces: BTreeMap::new(),
            capitals: BTreeMap::new(),
            edges: BTreeMap::new(),
            trie: vec![TrieNode::default()],
        };
        for lexeme in lexemes {
            let error = |reason: &str| LexicalError::Declaration {
                lexeme: lexeme.id,
                reason: reason.to_owned(),
            };
            if lexeme.id.is_empty()
                || lexeme.lemma.trim().is_empty()
                || lexeme.forms.is_empty()
                || lexeme.source.owner.is_empty()
                || lexeme.source.path.is_empty()
            {
                return Err(error("identity, lemma, forms and source must be nonempty"));
            }
            if result.lexemes.contains_key(&lexeme.id) {
                return Err(error("duplicate identity"));
            }
            if lexeme.surface_structure == SurfaceStructure::Opaque
                && !matches!(
                    lexeme.category,
                    Category::Catalog | Category::Keyword | Category::Symbol | Category::Numeral
                )
            {
                return Err(error(
                    "opaque spelling requires a catalog or notation category",
                ));
            }
            if !valid_surface(&lexeme.lemma, lexeme.surface_structure, lexeme.binding) {
                return Err(error("lemma violates its declared word boundaries"));
            }
            let mut slots = BTreeSet::new();
            // Whether a declared Onset is the one every spelling has anyway.
            let mut redundant_onset = lexeme.onset.is_some();
            let prefix_hyphenation = match (
                lexeme.category.hosts_prefix(),
                lexeme.properties.prefix_hyphenation,
            ) {
                (true, Some(crate::Hyphenation::Solid)) => {
                    return Err(error(
                        "a declared prefix hyphenation its host takes by default",
                    ));
                }
                (true, declared) => Some(declared.unwrap_or(crate::Hyphenation::Solid)),
                (false, None) => None,
                (false, Some(_)) => {
                    return Err(error(
                        "prefix hyphenation on a Lexeme whose Category hosts no prefix",
                    ));
                }
            };
            if let Some(defect) = morphology::modal_defect(&lexeme) {
                return Err(error(defect));
            }
            let plurals: BTreeSet<String> = lexeme
                .forms
                .iter()
                .filter(|slot| slot.form == WordForm::Plural)
                .map(|slot| spelling(&lexeme, slot))
                .collect();
            for declaration in &lexeme.forms {
                if !slots.insert((declaration.form, &declaration.features)) {
                    return Err(error("duplicate inflection slot"));
                }
                if !morphology::applicable(
                    lexeme.category,
                    lexeme.properties.modal,
                    declaration.form,
                    &declaration.features,
                ) {
                    return Err(error("inapplicable form or feature bundle"));
                }
                if let Some(reason) = missing_explicit_spelling(declaration) {
                    return Err(error(reason));
                }
                let surface = spelling(&lexeme, declaration);
                if surface.trim() != surface || surface.is_empty() {
                    return Err(error("empty or padded spelling"));
                }
                if !valid_surface(&surface, lexeme.surface_structure, lexeme.binding) {
                    return Err(error("spelling violates its declared word boundaries"));
                }
                let default_onset = crate::pronunciation::default_onset(&surface);
                redundant_onset &= lexeme
                    .onset
                    .is_none_or(|onset| default_onset == Some(onset));
                let value = LexicalValue {
                    lexeme: lexeme.id,
                    form: declaration.form,
                    features: declaration.features.clone(),
                };
                let edges = crate::Edges::new(
                    &declaration.features,
                    crate::AgreementProperties {
                        onset: lexeme.onset.or(default_onset),
                        prefix_hyphenation,
                        genitive_host: genitive_host(declaration.form, &surface, &plurals),
                    },
                );
                let initial = initial_surface(&surface);
                let capitalized =
                    lexeme.capitalization == Capitalization::Initial && initial != surface;
                result.edges.insert(value.clone(), edges);
                result.insert(
                    &value,
                    &surface,
                    lexeme.binding,
                    capitalized.then_some(LetterCase::Lower),
                );
                if capitalized {
                    result.insert(&value, &initial, lexeme.binding, Some(LetterCase::Capital));
                    result.capitals.insert(value.clone(), initial);
                }
                result.surfaces.insert(value, surface);
            }
            if let Some(reason) = card_name_defect(&lexeme).or_else(|| agreement_defect(&lexeme)) {
                return Err(error(reason));
            }
            if redundant_onset {
                return Err(error("a declared Onset its spellings have by default"));
            }
            if lexeme.set_name_host && lexeme.category != Category::Noun {
                return Err(error("a Set Name premodifies only a noun"));
            }
            if lexeme.kind_name_host && lexeme.category != Category::Noun {
                return Err(error("a Kind Name premodifies only a noun"));
            }
            result.lexemes.insert(lexeme.id, lexeme);
        }
        Ok(result)
    }

    fn insert(
        &mut self,
        value: &LexicalValue,
        surface: &str,
        binding: Binding,
        case: Option<LetterCase>,
    ) {
        let mut node = 0;
        for ch in surface.chars() {
            node = match self.trie[node]
                .children
                .binary_search_by_key(&ch, |&(c, _)| c)
            {
                Ok(at) => self.trie[node].children[at].1,
                Err(at) => {
                    let next = self.trie.len();
                    self.trie.push(TrieNode::default());
                    self.trie[node].children.insert(at, (ch, next));
                    next
                }
            };
        }
        self.trie[node].values.push((value.clone(), binding, case));
    }

    #[must_use]
    pub fn lexemes(&self) -> &BTreeMap<crate::LexemeId, Lexeme> {
        &self.lexemes
    }

    /// Enumerates licensed values independently of any analyzed source.
    pub fn values(&self) -> impl Iterator<Item = &LexicalValue> {
        self.surfaces.keys()
    }

    /// What Adjacency Agreement reads at the edges of one licensed
    /// realization ([`crate::RightEdge::agrees`]).
    ///
    /// # Errors
    /// Rejects undeclared word values and noncanonical numeral values.
    pub fn edges(&self, reading: &LexicalReading) -> Result<crate::Edges, LexicalError> {
        match reading {
            LexicalReading::Word(value) => self
                .edges
                .get(value)
                .copied()
                .ok_or(LexicalError::UnlicensedValue),
            LexicalReading::OwnName { .. }
            | LexicalReading::IntroducedName { .. }
            | LexicalReading::Numeral { .. }
            | LexicalReading::FlavorWord { .. }
            | LexicalReading::AnchorWord { .. }
            | LexicalReading::VoteOption { .. }
            | LexicalReading::SetName { .. }
            | LexicalReading::KindName { .. } => {
                self.realize(reading)?;
                crate::Edges::non_word(reading).ok_or(LexicalError::UnlicensedValue)
            }
        }
    }

    /// The spelling of a reading in its declared case: where a position
    /// calls for no capital ([`LetterCase::Lower`]).
    ///
    /// # Errors
    /// Rejects values not admitted by this lexicon, including lossy numeral
    /// values.
    pub fn realize(&self, reading: &LexicalReading) -> Result<String, LexicalError> {
        self.realize_in(reading, LetterCase::Lower)
    }

    /// The spelling of a reading where a position calls for `case`: with an
    /// initial capital only where `case` is [`LetterCase::Capital`] and the
    /// reading takes one ([`Lexicon::takes_capital`]). A word spelled
    /// exactly keeps its spelling in every position.
    ///
    /// # Errors
    /// As [`Lexicon::realize`].
    pub fn realize_in(
        &self,
        reading: &LexicalReading,
        case: LetterCase,
    ) -> Result<String, LexicalError> {
        match reading {
            LexicalReading::FlavorWord { label } => {
                // A registered label is never a Flavor Word, so neither
                // analysis nor realization admits one. Realization has no
                // card scope, so only a label registered everywhere refuses;
                // [`Lexicon::stands_in`] refuses the rest in their scope.
                if !valid_flavor_label(label)
                    || !self.title_cased(label)
                    || self
                        .registered_label(label, Some(&crate::CardScope::default()))
                        .is_some()
                {
                    return Err(LexicalError::UnlicensedValue);
                }
                Ok(format!("{label} —"))
            }
            // An own name is spelled exactly, from its name and form alone.
            LexicalReading::OwnName { name, form } => crate::own_name_spelling(name, *form)
                .filter(|spelling| valid_own_name(spelling))
                .ok_or(LexicalError::UnlicensedValue),
            LexicalReading::SetName { name } => {
                if !crate::set_name::valid_set_name(name) {
                    return Err(LexicalError::UnlicensedValue);
                }
                Ok(name.clone())
            }
            LexicalReading::KindName { kind, .. } => {
                if !crate::kind_name::valid_kind_name(kind) {
                    return Err(LexicalError::UnlicensedValue);
                }
                Ok(kind.clone())
            }
            // An Anchor Word is spelled exactly, as the label it heads its
            // ability with.
            LexicalReading::AnchorWord { word } => {
                if !valid_anchor_word(word) {
                    return Err(LexicalError::UnlicensedValue);
                }
                Ok(word.clone())
            }
            // A Vote Option is spelled exactly, as the word a vote offers.
            LexicalReading::VoteOption { word } => {
                if !crate::valid_vote_option(word) {
                    return Err(LexicalError::UnlicensedValue);
                }
                Ok(word.clone())
            }
            // An Introduced Name is spelled exactly, as the title run it is,
            // and is never a card's name.
            LexicalReading::IntroducedName { name } => {
                let chars: Vec<char> = name.chars().collect();
                if !crate::introduced::valid_introduced_name(name) || self.spells_card_name(&chars)
                {
                    return Err(LexicalError::UnlicensedValue);
                }
                Ok(name.clone())
            }
            LexicalReading::Word(value) => {
                let declared = self
                    .surfaces
                    .get(value)
                    .ok_or(LexicalError::UnlicensedValue)?;
                Ok(match (case, self.capitals.get(value)) {
                    (LetterCase::Capital, Some(capital)) => capital.clone(),
                    // A Lexeme spelled exactly, or whose declared spelling
                    // already opens with a capital, has one spelling.
                    (LetterCase::Capital, None) | (LetterCase::Lower, _) => declared.clone(),
                })
            }
            LexicalReading::Numeral { value, notation } => {
                if notation.canonical_notation(*value) != *notation {
                    return Err(LexicalError::UnlicensedValue);
                }
                let text = notation
                    .try_format(*value)
                    .map_err(|_| LexicalError::UnlicensedValue)?;
                Ok(match case {
                    LetterCase::Lower => text,
                    LetterCase::Capital => initial_surface(&text),
                })
            }
        }
    }

    /// The spelling a lexical match records: `case` is the case its first
    /// letter is written in, `None` for a reading with one spelling
    /// ([`LexicalMatch::case`]).
    ///
    /// # Errors
    /// As [`Lexicon::realize`], and a `case` that is `None` for a reading with
    /// two spellings, or names a case for a reading with one.
    pub fn realize_written(
        &self,
        reading: &LexicalReading,
        case: Option<LetterCase>,
    ) -> Result<String, LexicalError> {
        match (case, self.takes_capital(reading)?) {
            (Some(case), true) => self.realize_in(reading, case),
            (None, false) => self.realize(reading),
            (Some(_), false) | (None, true) => Err(LexicalError::UnlicensedValue),
        }
    }

    /// Whether a licensed reading takes an initial capital that its declared
    /// spelling lacks: a word whose Lexeme takes one
    /// ([`Capitalization::Initial`]) and a numeral spelled in words. Only
    /// such a reading's spelling depends on its position.
    ///
    /// # Errors
    /// As [`Lexicon::realize`].
    pub fn takes_capital(&self, reading: &LexicalReading) -> Result<bool, LexicalError> {
        let declared = self.realize(reading)?;
        Ok(self.realize_in(reading, LetterCase::Capital)? != declared)
    }

    /// Analyzes an italic run written at the head of an ability
    /// [CR#207.2c,207.2d].
    ///
    /// A run no registered label spells is not a vocabulary gap here: it is
    /// a flavor word, and the label is the run verbatim. Nothing
    /// enumerates flavor words, so the lexicon reports it as an open slot
    /// rather than an unknown word, as it does a Set Name.
    #[must_use]
    pub fn analyze_italic_head(&self, run: &str) -> ItalicHead {
        match self.registered_label(run, None) {
            Some(value) => match self.label_kind(&value) {
                Some("AbilityWord") => ItalicHead::AbilityWord(value),
                _ => ItalicHead::Keyword(value),
            },
            None => ItalicHead::FlavorWord {
                label: run.to_owned(),
            },
        }
    }

    /// The `LabelKind` a registered label declares: `AbilityWord`; a Keyword
    /// Ability declares none.
    fn label_kind(&self, value: &LexicalValue) -> Option<&str> {
        self.lexemes[&value.lexeme]
            .properties
            .features
            .get("LabelKind")
            .map(dpsi::Ident::as_str)
    }

    /// The registered label that spells all of `label`: a Keyword-category
    /// Lexeme, which is an Ability Word [CR#207.2c] or a Keyword Ability
    /// [CR#702.1]. A declared label kind is
    /// preferred to a Keyword Ability. A registered label is never also an
    /// open Flavor Word. With a `scope`, a label registers only where it
    /// stands ([`Lexicon::stands_in`]); without one, every label registers.
    fn registered_label(
        &self,
        label: &str,
        scope: Option<&crate::CardScope>,
    ) -> Option<LexicalValue> {
        let mut node = 0;
        for ch in label.chars() {
            node = self.trie[node].child(ch)?;
        }
        let mut registered: Vec<&LexicalValue> = self.trie[node]
            .values
            .iter()
            .map(|(value, _, _)| value)
            .filter(|value| scope.is_none_or(|scope| self.word_stands_in(value, scope)))
            .filter(|value| self.lexemes[&value.lexeme].category == Category::Keyword)
            .collect();
        registered.sort_by_key(|value| self.label_kind(value).is_none());
        registered.first().map(|value| (*value).clone())
    }

    /// Whether a label has the title casing of a Flavor Word: every word that
    /// begins in lowercase is a function word, which title casing leaves
    /// lowercase (`docs/oracle-style-guide.md` §14, Ability words and flavor
    /// words: "natural title casing"; *Sanctified Rules of Combat*, *Bring
    /// it Down!*). A clause before an em dash (*Each opponent faces a
    /// villainous choice — ...*) is no Flavor Word.
    fn title_cased(&self, label: &str) -> bool {
        label.split_whitespace().all(|word| {
            let word = word.trim_end_matches(|ch: char| !ch.is_alphanumeric());
            !word.starts_with(char::is_lowercase) || self.is_function_word(word)
        })
    }

    /// Whether a word spells a closed-class Lexeme that title casing leaves
    /// lowercase: a determinative, pronoun, preposition, coordinator or
    /// subordinator.
    fn is_function_word(&self, word: &str) -> bool {
        let mut node = 0;
        for ch in word.chars() {
            match self.trie[node].child(ch) {
                Some(next) => node = next,
                None => return false,
            }
        }
        self.trie[node]
            .values
            .iter()
            .any(|(value, _, _)| self.lexemes[&value.lexeme].category.is_function_word())
    }

    /// Whether a lexical value stands in the text of a face whose card names
    /// are `scope`: analysis finds it there, and admission admits it there.
    ///
    /// - An own name stands only where the scope holds that form of that name
    ///   [CR#201.5,201.5c].
    /// - A card name's Lexeme, another card's name, stands everywhere but where
    ///   the scope holds its name, which is the card's own there.
    /// - An Anchor Word [CR#614.12c] and a Vote Option [CR#701.38b] stand
    ///   everywhere, as open words: the grammar admits one only where its text
    ///   introduces it or uses a word the face introduces
    ///   (`AnalyzedText::open_words`).
    /// - An Introduced Name stands everywhere but where the scope holds its
    ///   spelling as the card's full name.
    /// - A Flavor Word stands only where no label registered there spells it
    ///   and the scope introduces no Anchor Word of its spelling.
    /// - Every other value, a Set Name included, stands everywhere.
    ///
    /// It checks scope alone: whether the lexicon licenses the value is
    /// [`Lexicon::realize`]'s question, which admission asks first.
    ///
    /// # Panics
    /// On a word value of no Lexeme of this lexicon.
    #[must_use]
    pub fn stands_in(&self, reading: &LexicalReading, scope: &crate::CardScope) -> bool {
        match reading {
            LexicalReading::Word(value) => self.word_stands_in(value, scope),
            LexicalReading::OwnName { name, form } => scope.holds(name, *form),
            LexicalReading::IntroducedName { name } => !scope.holds(name, crate::NameForm::Full),
            LexicalReading::FlavorWord { label } => {
                self.registered_label(label, Some(scope)).is_none()
                    && !scope.holds_anchor_word(label)
            }
            LexicalReading::AnchorWord { .. }
            | LexicalReading::VoteOption { .. }
            | LexicalReading::Numeral { .. }
            | LexicalReading::SetName { .. }
            | LexicalReading::KindName { .. } => true,
        }
    }

    fn word_stands_in(&self, value: &LexicalValue, scope: &crate::CardScope) -> bool {
        let lexeme = self
            .lexemes
            .get(&value.lexeme)
            .expect("a word value asked about scope is a Lexeme of this lexicon");
        !lexeme.card_name || !scope.holds_name(&lexeme.lemma)
    }

    /// Whether a card name's Lexeme spells all of `spelling`.
    fn spells_card_name(&self, spelling: &[char]) -> bool {
        let mut node = 0;
        for ch in spelling {
            match self.trie[node].child(*ch) {
                Some(next) => node = next,
                None => return false,
            }
        }
        self.names_card(node)
    }

    /// Whether the trie node is the end of a card name's spelling.
    fn names_card(&self, node: usize) -> bool {
        self.trie[node]
            .values
            .iter()
            .any(|(value, _, _)| self.lexemes[&value.lexeme].card_name)
    }

    /// The title run at each capitalized word of `tokens`, as an Introduced
    /// Name ([`crate::introduced`]): the longest run there, unless a card's
    /// full name spells it, or one written at that position runs past it
    /// (*an artifact named The Mightstone and Weakstone*; *a creature named
    /// Fang, Fearless l'Cie*). No run starts inside such a card name. A card's
    /// full name is one a card name's Lexeme spells or the scope holds; a
    /// Shortened Name is none, so it never ends a run at its comma
    /// (*Icingdeath, Frost Tongue* on Icingdeath, Frost Tyrant).
    fn introduced_name_matches(
        &self,
        tokens: &[char],
        scope: &crate::CardScope,
        matches: &mut Vec<LexicalMatch>,
    ) {
        let own: Vec<Vec<char>> = scope
            .forms()
            .filter(|(_, form)| *form == crate::NameForm::Full)
            .map(|(name, _)| name.chars().collect())
            .collect();
        let card_name = |run: &[char]| {
            self.spells_card_name(run) || own.iter().any(|name| name.as_slice() == run)
        };
        // The end of the last card name written out so far: no run starts
        // inside one.
        let mut written = 0;
        for start in 0..tokens.len() {
            if start < written || !crate::introduced::starts_title_run(tokens, start) {
                continue;
            }
            // The longest card name written here.
            let mut node = Some(0);
            let lexeme = tokens[start..]
                .iter()
                .enumerate()
                .filter_map(|(offset, ch)| {
                    node = node.and_then(|at| self.trie[at].child(*ch));
                    node.is_some_and(|at| self.names_card(at))
                        .then_some(offset + 1)
                })
                .last();
            let own = own
                .iter()
                .filter(|name| tokens[start..].starts_with(name))
                .map(Vec::len)
                .max();
            let name = lexeme.max(own).filter(|&length| {
                tokens
                    .get(start + length)
                    .is_none_or(|ch| !ch.is_alphanumeric())
            });
            let Some(end) = crate::introduced::title_run(tokens, start, card_name) else {
                continue;
            };
            let run = &tokens[start..end];
            written = start + name.unwrap_or(0);
            // A card name written here that spells the run or runs past it.
            if name.is_some_and(|length| length >= run.len()) {
                continue;
            }
            matches.push(LexicalMatch {
                start,
                end,
                reading: LexicalReading::IntroducedName {
                    name: run.iter().collect(),
                },
                case: None,
            });
        }
    }

    /// Every licensed lexical occurrence in `text`, as overlapping edges over
    /// its Unicode scalars, with no word split chosen. Words no declaration
    /// covers are reported by [`AnalyzedText::unknown_words`], never guessed.
    #[must_use]
    pub fn analyze(&self, text: &str) -> AnalyzedText {
        self.analyze_source(text, None)
    }

    #[must_use]
    pub fn analyze_source(&self, raw: &str, normalized: Option<&str>) -> AnalyzedText {
        self.analyze_source_scoped(raw, normalized, &crate::CardScope::default())
    }

    /// Analyzes the values that stand in the text of a face whose card names
    /// are `scope` ([`Lexicon::stands_in`]).
    #[must_use]
    pub fn analyze_source_scoped(
        &self,
        raw: &str,
        normalized: Option<&str>,
        scope: &crate::CardScope,
    ) -> AnalyzedText {
        let tokens: Vec<char> = normalized.unwrap_or(raw).chars().collect();
        let mut candidates = Vec::new();
        for start in 0..tokens.len() {
            let mut node = 0;
            for (end, ch) in tokens.iter().enumerate().skip(start) {
                let Some(next) = self.trie[node].child(*ch) else {
                    break;
                };
                node = next;
                for (value, binding, case) in &self.trie[node].values {
                    if !self.word_stands_in(value, scope) {
                        continue;
                    }
                    candidates.push((
                        LexicalMatch {
                            start,
                            end: end + 1,
                            reading: LexicalReading::Word(value.clone()),
                            case: *case,
                        },
                        *binding,
                    ));
                }
            }
        }
        // A name the scope holds is spelled by no Lexeme: each form of it
        // is found wherever its spelling is written.
        for (name, form) in scope.forms() {
            let Some(spelling) = crate::own_name_spelling(name, form) else {
                continue;
            };
            let spelling: Vec<char> = spelling.chars().collect();
            if spelling.is_empty() || !valid_own_name_chars(&spelling) {
                continue;
            }
            for start in 0..tokens.len().saturating_sub(spelling.len() - 1) {
                if tokens[start..].starts_with(&spelling) {
                    candidates.push((
                        LexicalMatch {
                            start,
                            end: start + spelling.len(),
                            reading: LexicalReading::OwnName {
                                name: name.to_owned(),
                                form,
                            },
                            case: None,
                        },
                        Binding::Free,
                    ));
                }
            }
        }
        // Prefixes expose a following word boundary; suffixes expose a
        // preceding one. Both passes follow increasing surface length, with
        // no grammar or host-category judgment at this boundary.
        let mut prefix_ends = vec![false; tokens.len() + 1];
        for (found, binding) in &candidates {
            if matches!(binding, Binding::Prefix | Binding::Bound)
                && (edges_match(&tokens, found.start, found.end, Binding::Prefix)
                    || prefix_ends[found.start]
                    || *binding == Binding::Bound)
            {
                prefix_ends[found.end] = true;
            }
        }
        let mut suffix_starts = vec![false; tokens.len() + 1];
        for (found, binding) in candidates.iter().rev() {
            if matches!(binding, Binding::Suffix | Binding::Bound)
                && (edges_match(&tokens, found.start, found.end, Binding::Suffix)
                    || suffix_starts[found.end]
                    || *binding == Binding::Bound)
            {
                suffix_starts[found.start] = true;
            }
        }
        let mut matches = candidates
            .into_iter()
            .filter_map(|(found, binding)| {
                let left = edges_match(&tokens, found.start, found.end, Binding::Prefix)
                    || matches!(binding, Binding::Suffix | Binding::Bound)
                    || prefix_ends[found.start];
                let right = edges_match(&tokens, found.start, found.end, Binding::Suffix)
                    || matches!(binding, Binding::Prefix | Binding::Bound)
                    || suffix_starts[found.end];
                (left && right).then_some(found)
            })
            .collect::<Vec<_>>();
        drop_shortened_inside_full(&mut matches);
        self.introduced_name_matches(&tokens, scope, &mut matches);
        let open_words = crate::vote_option::introduced_word_matches(&tokens, scope, &mut matches);
        numeral_matches(&tokens, &mut matches);
        flavor_word_matches(&tokens, &mut matches);
        let hosts = matches
            .iter()
            .filter(|found| match &found.reading {
                LexicalReading::Word(value) => self.lexemes[&value.lexeme].set_name_host,
                LexicalReading::OwnName { .. }
                | LexicalReading::IntroducedName { .. }
                | LexicalReading::FlavorWord { .. }
                | LexicalReading::SetName { .. }
                | LexicalReading::KindName { .. }
                | LexicalReading::AnchorWord { .. }
                | LexicalReading::VoteOption { .. }
                | LexicalReading::Numeral { .. } => false,
            })
            .map(|found| found.start)
            .collect();
        crate::set_name::set_name_matches(&tokens, &hosts, &mut matches);
        self.kind_name_matches(&tokens, &mut matches);
        matches.retain(|found| match &found.reading {
            LexicalReading::FlavorWord { label } => {
                self.title_cased(label) && self.stands_in(&found.reading, scope)
            }
            LexicalReading::Word(_)
            | LexicalReading::OwnName { .. }
            | LexicalReading::IntroducedName { .. }
            | LexicalReading::Numeral { .. }
            | LexicalReading::AnchorWord { .. }
            | LexicalReading::VoteOption { .. }
            | LexicalReading::SetName { .. }
            | LexicalReading::KindName { .. } => true,
        });
        AnalyzedText {
            raw: raw.to_owned(),
            normalized: normalized.map(str::to_owned),
            tokens,
            matches,
            open_words,
            scope: scope.clone(),
        }
    }

    /// Adds the Kind Names before each word a counter's kind premodifies
    /// ([`crate::Lexeme::kind_name_host`]), where no word of a closed class,
    /// adjective or numeral spells the word there
    /// ([`crate::LexicalProperties::closed_class`]).
    fn kind_name_matches(&self, tokens: &[char], matches: &mut Vec<LexicalMatch>) {
        let kind_hosts = matches
            .iter()
            .filter(|found| match &found.reading {
                LexicalReading::Word(value) => self.lexemes[&value.lexeme].kind_name_host,
                LexicalReading::OwnName { .. }
                | LexicalReading::IntroducedName { .. }
                | LexicalReading::FlavorWord { .. }
                | LexicalReading::SetName { .. }
                | LexicalReading::KindName { .. }
                | LexicalReading::AnchorWord { .. }
                | LexicalReading::VoteOption { .. }
                | LexicalReading::Numeral { .. } => false,
            })
            .map(|found| found.start)
            .collect();
        let closed: Vec<(usize, usize)> = matches
            .iter()
            .filter(|found| match &found.reading {
                LexicalReading::Word(value) => {
                    let lexeme = &self.lexemes[&value.lexeme];
                    lexeme.properties.closed_class || lexeme.category == Category::Adjective
                }
                LexicalReading::Numeral { .. } => true,
                LexicalReading::OwnName { .. }
                | LexicalReading::IntroducedName { .. }
                | LexicalReading::FlavorWord { .. }
                | LexicalReading::SetName { .. }
                | LexicalReading::KindName { .. }
                | LexicalReading::AnchorWord { .. }
                | LexicalReading::VoteOption { .. } => false,
            })
            .map(|found| (found.start, found.end))
            .collect();
        crate::kind_name::kind_name_matches(
            tokens,
            &kind_hosts,
            |start, end| closed.contains(&(start, end)),
            matches,
        );
    }
}

/// Drops each Shortened form of an own name written inside that name's
/// Full form: the full name written out is the name, and its Shortened form
/// is no name of its own inside it (*Instrument* in *Instrument of the
/// Bards*).
fn drop_shortened_inside_full(matches: &mut Vec<LexicalMatch>) {
    let full_names: Vec<(usize, usize, String)> = matches
        .iter()
        .filter_map(|found| match &found.reading {
            LexicalReading::OwnName {
                name,
                form: crate::NameForm::Full,
            } => Some((found.start, found.end, name.clone())),
            LexicalReading::OwnName {
                form: crate::NameForm::Shortened,
                ..
            }
            | LexicalReading::Word(_)
            | LexicalReading::IntroducedName { .. }
            | LexicalReading::Numeral { .. }
            | LexicalReading::FlavorWord { .. }
            | LexicalReading::AnchorWord { .. }
            | LexicalReading::VoteOption { .. }
            | LexicalReading::SetName { .. }
            | LexicalReading::KindName { .. } => None,
        })
        .collect();
    matches.retain(|found| match &found.reading {
        LexicalReading::OwnName {
            name,
            form: crate::NameForm::Shortened,
        } => !full_names
            .iter()
            .any(|(start, end, full)| full == name && *start <= found.start && found.end <= *end),
        LexicalReading::OwnName {
            form: crate::NameForm::Full,
            ..
        }
        | LexicalReading::Word(_)
        | LexicalReading::IntroducedName { .. }
        | LexicalReading::Numeral { .. }
        | LexicalReading::FlavorWord { .. }
        | LexicalReading::AnchorWord { .. }
        | LexicalReading::VoteOption { .. }
        | LexicalReading::SetName { .. }
        | LexicalReading::KindName { .. } => true,
    });
}

/// Why a card name's Lexeme is malformed: it has a second form, or is not
/// spelled as its lemma, the full name.
fn card_name_defect(lexeme: &Lexeme) -> Option<&'static str> {
    (lexeme.card_name && (lexeme.forms.len() != 1 || lexeme.forms[0].surface.is_some()))
        .then_some("a card name has one form, spelled as the full name")
}

/// Whether a spelling is one an own name may have: a line of text that
/// begins and ends with something written.
fn valid_own_name(spelling: &str) -> bool {
    valid_own_name_chars(&spelling.chars().collect::<Vec<_>>())
}

fn valid_own_name_chars(spelling: &[char]) -> bool {
    let written = |ch: &char| !ch.is_whitespace();
    spelling.first().is_some_and(written)
        && spelling.last().is_some_and(written)
        && !spelling.iter().any(|ch| ch.is_control())
}

/// Why a Lexeme's Adjacency Agreement paradigm is incomplete
/// ([`crate::FeatureBundle::complete_agreement`]).
fn agreement_defect(lexeme: &Lexeme) -> Option<&'static str> {
    let bundles: Vec<_> = lexeme.forms.iter().map(|slot| &slot.features).collect();
    (!crate::FeatureBundle::complete_agreement(&bundles))
        .then_some("an agreement dimension lacks a form for some value")
}

/// The spelling of one declared slot: its override, or the default rule.
fn spelling(lexeme: &Lexeme, slot: &crate::FormDeclaration) -> String {
    slot.surface
        .clone()
        .unwrap_or_else(|| morphology::default_surface(lexeme, slot.form, &slot.features))
}

/// The genitive alternant a host form takes, given its Lexeme's Plural
/// spellings: the bare genitive after a form ending in *s* that is a Plural,
/// regular or irregular (*players'*, *elves'*), or that is spelled like its
/// Plural (*Thalakos'* singular and plural alike); else the *'s* genitive,
/// after a singular noun ending in *s* whose Plural differs (*Octopus's*),
/// after a name the text treats as singular ending in *s* (*Swarm of
/// Rats's*), and after a form not ending in *s* (*Spawn's*). CGEL, Ch. 18
/// §4.2, p. 1596: the bare genitive is obligatory with plural nouns ending in
/// *s*, and "[n]ouns like *species* which have identical singular and plural
/// forms with final *s* take a bare genitive in the singular as well as the
/// plural"; with a singular proper name it is only a variant of *'s* ([35ii],
/// p. 1595), and Oracle writes *'s*.
fn genitive_host(
    form: WordForm,
    surface: &str,
    plurals: &BTreeSet<String>,
) -> crate::GenitiveAlternant {
    if surface.ends_with('s') && (form == WordForm::Plural || plurals.contains(surface)) {
        crate::GenitiveAlternant::Bare
    } else {
        crate::GenitiveAlternant::ApostropheS
    }
}

fn valid_flavor_label(label: &str) -> bool {
    !label.is_empty()
        && label.chars().any(char::is_alphabetic)
        && !label.contains(['{', '}', '[', ']'])
        && !label.starts_with('•')
        && !label.chars().any(char::is_control)
        && !label.contains(" —")
}

/// Whether `word` has the shape of an Anchor Word: a label an ability may
/// be headed with ([`valid_flavor_label`]), written without edge spaces.
#[must_use]
pub fn valid_anchor_word(word: &str) -> bool {
    valid_flavor_label(word) && word.trim() == word
}

fn missing_explicit_spelling(slot: &crate::FormDeclaration) -> Option<&'static str> {
    if slot.surface.is_some() {
        return None;
    }
    match slot.form {
        WordForm::Irrealis => Some("irrealis requires an explicit spelling"),
        WordForm::IndependentGenitive => Some("independent Genitive requires an explicit spelling"),
        _ => None,
    }
}

fn flavor_word_matches(tokens: &[char], matches: &mut Vec<LexicalMatch>) {
    for start in 0..tokens.len() {
        if start != 0 && tokens[start - 1] != '\n' {
            continue;
        }
        // Bullets and chapter numerals belong to the enclosing ability.
        let mut start = if tokens[start..].starts_with(&['•', ' ']) { start + 2 } else { start };
        let line_end = tokens[start..]
            .iter()
            .position(|ch| ch.is_control())
            .map_or(tokens.len(), |offset| start + offset);
        while let Some(offset) = tokens[start..line_end]
            .windows(2)
            .position(|pair| pair == [' ', '—'])
        {
            let dash = start + offset + 1;
            let label: String = tokens[start..dash - 1].iter().collect();
            let chapters = label
                .split(", ")
                .all(|part| crate::NumeralCodec::parse(crate::Numeral::Roman, part).is_ok());
            if chapters {
                start = dash + 1;
                if tokens.get(start) == Some(&' ') {
                    start += 1;
                }
                continue;
            }
            if valid_flavor_label(&label) {
                matches.push(LexicalMatch {
                    start,
                    end: dash + 1,
                    reading: LexicalReading::FlavorWord { label },
                    case: None,
                });
            }
            break;
        }
    }
}

fn word_character(ch: char) -> bool {
    unicode_ident::is_xid_continue(ch)
}

fn valid_surface(surface: &str, structure: SurfaceStructure, binding: Binding) -> bool {
    match structure {
        SurfaceStructure::Opaque => !surface.chars().any(char::is_control),
        SurfaceStructure::Multiword => surface
            .split(' ')
            .all(|word| valid_word(word, Binding::Free)),
        SurfaceStructure::Word => valid_word(surface, binding),
    }
}

fn valid_word(word: &str, binding: Binding) -> bool {
    let characters: Vec<_> = word.chars().collect();
    !characters.is_empty()
        && characters.iter().enumerate().all(|(index, ch)| {
            word_character(*ch)
                || *ch == '/'
                    && index > 0
                    && characters[index - 1].is_alphabetic()
                    && index + 1 < characters.len()
                    && characters[index + 1].is_alphabetic()
                || matches!(ch, '\'' | '-')
                    && (index > 0 && word_character(characters[index - 1])
                        || matches!(binding, Binding::Suffix | Binding::Bound))
                    && (index + 1 < characters.len() && word_character(characters[index + 1])
                        || matches!(binding, Binding::Prefix | Binding::Suffix | Binding::Bound))
        })
}

pub(crate) fn edges_match(tokens: &[char], start: usize, end: usize, binding: Binding) -> bool {
    let left = start == 0 || !word_character(tokens[start - 1]) || !word_character(tokens[start]);
    let right =
        end == tokens.len() || !word_character(tokens[end - 1]) || !word_character(tokens[end]);
    (left || matches!(binding, Binding::Suffix | Binding::Bound))
        && (right || matches!(binding, Binding::Prefix | Binding::Bound))
}

fn initial_surface(surface: &str) -> String {
    let mut chars = surface.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    first.to_uppercase().chain(chars).collect()
}

fn numeral_matches(tokens: &[char], matches: &mut Vec<LexicalMatch>) {
    for start in 0..tokens.len() {
        if tokens[start].is_whitespace() || !edges_match(tokens, start, start + 1, Binding::Prefix)
        {
            continue;
        }
        let mut end = start;
        let mut text = String::new();
        loop {
            let component_start = end;
            while end < tokens.len() && word_character(tokens[end]) {
                end += 1;
            }
            if component_start == end {
                if end == start && tokens[end] == '-' {
                    text.push('-');
                    end += 1;
                    continue;
                }
                break;
            }
            let component: String = tokens[component_start..end].iter().collect();
            let lower = component.to_lowercase();
            if !crate::numeral::numeral_component(&component)
                && !crate::numeral::numeral_component(&lower)
            {
                break;
            }
            text.push_str(&component);
            if edges_match(tokens, start, end, Binding::Free) {
                for notation in [
                    Numeral::Cardinal,
                    Numeral::Ordinal,
                    Numeral::Arabic(false),
                    Numeral::Arabic(true),
                    Numeral::Roman,
                ] {
                    // The declared spelling is the formatted one, and a numeral
                    // spelled in words also takes an initial capital.
                    for case in [LetterCase::Lower, LetterCase::Capital] {
                        let candidate = if case == LetterCase::Capital {
                            let mut chars = text.chars();
                            let Some(first) = chars.next() else {
                                continue;
                            };
                            first.to_lowercase().chain(chars).collect::<String>()
                        } else {
                            text.clone()
                        };
                        let Ok(value) = notation.parse(&candidate) else {
                            continue;
                        };
                        if notation.canonical_notation(value) != notation {
                            continue;
                        }
                        let capitalized = initial_surface(&candidate) != candidate;
                        if case == LetterCase::Capital
                            && (initial_surface(&candidate) != text || !capitalized)
                        {
                            continue;
                        }
                        matches.push(LexicalMatch {
                            start,
                            end,
                            reading: LexicalReading::Numeral { value, notation },
                            case: capitalized.then_some(case),
                        });
                    }
                }
            }
            if end == tokens.len() || !matches!(tokens[end], ' ' | '-' | ',') {
                break;
            }
            while end < tokens.len() && matches!(tokens[end], ' ' | '-' | ',') {
                text.push(tokens[end]);
                end += 1;
            }
            if end == tokens.len() {
                break;
            }
        }
    }
}
