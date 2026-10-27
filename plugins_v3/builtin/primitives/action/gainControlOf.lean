/-- "gain control of <subject> [from <from_>]" as an event or a forbidden act: "whenever an
opponent gains control of a permanent from you", a trigger that looks back in time [CR#603.10d],
or "can't gain control of them". The imperative is `gainControl`. -/
-- [CR#603.10d]
primitive gainControlOf (subject : NounPhrase) (from_ : Option NounPhrase := none) :
    Action := {
  -- TODO(spelling): Context-dependent wording: gain control of <subject> [from <from_>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.changeControl subject from_
}
