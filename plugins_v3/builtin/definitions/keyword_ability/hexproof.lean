/-- "Hexproof": "This permanent can't be the target of spells or abilities your opponents
control," and on a player "You can't be the target of …" [CR#702.11b,702.11c], so the body is
written over its holder (`this_`). -/
keyword hexproof : Ability := {
  lexical := { lemma := "hexproof", parameters := [[], [Quality (Some ("from"))]] },
  spelling := "hexproof",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.cant
          (Semantics.Templates.Subject.by_
            (Semantics.Templates.NounPhrase.allOf
              (Semantics.Templates.Predicate.and
                [Semantics.Templates.Predicate.in_ Semantics.Templates.Location.stack, Semantics.Templates.Predicate.opponentsControl])))
          (Semantics.Templates.Action.target Semantics.Templates.NounPhrase.this_))]
}
