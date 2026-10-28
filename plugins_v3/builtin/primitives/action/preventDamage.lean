/-- "prevent <cut> [of <damage>]. [<also>]": prevent `cut` of the damage `damage` names, or of
"that damage", the damage in scope, then do `also`, the prevention's additional effect
[CR#615.1,615.5]. Under `cant`, "<damage> can't be prevented" [CR#615.12]. -/
-- [CR#615.1,615.5]
-- [CR#615.12]
primitive preventDamage (cut : PreventCut) (damage : Option WouldClause := none)
    (also : Option Action := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by damage, also.
  body :=
    Semantics.Primitives.Action.preventDamage cut damage also
}
