/-- A joined "whenever <event> while <concurrent>" header. -/
template joinedHeadWhile (event : GameEvent) (while_ : Timing) : JoinedHeader := {
  -- TODO(spelling): Context-dependent wording: whenever <event> while <concurrent>; string substitution cannot express its omitted or inflected parts.
  body :=
    { event := event, alternatives := [], while_ := Option.some while_, timing := Option.none }
}
