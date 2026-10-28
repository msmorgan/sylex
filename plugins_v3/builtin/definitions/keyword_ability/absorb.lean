/-- "Absorb N": "If a source would deal damage to this creature, prevent N of that damage."
[CR#702.64a] -/
keyword absorb (amount : Amount) : Ability := {
  lexical := { lemma := "absorb", parameters := [[Amount]] },
  spelling := "absorb <amount>",
  body :=
    [static
        (replace
          (would (actor (a isSource))
            (dealDamage anyAmount thisCreature))
          (instead (preventDamage (ofThatDamage amount))))]
}
