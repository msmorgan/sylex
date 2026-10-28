/-- "After <roll>, <modify>": a Roll Modifier, applied before the roll's result is final, not a
trigger [CR#706.2,706.2a,113.3c]. `limit` is a printed "only once each turn". -/
-- [CR#706.2,706.2a,113.3c]
template after (roll : GameEvent) (modify : Action) (limit : Option UsageLimit := none) :
    Ability := {
  -- TODO(spelling): After <roll>, <modify>; wording varies with limit, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    static (rollModifier roll modify limit)
}
