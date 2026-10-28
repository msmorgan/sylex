/-- "Conspire" means "As an additional cost to cast this spell, you may tap two untapped creatures
you control that each share a color with it" and "When you cast this spell, if its conspire cost
was paid, copy it. If the spell has any targets, you may choose new targets for the copy."
[CR#702.78a] -/
keyword conspire : Ability := {
  lexical := { lemma := "conspire" },
  spelling := "conspire",
  body :=
    [static
        (addedCost
          (perform
            (tap
              (counted (exactly 2)
                (and
                  [isUntapped, isCreature, youControl,
                    shares .color .share
                      thisSpell]))))
          true .once),
      when
        (does (actor you)
          (cast thisSpell))
        (sequentially
          [copyOnStack thisSpell,
            may (chooseNewTargets (that .copy))])
        [] [] none none none
        (some
          (keywordCostPaid conspire.label thisSpell))]
}
