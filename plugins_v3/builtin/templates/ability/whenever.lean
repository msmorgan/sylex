/-- "Whenever <event>, <action>" -/
template whenever (event : GameEvent) (action : Action)
    (or_ : List GameEvent := []) (joins : List JoinedHeader := [])
    (while_ : Option Timing := none) (during : Option Timing := none)
    (limit : Option UsageLimit := none) (if_ : Option Condition := none) : Ability := {
  -- TODO(spelling): Include or omit the clauses supplied by while_, during, limit, if_. Format and
  -- coordinate the or_, joins list entries, including the empty case.
  body := triggered event or_ while_ joins during limit if_ action
}
