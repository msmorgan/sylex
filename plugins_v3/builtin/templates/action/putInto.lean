/-- "put <subject> into <destination> [from <from_>] [<riders>]", as "in any order" or "under
its owner's control". -/
template putInto (subject : NounPhrase) (destination : Location)
    (from_ : Option Location := none) (riders : List MoveRider := []) : Action := {
  -- TODO(spelling): Context-dependent wording: put <subject> into <destination> [from <from_>] [<riders>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.Action.zoneChange subject destination (Option.getD from_ Semantics.Templates.Location.wherever) riders
}
