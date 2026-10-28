primitive the (plur : optParam Plurality Plurality.one) : DetPhrase := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.DetPhrase.the plur
}
