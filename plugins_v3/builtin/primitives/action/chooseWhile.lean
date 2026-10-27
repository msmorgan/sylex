/-- "choose <subject> as you <event>" -/
primitive chooseWhile (subject : NounPhrase) (while_ : Timing) : Action := {
  -- TODO(spelling): Context-dependent wording: choose <subject> as you <event>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.choose Option.none subject Semantics.Disclosure.openly (Option.some while_)
}
