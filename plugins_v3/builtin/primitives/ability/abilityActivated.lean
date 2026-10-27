primitive abilityActivated (cost : Semantics.Cost) (action : Semantics.Action) (timing : Option Semantics.Timing) (limit : Option Semantics.UsageLimit) (guard : Option Semantics.Condition) (activator : Option Semantics.NounPhrase) : Semantics.Ability := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.Ability.activated cost action timing limit guard activator
}
