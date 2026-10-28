keyword poisonous (amount : Amount) : Ability := {
  lexical := { lemma := "poisonous", parameters := [[Amount]] },
  spelling := "poisonous <amount>",
  body :=
    [whenever
        (does (actor thisCreature)
          (dealDamage anyAmount (a isPlayer)
            (some Semantics.DamageKind.combat)))
        (putCounters amount (printedKind poison)
          they)]
}
