/-- "except its name is <name>" -/
template isNamed (name : String) : CopyExcept := {
  spelling := "except its name is <name>",
  body :=
    copyExceptEdits [characteristicEditName .sets name]
}
