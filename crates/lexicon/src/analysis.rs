use std::collections::BTreeMap;
use std::collections::BTreeSet;

use serde::Deserialize;
use serde::Serialize;

use crate::Binding;
use crate::Capitalization;
use crate::Category;
use crate::Lexeme;
use crate::LexicalReading;
use crate::LexicalValue;
use crate::SurfaceCase;
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
    /// A declared Anchor Word spells the run [CR#614.12c]. Printed in roman
    /// type, it is no Italic Head, but like an Ability Word it is a registered
    /// label, never a Flavor Word.
    AnchorWord(LexicalValue),
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyzedText {
    pub raw: String,
    /// None means the raw input was analyzed directly.
    pub normalized: Option<String>,
    pub tokens: Vec<char>,
    pub matches: Vec<LexicalMatch>,
    /// The card names the text was analyzed in the scope of: the names of
    /// the face whose text it is.
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

    #[must_use]
    pub fn unknown_words(&self) -> Vec<std::ops::Range<usize>> {
        self.words()
            .into_iter()
            .filter(|word| !self.is_covered(word.clone(), |_| true))
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
        if range.start >= range.end || range.end > self.tokens.len() {
            return false;
        }
        let mut edges = vec![Vec::new(); range.len()];
        for found in &self.matches {
            if found.start < range.end && found.end > range.start && include(&found.reading) {
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
    values: Vec<(LexicalValue, Binding)>,
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
    surface_features: BTreeMap<LexicalValue, crate::SurfaceFeatures>,
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
            surface_features: BTreeMap::new(),
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
            if lexeme.surface_structure == SurfaceStructure::MeasuredCompound
                && (lexeme.category != Category::Noun || lexeme.binding != Binding::Free)
            {
                return Err(error(
                    "a measured compound requires a free noun declaration",
                ));
            }
            if !valid_surface(&lexeme.lemma, lexeme.surface_structure, lexeme.binding) {
                return Err(error("lemma violates its declared word boundaries"));
            }
            let mut slots = BTreeSet::new();
            let mut realized_surfaces = BTreeSet::new();
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
                if !morphology::applicable(lexeme.category, declaration.form, &declaration.features)
                {
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
                realized_surfaces.insert(surface.clone());
                let value = LexicalValue {
                    lexeme: lexeme.id,
                    form: declaration.form,
                    features: declaration.features.clone(),
                    capitalization: SurfaceCase::Declared,
                };
                let features = crate::SurfaceFeatures {
                    onset: lexeme
                        .onsets
                        .get(&surface)
                        .copied()
                        .or_else(|| crate::pronunciation::default_onset(&surface)),
                    initial: initial_surface(&surface) == surface,
                    interior: true,
                };
                let edges = crate::Edges {
                    left: crate::LeftEdge {
                        onset: features.onset,
                        prefix_hyphenation: lexeme.properties.prefix_hyphenation,
                        genitive: declaration.features.genitive,
                    },
                    right: crate::RightEdge {
                        genitive_host: genitive_host(declaration.form, &surface, &plurals),
                        following_onset: declaration.features.following_onset,
                        hyphenation: declaration.features.hyphenation,
                    },
                };
                let initial = initial_surface(&surface);
                let capitalized =
                    lexeme.capitalization == Capitalization::Initial && initial != surface;
                result.surface_features.insert(value.clone(), features);
                result.edges.insert(value.clone(), edges);
                result.insert(value.clone(), surface, lexeme.binding);
                if capitalized {
                    let value = LexicalValue {
                        capitalization: SurfaceCase::Initial,
                        ..value
                    };
                    result.surface_features.insert(
                        value.clone(),
                        crate::SurfaceFeatures {
                            initial: true,
                            interior: false,
                            ..features
                        },
                    );
                    result.edges.insert(value.clone(), edges);
                    result.insert(value, initial, lexeme.binding);
                }
            }
            if let Some(reason) = card_scope_defect(&lexeme).or_else(|| agreement_defect(&lexeme)) {
                return Err(error(reason));
            }
            if lexeme
                .onsets
                .keys()
                .any(|surface| !realized_surfaces.contains(surface))
            {
                return Err(error("onset override has no declared spelling"));
            }
            result.lexemes.insert(lexeme.id, lexeme);
        }
        Ok(result)
    }

    fn insert(&mut self, value: LexicalValue, surface: String, binding: Binding) {
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
        self.trie[node].values.push((value.clone(), binding));
        self.surfaces.insert(value, surface);
    }

    #[must_use]
    pub fn lexemes(&self) -> &BTreeMap<crate::LexemeId, Lexeme> {
        &self.lexemes
    }

    /// Enumerates licensed values independently of any analyzed source.
    pub fn values(&self) -> impl Iterator<Item = &LexicalValue> {
        self.surfaces.keys()
    }

    /// Frozen pronunciation and positional capabilities of one licensed
    /// realization.
    ///
    /// # Errors
    /// Rejects undeclared word values and noncanonical numeral values.
    pub fn surface_features(
        &self,
        reading: &LexicalReading,
    ) -> Result<crate::SurfaceFeatures, LexicalError> {
        match reading {
            LexicalReading::FlavorWord { .. } => {
                self.realize(reading)?;
                Ok(crate::SurfaceFeatures {
                    onset: None,
                    initial: true,
                    interior: false,
                })
            }
            LexicalReading::Word(value) => self
                .surface_features
                .get(value)
                .copied()
                .ok_or(LexicalError::UnlicensedValue),
            LexicalReading::Numeral {
                value,
                capitalization,
                ..
            } => {
                let text = self.realize(reading)?;
                Ok(crate::SurfaceFeatures {
                    onset: crate::pronunciation::numeral_onset(*value),
                    initial: initial_surface(&text) == text,
                    interior: *capitalization == SurfaceCase::Declared,
                })
            }
        }
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
            LexicalReading::Numeral { .. } | LexicalReading::FlavorWord { .. } => {
                Ok(crate::Edges::plain(self.surface_features(reading)?.onset))
            }
        }
    }

    /// # Errors
    /// Rejects values not admitted by this lexicon, including lossy numeral
    /// values.
    pub fn realize(&self, reading: &LexicalReading) -> Result<String, LexicalError> {
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
            LexicalReading::Word(value) => self
                .surfaces
                .get(value)
                .cloned()
                .ok_or(LexicalError::UnlicensedValue),
            LexicalReading::Numeral {
                value,
                notation,
                capitalization,
            } => {
                if notation.canonical_notation(*value) != *notation {
                    return Err(LexicalError::UnlicensedValue);
                }
                let text = notation
                    .try_format(*value)
                    .map_err(|_| LexicalError::UnlicensedValue)?;
                if *capitalization == SurfaceCase::Declared {
                    return Ok(text);
                }
                let initial = initial_surface(&text);
                if initial == text {
                    return Err(LexicalError::UnlicensedValue);
                }
                Ok(initial)
            }
        }
    }

    /// Analyzes an italic run written at the head of an ability
    /// [CR#207.2c,207.2d].
    ///
    /// A run no registered label spells is not a vocabulary gap here: it is
    /// a flavor word, and the label is the run verbatim. Nothing
    /// enumerates flavor words, so this is the only open slot the lexicon
    /// reports rather than an unknown word.
    #[must_use]
    pub fn analyze_italic_head(&self, run: &str) -> ItalicHead {
        match self.registered_label(run, None) {
            Some(value) => match self.label_kind(&value) {
                Some("AbilityWord") => ItalicHead::AbilityWord(value),
                Some("AnchorWord") => ItalicHead::AnchorWord(value),
                _ => ItalicHead::Keyword(value),
            },
            None => ItalicHead::FlavorWord {
                label: run.to_owned(),
            },
        }
    }

    /// The `LabelKind` a registered label declares: `AbilityWord` or
    /// `AnchorWord`; a Keyword Ability declares none.
    fn label_kind(&self, value: &LexicalValue) -> Option<&str> {
        self.lexemes[&value.lexeme]
            .properties
            .features
            .get("LabelKind")
            .map(dpsi::Ident::as_str)
    }

    /// The registered label that spells all of `label`: a Keyword-category
    /// Lexeme, which is an Ability Word [CR#207.2c], an Anchor Word
    /// [CR#614.12c] or a Keyword Ability [CR#702.1]. A declared label kind is
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
            .map(|(value, _)| value)
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
        self.trie[node].values.iter().any(|(value, _)| {
            matches!(
                self.lexemes[&value.lexeme].category,
                Category::Determinative
                    | Category::Pronoun
                    | Category::Preposition
                    | Category::Coordinator
                    | Category::Subordinator
            )
        })
    }

    /// Whether a lexical value stands in the text of a face whose card names
    /// are `scope`: analysis finds it there, and admission admits it there.
    ///
    /// - A card name's Full form stands everywhere: in its scope it is the
    ///   card's own name, and elsewhere another card's [CR#201.5]. Its
    ///   Shortened form stands only where the scope holds that form
    ///   [CR#201.5c].
    /// - A card-scoped Lexeme, an Introduced Name or an Anchor Word, stands
    ///   only where the scope holds one of its cards.
    /// - A Flavor Word stands only where no label registered there spells it.
    /// - Every other value stands everywhere.
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
            LexicalReading::FlavorWord { label } => {
                self.registered_label(label, Some(scope)).is_none()
            }
            LexicalReading::Numeral { .. } => true,
        }
    }

    fn word_stands_in(&self, value: &LexicalValue, scope: &crate::CardScope) -> bool {
        let lexeme = self
            .lexemes
            .get(&value.lexeme)
            .expect("a word value asked about scope is a Lexeme of this lexicon");
        match value.features.name {
            Some(crate::NameForm::Full) => true,
            Some(form) => scope.holds(value.lexeme, form),
            None => {
                lexeme.card_scope.is_empty()
                    || lexeme.card_scope.iter().any(|card| scope.holds_card(*card))
            }
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
                for (value, binding) in &self.trie[node].values {
                    if !self.word_stands_in(value, scope) {
                        continue;
                    }
                    candidates.push((
                        LexicalMatch {
                            start,
                            end: end + 1,
                            reading: LexicalReading::Word(value.clone()),
                        },
                        *binding,
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
                let measured_left = match &found.reading {
                    LexicalReading::Word(value)
                        if self.lexemes[&value.lexeme].surface_structure
                            == SurfaceStructure::MeasuredCompound =>
                    {
                        found.start == 0
                            || !word_character(tokens[found.start - 1])
                                && !matches!(tokens[found.start - 1], '+' | '-' | '/')
                    }
                    _ => true,
                };
                (left && right && measured_left).then_some(found)
            })
            .collect::<Vec<_>>();
        numeral_matches(&tokens, &mut matches);
        flavor_word_matches(&tokens, &mut matches);
        matches.retain(|found| match &found.reading {
            LexicalReading::FlavorWord { label } => {
                self.title_cased(label) && self.stands_in(&found.reading, scope)
            }
            LexicalReading::Word(_) | LexicalReading::Numeral { .. } => true,
        });
        AnalyzedText {
            raw: raw.to_owned(),
            normalized: normalized.map(str::to_owned),
            tokens,
            matches,
            scope: scope.clone(),
        }
    }
}

/// Why a Lexeme's card scope or name forms are malformed: a card name scoped
/// to other cards, a card name without exactly one Full form and at most one
/// Shortened form, or name forms on a Lexeme that is no card name.
fn card_scope_defect(lexeme: &Lexeme) -> Option<&'static str> {
    let names: Vec<_> = lexeme.forms.iter().map(|slot| slot.features.name).collect();
    let count = |form| names.iter().filter(|name| **name == Some(form)).count();
    if !lexeme.card_name {
        names
            .iter()
            .any(Option::is_some)
            .then_some("name forms on a Lexeme that is no card name")
    } else if !lexeme.card_scope.is_empty() {
        Some("a card name is scoped to its own card, never to others")
    } else if names.iter().any(Option::is_none)
        || count(crate::NameForm::Full) != 1
        || count(crate::NameForm::Shortened) > 1
    {
        Some("a card name has one Full form and at most one Shortened form")
    } else {
        None
    }
}

