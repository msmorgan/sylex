primitive dividedVerbDamage (source : Semantics.NounPhrase) : Semantics.DividedVerb := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.DividedVerb.damage source
}
