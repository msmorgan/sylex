keyword craft (cost : Cost) : Ability := {
  lexical := {
    lemma := "craft",
    parameters := [[Quality (Some ("with")), Cost]],
    verbs := [{ lemma := "craft", frames := [Monotransitive] }]
  },
  -- TODO(spelling): Craft with [materials] [cost]; the Cost combines material selection and
  -- payment, which require structural decomposition.
  body :=
    []
}
