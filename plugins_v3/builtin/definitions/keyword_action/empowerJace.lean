/-- "empower Jace N" (`Definition`, [CR#701.71a]): "If you don't control a Jace planeswalker token,
create a blue Jace planeswalker token with 0 loyalty, '[−1]: Surveil 1,' and '[−3]: Draw a card.'
Choose a Jace planeswalker token you control. Put N loyalty counters on it." -/
action empowerJace (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "empower Jace", third_singular := "empowers Jace", past := "empowered Jace", gerund := "empowering Jace", frames := [Amount] }] },
  spelling := "empower Jace <amount>",
  body :=
    have jaceToken :=
      Semantics.Templates.Predicate.and
        [Semantics.Templates.Predicate.isToken, Semantics.Templates.Predicate.isPlaneswalker,
          Semantics.Templates.Predicate.hasSubtype Semantics.Definitions.jace, Semantics.Templates.Predicate.actorControls];
    Semantics.Templates.Action.sequentially
      [Semantics.Templates.Action.doIf (Semantics.Templates.Condition.not (Semantics.Templates.Condition.exists_ jaceToken))
          (Semantics.Definitions.create 1
            (Semantics.Templates.TokenSpec.written
              {
                characteristics :=
                  { colors := [Semantics.Color.blue], types := [Semantics.CardType.planeswalker],
                    subtypes := [Semantics.Definitions.jace],
                    text :=
                      [Semantics.Templates.Ability.activated (Semantics.Templates.Cost.loyaltySymbol (Semantics.Templates.down 1))
                          (Semantics.Definitions.surveil 1),
                        Semantics.Templates.Ability.activated (Semantics.Templates.Cost.loyaltySymbol (Semantics.Templates.down 3))
                          (Semantics.Templates.Action.draw 1)],
                    loyalty := Option.some 0 } })),
        Semantics.Templates.Action.choose (Semantics.Templates.NounPhrase.a jaceToken),
        Semantics.Templates.Action.putCounters amount (Semantics.Templates.CounterKindSource.printedKind Semantics.Definitions.loyaltyCounter)
          Semantics.Templates.NounPhrase.it]
}
