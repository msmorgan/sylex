/-- "<performer> discovers N" (`Definition`, [CR#701.57a]). -/
action discover (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "discover", frames := [Amount, Intransitive] }] },
  spelling := "discover <amount>",
  body :=
    let fits : Predicate := .compare [.stat .manaValue] .atMost amount
    sequentially
      [ exile (.run .top (and [non isLand, fits]) (libraryOf .actor)),
        .withContinuation .optional (castWithoutPaying (that .card) fits) none
          (putInto (that .card) hand),
        .move theRest .anyLocation onBottom .arrival
          [(.inOrder .randomOrder)] ]
}
