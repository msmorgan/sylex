/-- "<performer> discovers N" (`Definition`, [CR#701.57a]). -/
action discover (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "discover", frames := [Amount, Intransitive] }] },
  spelling := "discover <amount>",
  body :=
    sequentially
      [exile
          (run top
            (and [non isLand, compare atMost amount [.stat manaValue]])
            (libraryOf actor)),
        withContinuation .optional
          (castWithoutPaying (that card)
            (some (compare atMost amount [.stat manaValue])))
          none
          (some
            (putInto (that card) hand)),
        move theRest wherever
          (some onBottom) .arrival
          [inOrder randomOrder]]
}
