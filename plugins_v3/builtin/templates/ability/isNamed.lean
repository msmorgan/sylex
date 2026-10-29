/-- "except its name is <name>" -/
template isNamed (name : String) : CopyExcept := {
  spelling := "except its name is <name>",
  body :=
    characteristicEdits [name sets name]
}
