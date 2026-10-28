/-- Copy exceptions retain their own edit block; added abilities remain copy-specific. -/
template copyCharacteristics (c : Characteristics) (typesAdded : Bool) : List CopyExcept := {
  body :=
    have typeOp := if typesAdded = true then Semantics.QualityOp.adds else Semantics.QualityOp.sets;
    have types :=
      if
          (List.isEmpty (Semantics.Characteristics.supertypes c) && List.isEmpty (Semantics.Characteristics.types c) &&
              List.isEmpty (Semantics.Characteristics.subtypes c)) =
            true then
        []
      else [typeLine typeOp (Semantics.Characteristics.typeChanges c)];
    have values :=
      Semantics.Characteristics.valueEdits
        { name := Semantics.Characteristics.name c, cost := Semantics.Characteristics.cost c,
          colors := Semantics.Characteristics.colors c, supertypes := Semantics.Characteristics.supertypes c,
          types := Semantics.Characteristics.types c, subtypes := Semantics.Characteristics.subtypes c,
          power := Semantics.Characteristics.power c, toughness := Semantics.Characteristics.toughness c,
          loyalty := Semantics.Characteristics.loyalty c, defense := Semantics.Characteristics.defense c }
        Semantics.QualityOp.sets;
    have edits := types ++ values;
    (if List.isEmpty edits = true then [] else [edits edits]) ++
      List.map alsoHas (Semantics.Characteristics.text c)
}
