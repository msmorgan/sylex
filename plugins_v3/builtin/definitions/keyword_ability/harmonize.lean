/-- "Harmonize [cost]" means "You may cast this card from your graveyard by paying [cost] and
tapping up to one untapped creature you control rather than paying this spell's mana cost," "If
you cast this spell using its harmonize ability, its total cost is reduced by an amount of
generic mana equal to the tapped creature's power," and "If the harmonize cost was paid, exile
this card instead of putting it anywhere else any time it would leave the stack." [CR#702.180a] -/
keyword harmonize (cost : Cost) : Ability := {
  lexical := { lemma := "harmonize", parameters := [[Cost]] },
  spelling := "harmonize <cost>",
  body :=
    [static
        (can (actor you)
          (cast this_ (some yourGraveyard)
            (payingInstead
              (compound
                [cost,
                  perform
                    (tap
                      (counted (upTo 1)
                        (and
                          [isUntapped,
                            and
                              [isCreature, youControl]])))])))),
      static
        (onlyWhile
          (costShift thisSpell
            (generic
              (down
                (powerOf
                  (the
                    (and
                      [was (tap which),
                        isCreature]))))
              none))
          (keywordCostPaid harmonize.label thisSpell)),
      static
        (replace
          (would (actor thisSpell)
            (leave stack))
          (doIf
            (keywordCostPaid harmonize.label thisSpell)
            (instead (exile thisSpell))))]
}
