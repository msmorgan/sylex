/-- "<cost>: <action>" [CR#602.1]. -/
primitive activated (cost : Cost) (action : Action) (timing : Option Timing := none)
    (limit : Option UsageLimit := none) (guard : Option Condition := none)
    (activator : Option NounPhrase := none) : Ability := {
  spelling := "<cost>: <action>",
  body := Semantics.Primitives.Ability.activated cost action timing limit guard activator
}
