/-- "Poisonous N" means "Whenever this creature deals combat damage to a player, that player gets N
poison counters." [CR#702.70a] -/
keyword poisonous (amount : Amount) : Ability := {
  lexical := { lemma := "poisonous", parameters := [[Amount]] },
  spelling := "poisonous <amount>",
  body :=
    [whenever
        (does (actor thisCreature)
          (dealDamage anyAmount (a isPlayer)
            (some .combat)))
        (putCounters amount (printedKind poison)
          they)]
}
