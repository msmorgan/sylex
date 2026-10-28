/-- "<subject> is a <subtype> in addition to its other types": a type-changing
effect that adds one subtype [CR#205.1b], applied in the type-changing layer
[CR#613.1d]. -/
template addSubtype (subject : NounPhrase) (subtype : Subtype) : StaticSpec := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    characteristicChange subject
      [typeLine .adds { subtypes := some [subtype] }]
}
