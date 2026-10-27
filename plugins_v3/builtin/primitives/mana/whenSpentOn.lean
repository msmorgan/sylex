/-- "When that mana is spent to cast <spell>, <says>"; "that spell <says>" (`mode`). -/
primitive whenSpentOn (mode : SpentMode) (only : Bool) (spell : NounPhrase) (says : Action) :
    ManaRider := {
  -- TODO(spelling): When that mana is spent to cast <spell>, <says>; wording varies with mode, only, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.ManaRider.onSpent mode only spell says
}
