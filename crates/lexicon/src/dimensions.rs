//! The Feature Bundle's dimensions, each declared once.
//!
//! A dimension owned by one Category's invariant form, and an Adjacency
//! Agreement dimension besides with the edge that carries it and the
//! property of the neighboring word it agrees with. Applicability,
//! paradigm completeness, the edges and the agreement relation all derive
//! from [`dimensions!`]'s one declaration.

use serde::Deserialize;
use serde::Serialize;

use crate::Case;
use crate::Category;
use crate::Finiteness;
use crate::Number;
use crate::Person;
use crate::Tense;

/// A finite value domain whose `ALL` lists every value, in declaration
/// order.
macro_rules! domain {
    (
        $(#[$meta:meta])*
        pub enum $name:ident { $( $(#[$variant_meta:meta])* $variant:ident, )* }
    ) => {
        $(#[$meta])*
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Ord, PartialOrd, Serialize,
        )]
        pub enum $name { $( $(#[$variant_meta])* $variant, )* }

        impl $name {
            /// Every value.
            pub const ALL: &[Self] = &[$(Self::$variant),*];
        }
    };
}

/// Declares the Feature Bundle.
///
/// - `inflection`: the inflectional dimensions, which [`crate::morphology`]
///   checks per Word Form.
/// - `agrees_with_next`: an Adjacency Agreement dimension of the word before,
///   carried at its [`RightEdge`], that must equal a property of the word
///   after, read at that word's [`LeftEdge`].
/// - `agrees_with_previous`: the mirror image, carried at the [`LeftEdge`] and
///   agreeing with a property at the previous word's [`RightEdge`].
/// - `selected`: a dimension that selects a form without agreeing with a
///   neighbor.
///
/// Every dimension but an inflectional one belongs to one Category's
/// invariant form. A property is compared through `Option::from`, so it is
/// either always present or an `Option` that is `None` where the word has
/// none, which no dimension agrees with.
macro_rules! dimensions {
    (
        $(#[$bundle_meta:meta])*
        inflection { $( $inflection:ident: $inflection_ty:ty, )* }
        agrees_with_next {
            $(
                $(#[$next_meta:meta])*
                $next:ident: $next_ty:ty => $next_owner:ident,
                reads $(#[$next_property_meta:meta])* $next_property:ident: $next_property_ty:ty;
            )*
        }
        agrees_with_previous {
            $(
                $(#[$previous_meta:meta])*
                $previous:ident: $previous_ty:ty => $previous_owner:ident,
                reads $(#[$previous_property_meta:meta])*
                    $previous_property:ident: $previous_property_ty:ty;
            )*
        }
        selected {
            $( $(#[$selected_meta:meta])* $selected:ident: $selected_ty:ty => $selected_owner:ident; )*
        }
    ) => {
        $(#[$bundle_meta])*
        #[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Ord, PartialOrd, Serialize)]
        #[serde(deny_unknown_fields)]
        pub struct FeatureBundle {
            $( pub $inflection: Option<$inflection_ty>, )*
            $( $(#[$next_meta])* #[serde(default)] pub $next: Option<$next_ty>, )*
            $( $(#[$previous_meta])* #[serde(default)] pub $previous: Option<$previous_ty>, )*
            $( $(#[$selected_meta])* #[serde(default)] pub $selected: Option<$selected_ty>, )*
        }

        impl FeatureBundle {
            /// The Category that owns each non-inflectional dimension the
            /// bundle carries: only that Category's invariant form carries
            /// it.
            pub(crate) fn owners(&self) -> Vec<Category> {
                let mut owners = Vec::new();
                $( if self.$next.is_some() { owners.push(Category::$next_owner); } )*
                $( if self.$previous.is_some() { owners.push(Category::$previous_owner); } )*
                $( if self.$selected.is_some() { owners.push(Category::$selected_owner); } )*
                owners
            }

            /// The bundle's inflectional dimensions alone.
            pub(crate) fn inflection(&self) -> Self {
                Self {
                    $( $inflection: self.$inflection, )*
                    ..Self::default()
                }
            }

            /// Whether each Adjacency Agreement dimension one of `bundles`
            /// carries is carried by every one, with every value between
            /// them: the context may fix any value.
            pub(crate) fn complete_agreement(bundles: &[&Self]) -> bool {
                fn complete<T: PartialEq + Copy>(values: &[Option<T>], domain: &[T]) -> bool {
                    values.iter().all(Option::is_none)
                        || values.iter().all(Option::is_some)
                            && domain.iter().all(|value| values.contains(&Some(*value)))
                }
                true
                $( && complete(
                    &bundles.iter().map(|bundle| bundle.$next).collect::<Vec<_>>(),
                    <$next_ty>::ALL,
                ) )*
                $( && complete(
                    &bundles.iter().map(|bundle| bundle.$previous).collect::<Vec<_>>(),
                    <$previous_ty>::ALL,
                ) )*
            }
        }

        /// What Adjacency Agreement reads at a word's left edge: the
        /// properties the word before it agrees with, and its own dimensions
        /// that agree with that word.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
        pub struct LeftEdge {
            $( $(#[$next_property_meta])* pub $next_property: $next_property_ty, )*
            $( #[doc = concat!("Its [`FeatureBundle::", stringify!($previous), "`].")]
               pub $previous: Option<$previous_ty>, )*
        }

        /// What Adjacency Agreement reads at a word's right edge: the
        /// properties the word after it agrees with, and its own dimensions
        /// that agree with that word.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
        pub struct RightEdge {
            $( $(#[$previous_property_meta])* pub $previous_property: $previous_property_ty, )*
            $( #[doc = concat!("Its [`FeatureBundle::", stringify!($next), "`].")]
               pub $next: Option<$next_ty>, )*
        }

        /// The properties of a word that its neighbors' Adjacency Agreement
        /// dimensions agree with, by name, so no two can be swapped.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct AgreementProperties {
            $( $(#[$next_property_meta])* pub $next_property: $next_property_ty, )*
            $( $(#[$previous_property_meta])* pub $previous_property: $previous_property_ty, )*
        }

        impl Edges {
            /// The edges of a word with the Feature Bundle `features` and
            /// these properties.
            #[must_use]
            pub fn new(features: &FeatureBundle, properties: AgreementProperties) -> Self {
                let AgreementProperties {
                    $( $next_property, )*
                    $( $previous_property, )*
                } = properties;
                Self {
                    left: LeftEdge {
                        $( $next_property, )*
                        $( $previous: features.$previous, )*
                    },
                    right: RightEdge {
                        $( $previous_property, )*
                        $( $next: features.$next, )*
                    },
                }
            }
        }

        impl RightEdge {
            /// Adjacency Agreement: whether a word with this right edge may
            /// stand immediately before one with the `next` left edge. Each
            /// agreement dimension of either word must equal the property
            /// of the other it agrees with.
            #[must_use]
            pub fn agrees(self, next: LeftEdge) -> bool {
                true
                $( && self.$next.is_none_or(|value| {
                    Option::<$next_ty>::from(next.$next_property) == Some(value)
                }) )*
                $( && next.$previous.is_none_or(|value| {
                    Option::<$previous_ty>::from(self.$previous_property) == Some(value)
                }) )*
            }
        }
    };
}

domain! {
    /// Whether a word begins with a consonant or a vowel sound.
    pub enum Onset {
        Consonant,
        Vowel,
    }
}

domain! {
    /// How a prefix joins its host: solid (*nonland*) or with a hyphen
    /// (*non-Vampire*; `docs/oracle-style-guide.md` §7, Negation with non-).
    /// The host's lexical property selects it: a host that declares none is
    /// `Solid`, and a word of a Category that hosts no prefix has none.
    pub enum Hyphenation {
        Solid,
        Hyphenated,
    }
}

domain! {
    /// The two alternants of the genitive suffix (CGEL, Ch. 18 §4.2: the
    /// [35] table, p. 1595, and its account, p. 1596): the bare genitive, an
    /// apostrophe alone, which every Plural noun form ending in *s* takes,
    /// regular or irregular (*players'*, *elves'*), and so does the singular
    /// of a noun spelled alike in both numbers with final *s* (*Thalakos'*,
    /// singular or plural); and the default *'s* genitive after any other
    /// form (*player's*, *Spawn's*), a singular noun ending in *s* whose
    /// plural differs and a name the text treats as singular ending in *s*
    /// included (*Swarm of Rats's*): [35ii] makes the bare genitive of a
    /// singular proper name only a variant, and Oracle writes *'s*.
    pub enum GenitiveAlternant {
        ApostropheS,
        Bare,
    }
}

domain! {
    /// The two forms of a card name: the Full name and its Shortened Name
    /// [CR#201.5c].
    pub enum NameForm {
        Full,
        Shortened,
    }
}

dimensions! {
    /// One correlated lexical alternative.
    ///
    /// A missing dimension is inapplicable, not a wildcard. Which
    /// combinations are applicable is enforced by the lexical engine that
    /// owns morphology.
    ///
    /// The Adjacency Agreement dimensions select a word's allomorph by a
    /// property of the word next to it, as Number selects a form by its
    /// subject: the context fixes each one, and no allomorph is a free
    /// choice. The relation is [`RightEdge::agrees`].
    inflection {
        number: Number,
        person: Person,
        tense: Tense,
        finiteness: Finiteness,
        case: Case,
    }
    agrees_with_next {
        /// An indefinite article's: agrees with the Onset of the word after
        /// it (*a creature*, *an artifact*, *an 8/8*).
        following_onset: Onset => Determinative,
        reads
            /// Its Onset, which an article before it agrees with; `None`
            /// when no pronunciation is known, which no article agrees with.
            onset: Option<Onset>;
        /// A prefix's: agrees with how the host after it takes a prefix
        /// ([`crate::LexicalProperties::prefix_hyphenation`]).
        hyphenation: Hyphenation => Affix,
        reads
            /// How a prefix before it joins it; `None` when its Category
            /// hosts no prefix ([`Category::hosts_prefix`]), which no prefix
            /// agrees with.
            prefix_hyphenation: Option<Hyphenation>;
    }
    agrees_with_previous {
        /// The genitive suffix's: agrees with the alternant the host before
        /// it takes ([`RightEdge::genitive_host`]).
        genitive: GenitiveAlternant => Clitic,
        reads
            /// The genitive alternant a genitive suffix after it takes
            /// ([`GenitiveAlternant`]).
            genitive_host: GenitiveAlternant;
    }
    selected {
        /// A card name's form [CR#201.5c].
        name: NameForm => Catalog;
    }
}

/// The edges of one realized word.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Edges {
    pub left: LeftEdge,
    pub right: RightEdge,
}

impl Edges {
    /// The edges of a numeral or a Flavor Word, which no Feature Bundle
    /// declares: a numeral has the Onset of its spoken cardinal (*an 8/8*),
    /// a Flavor Word none, neither hosts a prefix, and each takes the *'s*
    /// genitive. `None` for a declared word, whose edges its Lexicon holds
    /// ([`crate::Lexicon::edges`]).
    #[must_use]
    pub fn non_word(reading: &crate::LexicalReading) -> Option<Self> {
        let onset = match reading {
            crate::LexicalReading::Word(_) => return None,
            crate::LexicalReading::Numeral { value, .. } => crate::numeral_onset(*value),
            crate::LexicalReading::FlavorWord { .. } => None,
        };
        Some(Self::plain(onset))
    }

    /// The edges of a word with this Onset and no Feature Bundle: a numeral,
    /// a Flavor Word or a spoken mark. It hosts no prefix and takes the *'s*
    /// genitive.
    #[must_use]
    pub fn plain(onset: Option<Onset>) -> Self {
        Self::new(
            &FeatureBundle::default(),
            AgreementProperties {
                onset,
                prefix_hyphenation: None,
                genitive_host: GenitiveAlternant::ApostropheS,
            },
        )
    }
}
