/-- "Hexproof from [quality]": "This permanent can't be the target of [quality] spells your
opponents control or abilities your opponents control from [quality] sources." It is a hexproof
ability, so it is written as hexproof with a quality (`Definition`, [CR#702.11d]). -/
keyword hexproofFrom (quality : Predicate) : Ability := {
  lexical := { lemma := "hexproof", parameters := [[], [Quality (Some ("from"))]] },
  label := "hexproof",
  spelling := "hexproof from <quality>",
  body :=
    [ static
        (cant
          (by_
            (allOf
              (and
                [ or
                    [ and [isSpell, quality],
                      abilityOf (allOf (and [isSource, quality])) ],
                  opponentsControl ])))
          (target thisPermanent)) ]
}
