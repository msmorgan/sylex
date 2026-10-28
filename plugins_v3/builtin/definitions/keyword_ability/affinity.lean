/-- "Affinity for [text]" means "This spell costs {1} less to cast for each [text] you control."
[CR#702.41a] -/
keyword affinity (quality : Predicate) : Ability := {
  lexical := { lemma := "affinity", parameters := [[Quality (Some ("for"))]] },
  spelling := "affinity for <quality>",
  body :=
    [static
        (costShift thisSpell
          (generic
            (down
              (forEach 1 (and [quality, youControl])))
            none))]
}