/// Why a Lexeme's Adjacency Agreement paradigm is incomplete: a dimension
/// some form carries must be carried by every form, with every value, since
/// the context may fix any of them.
fn agreement_defect(lexeme: &Lexeme) -> Option<&'static str> {
    fn complete<T: PartialEq + Copy>(
        lexeme: &Lexeme,
        dimension: impl Fn(&crate::FeatureBundle) -> Option<T>,
        domain: &[T],
    ) -> bool {
        let values: Vec<Option<T>> = lexeme
            .forms
            .iter()
            .map(|slot| dimension(&slot.features))
            .collect();
        values.iter().all(Option::is_none)
            || values.iter().all(Option::is_some)
                && domain.iter().all(|value| values.contains(&Some(*value)))
    }
    let complete = complete(
        lexeme,
        |features| features.following_onset,
        &[crate::Onset::Consonant, crate::Onset::Vowel],
    ) && complete(
        lexeme,
        |features| features.hyphenation,
        &[crate::Hyphenation::Solid, crate::Hyphenation::Hyphenated],
    ) && complete(
        lexeme,
        |features| features.genitive,
        &[
            crate::GenitiveAlternant::ApostropheS,
            crate::GenitiveAlternant::Bare,
        ],
    );
    (!complete).then_some("an agreement dimension lacks a form for some value")
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
        SurfaceStructure::MeasuredCompound => {
            let Some((measure, head)) = surface.split_once(' ') else {
                return false;
            };
            let Some((left, right)) = measure.split_once('/') else {
                return false;
            };
            canonical_measure_component(left)
                && canonical_measure_component(right)
                && valid_word(head, Binding::Free)
        }
        SurfaceStructure::Word => valid_word(surface, binding),
    }
}

fn canonical_measure_component(component: &str) -> bool {
    if let Some(unsigned) = component
        .strip_prefix('+')
        .or_else(|| component.strip_prefix('-'))
    {
        Numeral::Arabic(false)
            .parse(unsigned)
            .is_ok_and(|value| value >= 0)
            || (component.starts_with('-') && Numeral::Arabic(false).parse(component).is_ok())
    } else {
        Numeral::Arabic(false).parse(component).is_ok()
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

fn edges_match(tokens: &[char], start: usize, end: usize, binding: Binding) -> bool {
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
                    for capitalization in [SurfaceCase::Declared, SurfaceCase::Initial] {
                        let candidate = if capitalization == SurfaceCase::Initial {
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
                        if capitalization == SurfaceCase::Initial
                            && (initial_surface(&candidate) != text || candidate == text)
                        {
                            continue;
                        }
                        matches.push(LexicalMatch {
                            start,
                            end,
                            reading: LexicalReading::Numeral {
                                value,
                                notation,
                                capitalization,
                            },
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
