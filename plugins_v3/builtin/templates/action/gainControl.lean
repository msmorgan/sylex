/-- "[<controller>] gain[s] control of <subject> [duration]": the performer by default, so "gain
control of target creature" and "target player gains control of …" (`instruct`) read alike
[CR#613.1b]. -/
-- [CR#613.1b]
template gainControl (subject : NounPhrase) (duration : Option Duration := none)
    (controller : NounPhrase := theActor) : Action := {
  -- TODO(spelling): Context-dependent wording: [<controller>] gain[s] control of <subject> [duration]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.establish (Semantics.Templates.StaticSpec.controlGrant controller subject) duration
}
