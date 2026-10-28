/-- "Whenever <event>, <action>" -/
template whenever (event : GameEvent) (action : Action)
    (or_ : List GameEvent := []) (joins : List JoinedHeader := [])
    (while_ : Option Timing := none) (during : Option Timing := none)
    (limit : Option UsageLimit := none) (if_ : Option Condition := none) : Ability := {
  -- TODO(spelling): Whenever <event>, <action>; wording varies with during, if_, joins, limit, or_, while_, requiring optional clauses, inflection or structure-dependent phrasing.
  body := triggered event or_ while_ joins during limit if_ action
}
