/-- "instead": the word marking `action` as what happens in the replaced event's place
("draw two cards instead", "you may instead mill …") [CR#614.1a]. Cards write a conditional
replacement in Oracle word order, `doIf c (instead a)`. -/
-- [CR#614.1a]
primitive instead (action : Action) : Replacement := {
  -- TODO(spelling): instead; wording varies with action, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.Replacement.instead action
}
