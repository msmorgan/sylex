keyword forecast (ability : Ability) : Ability := {
  lexical := { lemma := "forecast", parameters := [[DashAbility]] },
  spelling := "forecast — <ability>",
  body :=
    [ .qualified ability (duringPart .upkeep (some (ownerOf this_)))
        oncePerTurn none (some .hand) ]
}
