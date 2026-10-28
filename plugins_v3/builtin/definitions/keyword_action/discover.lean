/-- "<performer> discovers N" (`Definition`, [CR#701.57a]). -/
action discover (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "discover", frames := [Amount, Intransitive] }] },
  spelling := "discover <amount>",
  body :=
    have fits :=
      compare atMost amount [.stat manaValue];
    sequentially
      [exile
          (run top
            (and [non isLand, fits])
            (libraryOf actor)),
        withContinuation .optional
          (castWithoutPaying (that card) (some fits))
          none
          (some
            (putInto (that card) hand)),
        move theRest wherever
          (some onBottom) .arrival
          [inOrder randomOrder]]
}
