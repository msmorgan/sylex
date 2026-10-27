/-- "except its name is <name>" -/
template isNamed (name : String) : CopyExcept := {
  spelling := "except its name is <name>",
  body :=
    Semantics.Templates.CopyExcept.copyExceptEdits [Semantics.Templates.CharacteristicEdit.characteristicEditName Semantics.QualityOp.sets name]
}
