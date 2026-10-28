/-- "Hexproof": "This permanent can't be the target of spells or abilities your opponents
control," and on a player "You can't be the target of …" [CR#702.11b,702.11c], so the body is
written over its holder (`this_`). -/
keyword hexproof : Ability := {
  lexical := { lemma := "hexproof", parameters := [[], [Quality (Some ("from"))]] },
  spelling := "hexproof",
  body :=
    [static
        (cant
          (by_
            (allOf
              (and
                [in_ stack, opponentsControl])))
          (target this_))]
}
