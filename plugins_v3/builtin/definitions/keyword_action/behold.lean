/-- Behold a quality: reveal a card of that quality from your hand or choose a permanent of that
quality you control. [CR#701.4a] -/
action behold (quality : Predicate) : Action := {
  lexical := { verbs := [{ lemma := "behold", past := "beheld", frames := [Monotransitive] }] },
  spelling := "behold <quality>",
  body :=
    chooseOne
      (expose .reveal
        (cards
          (a
            (and
              [isCard, quality,
                in_ (handOf actor)]))))
      (choose
        (a
          (and
            [quality, isPermanent, actorControls])))
}
