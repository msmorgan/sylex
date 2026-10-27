/-- "<spell> was <designation>": the spell was a card holding it before it was cast, as "if this
spell was foretold" [CR#702.143c]. -/
-- [CR#702.143c]
primitive wasBeforeCast (designation : DesignationLabel) : Predicate := {
  -- TODO(spelling): Context-dependent wording: <spell> was <designation>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Predicate.heldBeforeCast designation
}
