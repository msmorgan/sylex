/-- A special action that pays `cost` and performs `does` without using the stack [CR#116.1],
subject to the supplied timing restriction [CR#116.2c]. -/
primitive specialAction (when : Option Timing) (cost : Cost) (does : Action)
    (player : Option NounPhrase := none) : StaticSpec := {
  body := Semantics.Primitives.StaticSpec.specialAction player when cost does
}
