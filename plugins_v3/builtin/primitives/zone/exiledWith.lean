/-- "exiled with <source>": the cards its linked ability exiled, face up unless that ability
exiled them face down [CR#406.3,406.6,607.2a]. -/
-- [CR#406.3,406.6,607.2a]
primitive exiledWith (source : NounPhrase) : Location := {
  spelling := "exiled with <source>",
  body :=
    Semantics.Primitives.Location.exiledWith source
}
