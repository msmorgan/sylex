/-- "<objects>" a player may look at -/
primitive objects (objects : NounPhrase) : VisibleThing := {
  spelling := "<objects>",
  body :=
    Semantics.Primitives.VisibleThing.objects objects
}
