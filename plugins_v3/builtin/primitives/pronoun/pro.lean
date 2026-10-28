primitive pro (reach : Reach) (plurality : Plurality) : NounPhrase := {
  -- TODO(spelling): Pronoun form depends on reach, number, and the antecedent's grammatical features.
  body := Semantics.Primitives.NounPhrase.pro reach plurality
}
