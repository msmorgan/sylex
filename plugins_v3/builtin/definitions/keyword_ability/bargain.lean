/-- "Bargain" means "As an additional cost to cast this spell, you may sacrifice an artifact,
enchantment, or token." [CR#702.166a] -/
keyword bargain : Ability := {
  lexical := { lemma := "bargain", verbs := [{ lemma := "bargain", frames := [Monotransitive] }] },
  spelling := "bargain",
  body :=
    [static
        (addedCost
          (perform
            (sacrifice
              (a
                (or
                  [and [isArtifact, isPermanent],
                    and
                      [isEnchantment, isPermanent],
                    isToken]))))
          true .once)]
}
