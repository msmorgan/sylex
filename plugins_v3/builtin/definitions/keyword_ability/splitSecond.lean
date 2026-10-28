keyword splitSecond : Ability := {
  lexical := { lemma := "split second" },
  spelling := "split second",
  body :=
    [static
        (onlyWhile
          (staticSpecConjunction none
            [cant (actor (bare isPlayer))
                (cast (bare anotherSpell)),
              cant (actor (bare isPlayer))
                (activate
                  (allOf
                    (and
                      [abilityHead .anyActivated,
                        non isManaAbility])))])
          (is thisSpell (in_ stack)))]
}
