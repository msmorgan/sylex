keyword conspire : Ability := {
  lexical := { lemma := "conspire" },
  spelling := "conspire",
  body :=
    [ static
        (.addedCost
          (.perform
            (tap
              (counted (exactly 2)
                (and [isUntapped, isCreature, youControl, .shares .color .share thisSpell]))))
          true .once),
      when (does you (cast thisSpell))
        (if_ := keywordCostPaid conspire thisSpell)
        (sequentially
          [ copyOnStack thisSpell,
            may (.chooseNewTargets (that .copy)) ]) ]
}
