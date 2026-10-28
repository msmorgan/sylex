/-- "<the ability, for the first keyword>. The same is true for <the other keywords>.": one ability
[CR#113.2c] that does, for each keyword in Oracle order, what `body` writes for it. A triggered
`body` makes one triggered ability under the first keyword's trigger, whose action runs each
keyword's action `sequentially`; a static `body` makes one static ability whose parts are each
keyword's (`conjunction`). Only the first keyword's trigger is read. Any other `body`, a mix of
triggered and static, or an empty list joins nothing: an empty `conjunction`, refused `nonEmpty`.
A keyword with variants grants each of them through `keywordGrant` [CR#702.1c]. -/
-- [CR#113.2c]
-- [CR#702.1c]
template alsoForKeywords (body : KeywordTerm → Ability)
    (keywords : List KeywordTerm := []) : Ability := {
  -- TODO(spelling): Format and coordinate the keywords list entries, including the empty case.
  body :=
    have parts := List.map body keywords;
    have actions :=
      List.filterMap
        (fun (x : Ability) =>
          match x with
          | Semantics.Ability.triggered event alternatives while_ joins timing limit intervening action =>
            some action
          | x => none)
        parts;
    have specs :=
      List.filterMap
        (fun (x : Ability) =>
          match x with
          | Semantics.Ability.static spec => some spec
          | x => none)
        parts;
    match parts with
    | Semantics.Ability.triggered event alternatives while_ joins timing limit intervening action :: tail =>
      if (List.length actions == List.length parts) = true then
        triggered event alternatives while_ joins timing limit intervening
          (sequentially actions)
      else static (conjunction [] none)
    | Semantics.Ability.static spec :: tail =>
      static
        (conjunction (if (List.length specs == List.length parts) = true then specs else []) none)
    | x => static (conjunction [] none)
}
