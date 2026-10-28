/-- A Mana Substitute's act pays one mana of a color: convoke's "colored mana … tap an untapped
creature of that color" [CR#702.51a]. -/
primitive ofColor (color : ColorTerm) : PipPaid := {
  body :=
    Semantics.Primitives.PipPaid.ofColor color
}
