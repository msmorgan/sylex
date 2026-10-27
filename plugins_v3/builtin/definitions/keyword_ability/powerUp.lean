keyword powerUp (ability : Ability) : Ability := {
  lexical := { lemma := "power-up", parameters := [[DashAbility]] },
  spelling := "power-up — <ability>",
  body :=
    [ .qualified ability none oncePerObject none none,
      static
        (onlyWhile (reducedBySymbols thisAbility (.manaCostOf thisPermanent))
          (did thisPermanent enter soFarThisTurn)) ]
}
