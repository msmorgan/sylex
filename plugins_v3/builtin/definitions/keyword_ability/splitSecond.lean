/-- "Split second" means "As long as this spell is on the stack, players can't cast other spells or
activate abilities that aren't mana abilities." [CR#702.61a] -/
keyword splitSecond : Ability := {
  lexical := { lemma := "split second" },
  spelling := "split second",
  body :=
    [static
        (onlyWhile
          (conjunction [cant (actor (bare isPlayer))
                (cast (bare anotherSpell)),
              cant (actor (bare isPlayer))
                (activate
                  (allOf
                    (and
                      [abilityHead anyActivated,
                        non isManaAbility])))] none)
          (is thisSpell (in_ stack)))]
}
