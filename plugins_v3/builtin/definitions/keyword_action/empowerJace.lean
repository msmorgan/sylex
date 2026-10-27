/-- "empower Jace N" (`Definition`, [CR#701.71a]): "If you don't control a Jace planeswalker token,
create a blue Jace planeswalker token with 0 loyalty, '[−1]: Surveil 1,' and '[−3]: Draw a card.'
Choose a Jace planeswalker token you control. Put N loyalty counters on it." -/
action empowerJace (amount : Amount) : Action := {
  lexical := { verbs := [{ lemma := "empower Jace", third_singular := "empowers Jace", past := "empowered Jace", gerund := "empowering Jace", frames := [Amount] }] },
  spelling := "empower Jace <amount>",
  body :=
    let jaceToken : Predicate := and [.isToken, isPlaneswalker, .hasSubtype jace, actorControls]
    sequentially
        [ doIf (not (exists_ jaceToken))
            (create 1 (written
              { characteristics :=
                  { colors := [.blue], types := [.planeswalker], subtypes := [jace],
                    loyalty := some 0,
                    text :=
                      [ activated (loyaltySymbol (.down 1)) (surveil 1),
                        activated (loyaltySymbol (.down 3)) (draw 1) ] } }))
            none,
          choose (a jaceToken),
          putCounters amount loyaltyCounter it ]
}
