/-- "Offspring [cost]" means "You may pay an additional [cost] as you cast this spell" and "When
this permanent enters, if its offspring cost was paid, create a token that's a copy of it,
except it's 1/1." [CR#702.175a] -/
keyword offspring (cost : Cost) : Ability := {
  lexical := { lemma := "offspring", parameters := [[Cost]] },
  spelling := "offspring <cost>",
  body :=
    [static (addedCost cost true once),
      when
        (does (actor thisPermanent) enter)
        (create 1
          (copyOf thisPermanent
            [characteristicEdits
                [stat power (set 1),
                  stat toughness (set 1)]]))
        [] [] none none none
        (some
          (keywordCostPaid offspring.label thisPermanent))]
}
