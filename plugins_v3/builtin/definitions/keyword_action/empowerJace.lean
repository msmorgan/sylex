/-- "empower Jace N" (`Definition`, [CR#701.71a]): "If you don't control a Jace planeswalker token,
create a blue Jace planeswalker token with 0 loyalty, '[−1]: Surveil 1,' and '[−3]: Draw a card.'
Choose a Jace planeswalker token you control. Put N loyalty counters on it." -/
action empowerJace (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "empower Jace", third_singular := "empowers Jace", past := "empowered Jace", gerund := "empowering Jace", frames := [Amount] }] },
  spelling := "empower Jace <amount>",
  body :=
    have jaceToken :=
      and
        [isToken, isPlaneswalker,
          hasSubtype jace, actorControls];
    sequentially
      [doIf (not (exists_ jaceToken))
          (create 1
            (written
              {
                characteristics :=
                  { colors := [Semantics.Color.blue], types := [Semantics.CardType.planeswalker],
                    subtypes := [jace],
                    text :=
                      [activated (loyaltySymbol (down 1))
                          (surveil 1),
                        activated (loyaltySymbol (down 3))
                          (draw 1)],
                    loyalty := some 0 } })),
        choose (a jaceToken),
        putCounters amount (printedKind loyaltyCounter)
          it]
}
