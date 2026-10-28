/-- "As <subject> becomes attached, choose <chosen>": an Off-Stack Static whose choice is made as
it becomes attached. -/
template attachmentChoice (subject chosen : NounPhrase) : StaticSpec := {
  spelling := "As <subject> becomes attached, choose <chosen>",
  body :=
    asEvent
      (does Templates.Subject.any
        (attachment .attached subject none))
      (actionChoose none chosen .openly none)
}
