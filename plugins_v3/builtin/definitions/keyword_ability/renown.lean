/-- "Renown N" with its reminder text: "When this creature deals combat damage to a player, if it
isn't renowned, put N +1/+1 counters on it and it becomes renowned." [CR#702.112a] -/
keyword renown (count : Amount) : Ability := {
  lexical := { lemma := "renown", parameters := [[Amount]] },
  spelling := "renown <count>",
  body :=
    [ when (does thisCreature (dealDamage (to := a .isPlayer) (kind := some .combat)))
        (if_ := some (not (is thisCreature (hasDesignation renowned))))
        (sequentially
          [ putCounters count p1p1Counter thisCreature,
            gainDesignation thisCreature renowned ]) ]
}
