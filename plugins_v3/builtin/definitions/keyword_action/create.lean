/-- "create <count> <token> [riders]": the tokens are put onto the battlefield, entering under
their creator's control (`Definition`, [CR#701.7a,111.2]). -/
action create (count : Amount) (token : TokenSpec) (riders : List MoveRider := []) :
    Action := {
  -- TODO(spelling): create <count> <token> [<riders>]; string substitution cannot branch on
  -- optional fields or supply contextual agreement and morphology.
  lexical := { verbs := [{ lemma := "create", frames := [Monotransitive] }] },
  body :=
    createObject count (CreationSpec.token token riders) none
}
