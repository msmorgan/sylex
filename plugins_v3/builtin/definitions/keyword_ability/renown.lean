/-- "Renown N" with its reminder text: "When this creature deals combat damage to a player, if it
isn't renowned, put N +1/+1 counters on it and it becomes renowned." [CR#702.112a] -/
keyword renown (count : Amount) : Ability := {
  lexical := { lemma := "renown", parameters := [[Amount]] },
  spelling := "renown <count>",
  body :=
    [when
        (does (actor thisCreature)
          (dealDamage anyAmount (a isPlayer)
            (some Semantics.DamageKind.combat)))
        (sequentially
          [putCounters count (printedKind p1p1Counter)
              thisCreature,
            gainDesignation thisCreature renowned])
        [] [] none none none
        (some
          (not
            (is thisCreature
              (hasDesignation renowned))))]
}
