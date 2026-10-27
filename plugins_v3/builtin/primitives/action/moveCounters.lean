/-- "move <amount> [<kind>] counters from <source> onto <destination>": the all-or-nothing move
(`Action.moveCounters`), a removal from the source and a placement on the destination
[CR#122.5]. With no kind written, what is put is counters of the kind removed (`those`). -/
-- [CR#122.5]
primitive moveCounters (amount : Amount) (source destination : NounPhrase)
    (kind : Option CounterKindSource := none) : Action := {
  -- TODO(spelling): Context-dependent wording: move <amount> [<kind>] counters from <source> onto <destination>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.moveCounters amount kind source destination
}
