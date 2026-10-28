/-- "<ability> triggers [by <cause>]": the ability is the subject of `trigger` [CR#603.2]; whose
ability it is ("a triggered ability of a creature you control") is `Predicate.abilityOf` in its
description, and what made it trigger is a cause. -/
-- [CR#603.2]
template triggers (ability : NounPhrase) (by_ : Option NounPhrase := none) : GameEvent := {
  -- TODO(spelling): Context-dependent wording: <ability> triggers [by <cause>]; string substitution cannot express its omitted or inflected parts.
  body :=
    match by_ with
    | none => does (actor ability) trigger
    | some cause =>
      causes (bySource cause)
        (does (actor ability) trigger)
}
