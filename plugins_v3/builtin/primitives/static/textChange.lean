/-- A Text Change of `subject`'s rules text, as `change` names it. -/
primitive textChange (subject : NounPhrase) (change : TextChange) : StaticSpec := {
  -- TODO(spelling): A Text Change of `subject`'s rules text, as `change` names it.
  body :=
    Semantics.Primitives.StaticSpec.textChange subject change
}
