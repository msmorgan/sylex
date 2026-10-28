/-- "Power-up — [Cost]: [Effect]" means "[Cost]: [Effect]. If this permanent entered this turn, this
ability's cost is reduced by this permanent's mana cost. Activate this ability only once."
[CR#702.193a] -/
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
