/-- Merge the subject on top of or under the permanent [CR#730.2]. -/
primitive merge (subject onto : NounPhrase) (position : MergePosition) : Action := {
  body :=
    Semantics.Primitives.Action.merge subject onto position
}
