keyword forecast (ability : Ability) : Ability := {
  lexical := { lemma := "forecast", parameters := [[DashAbility]] },
  spelling := "forecast — <ability>",
  body :=
    [qualified ability
        (some
          (duringPart .upkeep
            (some (ownerOf this_))))
        (some oncePerTurn) none (some .hand)]
}
