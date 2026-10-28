/-- "As <subject> becomes attached, choose <chosen>": an Off-Stack Static whose choice is made as
it becomes attached. -/
template attachmentChoice (subject chosen : NounPhrase) : StaticSpec := {
  spelling := "As <subject> becomes attached, choose <chosen>",
  body :=
    asEvent
      (does Subject.any
        (attachment Semantics.AttachMove.attached subject none))
      (actionChoose none chosen Semantics.Disclosure.openly none)
}
