/-- "<action>. <forbidden> can't happen this turn": the action, then a prohibition of
`forbidden` until end of turn, its patient read back from the action ("It can't be
regenerated"). -/
template doAndForbid (action forbidden : Action) : Action := {
  spelling := "<action>. <forbidden> can't happen this turn",
  body :=
    Semantics.Templates.Action.sequentially
      [action,
        Semantics.Templates.Action.establish (Semantics.Templates.StaticSpec.staticSpecCant Semantics.Templates.Subject.any forbidden { })
          (Option.some Semantics.Templates.Duration.untilEndOfTurn)]
}
