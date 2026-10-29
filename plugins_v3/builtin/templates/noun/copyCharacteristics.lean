/-- "Except it has <c>" over one set of characteristics, its types "in addition to its other types"
when `typesAdded`: each written value set and each written ability added. -/
template copyCharacteristics (c : Characteristics) (typesAdded : Bool := false) : List CopyExcept := {
  body :=
    [characteristics c typesAdded]
}
