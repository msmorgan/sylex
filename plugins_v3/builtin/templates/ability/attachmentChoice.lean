/-- "As <subject> becomes attached, choose <chosen>": an Off-Stack Static whose choice is made as
it becomes attached. -/
template attachmentChoice (subject chosen : NounPhrase) : StaticSpec := {
  spelling := "As <subject> becomes attached, choose <chosen>",
  body :=
    Semantics.Templates.StaticSpec.asEvent
      (Semantics.Templates.GameEvent.does Semantics.Templates.Subject.any
        (Semantics.Templates.Action.attachment Semantics.AttachMove.attached subject Option.none))
      (Semantics.Templates.Action.actionChoose Option.none chosen Semantics.Disclosure.openly Option.none)
}
