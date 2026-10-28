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
