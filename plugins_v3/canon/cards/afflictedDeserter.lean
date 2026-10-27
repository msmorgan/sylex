/-- Afflicted Deserter // Werewolf Ransacker -/
card afflictedDeserter : Spelled := { body := .transforming
  { characteristics :=
    { name := "Afflicted Deserter", cost := [generic 3, pip .red], types := [.creature],
      subtypes := [human, werewolf],
      text :=
        [ at_ (beginningOfEach .upkeep)
            (if_ := some (not (did any (cast (a isSpell)) lastTurn)))
            (transform thisCreature) ],
      power := stat 3, toughness := stat 2 } }
  { characteristics :=
    { name := "Werewolf Ransacker", types := [.creature], subtypes := [werewolf],
      text :=
        [ whenever
            (does any (transform thisCreature (into := named "Werewolf Ransacker")))
            (sequentially
              [ may (destroy (target isArtifact)),
                doIf (did (that artifact) (putInto theActor graveyard) thisWay)
                  (instruct thisCreature (dealDamage 3 (controllerOf (that artifact)))) ]),
          at_ (beginningOfEach .upkeep)
            (if_ := (compareAmt
                  (aggregateOver .max isPlayer (eventCount (cast (a isSpell)) they lastTurn))
                    .atLeast 2))
            (transform thisCreature) ],
      power := stat 5, toughness := stat 4 } } }
