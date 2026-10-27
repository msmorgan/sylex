primitive activated (cost : Cost) (action : Action) : Ability := {
  spelling := "<cost>: <action>",
  body :=
    Semantics.Primitives.Ability.activated cost action Option.none Option.none Option.none Option.none
}
