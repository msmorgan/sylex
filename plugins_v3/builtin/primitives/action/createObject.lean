/-- Create the specified objects, optionally naming their controller. -/
primitive createObject (count : Amount) (spec : CreationSpec)
    (under : Option NounPhrase := none) : Action := {
  body :=
    Semantics.Primitives.Action.createObject count spec under
}
