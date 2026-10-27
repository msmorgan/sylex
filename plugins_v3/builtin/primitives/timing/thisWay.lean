/-- "… this way": the event the enclosing action caused. -/
primitive thisWay : LookbackSpan := {
  -- TODO(spelling): Context-dependent wording: … this way; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.LookbackSpan.thisWay
}
