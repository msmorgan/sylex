/-- "Exhaust — [Cost]: [Effect]" means "[Cost]: [Effect]. Activate only once." [CR#702.177a] -/
keyword exhaust (ability : Ability) : Ability := {
  lexical := { lemma := "exhaust", parameters := [[DashAbility]] },
  spelling := "exhaust — <ability>",
  body :=
    [qualified ability none (some oncePerObject) none
        none]
}
