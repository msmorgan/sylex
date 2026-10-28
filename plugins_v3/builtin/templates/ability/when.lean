/-- "When <event>, <action>". The trigger words `when`, `whenever` and `at_` expand to
one triggered-ability constructor [CR#603.1,113.3c]. Its optional header parts are alternative
events, joined headers, a concurrent condition, a timing window, a usage limit and an intervening
"if" clause [CR#603.4]. -/
-- [CR#603.1,113.3c,603.4]
template when (event : GameEvent) (action : Action)
    (or_ : List GameEvent := []) (joins : List JoinedHeader := [])
    (while_ : Option Timing := none) (during : Option Timing := none)
    (limit : Option UsageLimit := none) (if_ : Option Condition := none) : Ability := {
  -- TODO(spelling): When <event>, <action>; wording varies with during, if_, joins, limit, or_, while_, requiring optional clauses, inflection or structure-dependent phrasing.
  body := triggered event or_ while_ joins during limit if_ action
}
