/-- The Characteristic Edit one stat's delta writes: raised, lowered, or set. -/
template statEdit (axis : CharacteristicStat) (delta : Delta Amount) :
    CharacteristicEdit := {
  body :=
    match delta with
    | Semantics.Delta.up amount => stat adds axis amount
    | Semantics.Delta.down amount => stat loses axis amount
    | Semantics.Delta.set amount => stat sets axis amount
}
