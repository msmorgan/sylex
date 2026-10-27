template addSubtype (subject : NounPhrase) (subtype : Subtype) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Templates.StaticSpec.characteristicChange subject
      [Semantics.Templates.CharacteristicEdit.characteristicEditTypeLine Semantics.QualityOp.adds { subtypes := Option.some [subtype] }]
}
