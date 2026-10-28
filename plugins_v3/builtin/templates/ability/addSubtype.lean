template addSubtype (subject : NounPhrase) (subtype : Subtype) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    characteristicChange subject
      [typeLine .adds { subtypes := some [subtype] }]
}
