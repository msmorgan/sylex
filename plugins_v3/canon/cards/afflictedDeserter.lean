/-- Afflicted Deserter // Werewolf Ransacker -/
card := transforming
  {
    front := {
      name := "Afflicted Deserter",
      cost := [3, red],
      types := [creature],
      subtypes := [human, werewolf],
      text := [
        at_ (beginningOfEach upkeep) (if_ := some (not (did any (cast (a isSpell)) lastTurn)))
          (transform thisCreature)
      ],
      power := 3,
      toughness := 2
    },
    back := {
      name := "Werewolf Ransacker",
      types := [creature],
      subtypes := [werewolf],
      text := [
        whenever (does any (transform thisCreature (into := named "Werewolf Ransacker")))
          [
            may (destroy (target isArtifact)),
            doIf (did (that artifact) (putInto theActor graveyard) thisWay)
              (instruct thisCreature (dealDamage 3 (controllerOf (that artifact))))
          ],
        at_ (beginningOfEach upkeep)
          (if_ :=
             (compareAmt (aggregateOver max isPlayer (eventCount (cast (a isSpell)) they lastTurn))
                atLeast 2))
          (transform thisCreature)
      ],
      power := 5,
      toughness := 4
    }
  }
