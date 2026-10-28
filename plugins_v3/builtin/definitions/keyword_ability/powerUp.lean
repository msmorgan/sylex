keyword powerUp (ability : Ability) : Ability := {
  lexical := { lemma := "power-up", parameters := [[DashAbility]] },
  spelling := "power-up — <ability>",
  body :=
    [qualified ability none (some oncePerObject) none
        none,
      static
        (onlyWhile
          (reducedBySymbols thisAbility
            (manaCostOf thisPermanent))
          (did (actor thisPermanent)
            enter (within soFarThisTurn)))]
}
