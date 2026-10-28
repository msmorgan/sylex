/-- "Forecast — [Activated ability]": an activated ability that can be activated only from its
owner's hand, only during that player's upkeep and only once each turn [CR#702.57a,702.57b] -/
keyword forecast (ability : Ability) : Ability := {
  lexical := { lemma := "forecast", parameters := [[DashAbility]] },
  spelling := "forecast — <ability>",
  body :=
    [qualified ability
        (some
          (duringPart upkeep
            (some (ownerOf this_))))
        (some oncePerTurn) none (some hand)]
}
