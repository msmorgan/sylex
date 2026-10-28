/-- A Text Change of `subject`'s rules text, as `change` names it. -/
primitive textChange (subject : NounPhrase) (change : TextChange) : StaticSpec := {
  body :=
    Semantics.Primitives.StaticSpec.textChange subject change
}
