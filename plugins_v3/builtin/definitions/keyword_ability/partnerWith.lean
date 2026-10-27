/-- "Partner with [name]" represents two abilities [CR#702.124j]. The first lets two legendary
cards be your commander and functions before the game begins, a deck-construction rule of the
Commander variant [CR#702.124a]; it is no ability of the game, so the body writes the second:
"When this permanent enters, target player may search their library for a card named [name],
reveal it, put it into their hand, then shuffle." `quality` is the card's name. -/
keyword partnerWith (quality : Predicate) : Ability := {
  lexical := {
    lemma := "partner",
    parameters := [[], [Name (Some ("with"))]],
    variants := [
                  { lemma := "partner—Character select" },
                  { lemma := "partner—Father & son" },
                  { lemma := "partner—Friends forever" },
                  { lemma := "partner—Survivors" },
                  { lemma := "choose a Background" },
                  { lemma := "doctor's companion" }
                ]
  },
  -- TODO(spelling): Partner with [card name]; quality is a named Predicate, so the spelling
  -- must extract its name rather than substitute the rendered predicate.
  body :=
    [Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent) Semantics.Templates.Action.enter)
        (Semantics.Templates.Action.instruct (Semantics.Templates.NounPhrase.target Semantics.Templates.Predicate.isPlayer)
          (Semantics.Templates.Action.may
            (Semantics.Templates.Action.sequentially
              [Semantics.Definitions.search (Semantics.Templates.Location.libraryOf Semantics.Templates.NounPhrase.actor)
                  (Semantics.Templates.NounPhrase.a (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isCard, quality])),
                Semantics.Definitions.reveal Semantics.Templates.NounPhrase.it,
                Semantics.Templates.Action.putInto Semantics.Templates.NounPhrase.it Semantics.Templates.Location.hand,
                Semantics.Definitions.shuffle])))]
}
