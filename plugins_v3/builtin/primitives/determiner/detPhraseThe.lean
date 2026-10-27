primitive detPhraseThe (plur : optParam Semantics.Plurality Semantics.Plurality.one) : Semantics.DetPhrase := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.DetPhrase.the plur
}
