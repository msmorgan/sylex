/-- "create <count> <token> [riders]": the tokens are put onto the battlefield, entering under
their creator's control (`Definition`, [CR#701.7a,111.2]). -/
action create (count : Amount) (token : TokenSpec) (riders : List MoveRider := []) :
    Action := {
  -- TODO(spelling): Format and coordinate the riders list entries, including the empty case.
  lexical := { verbs := [{ lemma := "create", frames := [Monotransitive] }] },
  body :=
    createObject count (Templates.CreationSpec.token token riders) none
}
