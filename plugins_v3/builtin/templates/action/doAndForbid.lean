/-- "<action>. <forbidden> can't happen this turn": the action, then a prohibition of
`forbidden` until end of turn, its patient read back from the action ("It can't be
regenerated"). -/
template doAndForbid (action forbidden : Action) : Action := {
  spelling := "<action>. <forbidden> can't happen this turn",
  body :=
    sequentially
      [action,
        establish (staticSpecCant any forbidden { })
          (some untilEndOfTurn)]
}
