action behold (quality : Predicate) : Action := {
  lexical := { verbs := [{ lemma := "behold", past := "beheld", frames := [Monotransitive] }] },
  spelling := "behold <quality>",
  body :=
    chooseOne
        (.expose .reveal (.cards (a (and [.isCard, quality, .in_ (handOf .actor)]))))
        (choose (a (and [quality, isPermanent, actorControls])))
}
