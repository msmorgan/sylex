/-- "<Type>cycling <cost>": a cycling ability qualified by the card it searches for, labelled
cycling, since typecycling abilities are cycling abilities [CR#702.29e,702.29f]. "[Cost], Discard
this card: Search your library for a [type] card, reveal it, and put it into your hand. Then
shuffle your library." [CR#702.29e] -/
keyword typecycling (quality : Predicate) (cost : Cost) : Ability := {
  lexical := {
    lemma := "cycling",
    parameters := [[Cost], [DashNonManaCost]],
    bound := { surface := "cycling", stems := [Subtype, CardType, Supertype] },
    verbs := [{ lemma := "cycle", frames := [Monotransitive] }]
  },
  label := "cycling",
  -- TODO(spelling): Islandcycling [cost] / basic landcycling [cost]; the Predicate needs a
  -- lowercase bound stem rather than its ordinary rendered phrase.
  body :=
    [Semantics.Templates.Ability.activated
        (Semantics.Templates.Cost.compound [cost, Semantics.Templates.Cost.perform (Semantics.Definitions.discard Semantics.Templates.NounPhrase.this_)])
        (Semantics.Templates.Action.sequentially
          [Semantics.Definitions.search Semantics.Templates.Location.yourLibrary
              (Semantics.Templates.NounPhrase.a (Semantics.Templates.Predicate.and [Semantics.Templates.Predicate.isCard, quality])),
            Semantics.Definitions.reveal Semantics.Templates.NounPhrase.it,
            Semantics.Templates.Action.putInto Semantics.Templates.NounPhrase.it Semantics.Templates.Location.hand, Semantics.Definitions.shuffle])]
}
