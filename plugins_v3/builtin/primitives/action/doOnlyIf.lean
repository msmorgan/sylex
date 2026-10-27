/-- "<action> if <condition>" ("counter target spell if its mana value is X"): the conditional
action with its condition printed after it (`postposed`): the condition reads the action's
targets and its mentions of earlier referents as the clause ranks them ("the creature you control
gets +2/+1 until end of turn if it's a Knight"), but nothing the action would choose or create
as it resolves [CR#601.2c,608.2c]. -/
-- [CR#601.2c,608.2c]
primitive doOnlyIf (action : Action) (condition : Condition)
    (otherwise : Option Action := none) : Action := {
  -- TODO(spelling): <action> if <condition>; wording varies with otherwise, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.Action.doIf condition action otherwise Bool.true
}
