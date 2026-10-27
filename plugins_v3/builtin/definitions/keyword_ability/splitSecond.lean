keyword splitSecond : Ability := {
  lexical := { lemma := "split second" },
  spelling := "split second",
  body :=
    [ static
        (onlyWhile
          (.conjunction none
            [ cant (bare isPlayer) (cast (bare anotherSpell)),
              cant (bare isPlayer)
                (activate (allOf (and [.abilityHead .anyActivated, non isManaAbility]))) ])
          (is thisSpell (.in_ stack))) ]
}
