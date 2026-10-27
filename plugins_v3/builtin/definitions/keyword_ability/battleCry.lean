keyword battleCry : Ability := {
  lexical := { lemma := "battle cry" },
  spelling := "battle cry",
  body :=
    [ whenever (does thisCreature (.attack none))
        (establishThroughout
          (Templates.getsPt (each (and [isCreature, isAttacking, isOther]))
            (.up 1) (.up 0))
          untilEndOfTurn) ]
}
