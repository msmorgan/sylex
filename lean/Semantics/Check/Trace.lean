/-!
# Semantics.Check.Trace

A default-off side channel for the discourse exporter (`lean/DiscourseExport/`). The checker
calls `Trace.frame` (or `frameWith`, which also summarises what a referring phrase resolved to) at
its stepping closures, `Trace.emit` where it mints or moves a referent, and `Trace.rewrite` where
it steps a rewritten copy of a card's node. All are the identity on their result (`emit_eq`,
`rewrite_eq`, `frame_eq`, `frameWith_eq`, `scope_eq`); while no exporter has switched the log on
(`Trace.log` is `none`) their compiled forms only read the switch. The log lives outside the
`DRS`, so a branch that rolls the context back cannot drop its events. The implementations are
`never_extract` too, as the compiler checks the callee after the `implemented_by` swap.

Disposable: deleted with Lean (`lean-retirement`).
-/

namespace Semantics.Trace


/-- One checker event, in stepping order. A node is named by its object address, which the
exporter aligns with the card it evaluated. -/
inductive Event where
  /-- A stepping closure (`kind` is the stepped type's name) or a `scope:` marker begins. -/
  | enter (kind : String) (node : USize)
  /-- The innermost frame ends, returning `ids`; `note` names the rule a referring phrase used;
  `before` is the context's next id when the frame began. -/
  | exit (ids : List Nat) (note : String) (before : Nat)
  /-- Referents introduced together, each with its id and its kind tag. -/
  | mint (entries : List (Nat × String))
  /-- A zone move gave each referent `old` the fresh identity `new` [CR#400.7]. -/
  | successors (moves : List (Nat × Nat))
  /-- The checker rewrote the card's node at `original` as the node at `node`, which it steps in
its place. -/
  | rewritten (node original : USize)
  deriving Inhabited, Repr

/-- The log; `none` while tracing is off. -/
initialize log : IO.Ref (Option (Array Event)) ← IO.mkRef none

@[noinline, never_extract]
private unsafe def emitImpl {α : Type} (event : Unit → Event) (value : α) : α :=
  unsafeBaseIO do
    log.modify fun state => state.map (·.push (event ()))
    return value

/-- Log `event` when tracing is on; `value` either way. -/
@[implemented_by emitImpl, never_extract]
def emit {α : Type} (_event : Unit → Event) (value : α) : α := value

@[noinline, never_extract]
private unsafe def rewriteImpl {α : Type} (original result : α) : α :=
  unsafeBaseIO do
    log.modify fun state =>
      state.map (·.push (.rewritten (ptrAddrUnsafe result) (ptrAddrUnsafe original)))
    return result

/-- `result`, which the checker steps in place of `original`; logged when tracing is on. -/
@[implemented_by rewriteImpl, never_extract]
def rewrite {α : Type} (_original result : α) : α := result

@[noinline, never_extract]
private unsafe def frameImpl {α : Type} {β : Type} (kind : String) (node : α)
    (summary : Unit → β → List Nat × String × Nat) (body : Unit → β) : β :=
  unsafeBaseIO do
    let on ← log.modifyGet fun state => match state with
      | none => (false, none)
      | some events => (true, some (events.push (.enter kind (ptrAddrUnsafe node))))
    let result := body ()
    if on then
      let (ids, note, before) := summary () result
      log.modify fun state => state.map (·.push (.exit ids note before))
    return result

/-- Step `body` as the frame `kind` over `node`, summarised at its end by `summary`. -/
@[implemented_by frameImpl, never_extract]
def frameWith {α : Type} {β : Type} (_kind : String) (_node : α)
    (_summary : Unit → β → List Nat × String × Nat) (body : Unit → β) : β := body ()

/-- Step `body` as the frame `kind` over `node`. -/
@[inline] def frame {α : Type} {β : Type} (kind : String) (node : α) (body : Unit → β) : β :=
  frameWith kind node (fun _ _ => ([], "", 0)) body

/-- A labelled scope with no node (`scope:<label>`), such as a loop's published aggregate. -/
@[inline] def scope {β : Type} (label : String) (body : Unit → β) : β :=
  frameWith ("scope:" ++ label) () (fun _ _ => ([], "", 0)) body

@[simp] theorem emit_eq {α : Type} (event : Unit → Event) (value : α) :
    emit event value = value := rfl

@[simp] theorem rewrite_eq {α : Type} (original result : α) :
    rewrite original result = result := rfl

@[simp] theorem frame_eq {α : Type} {β : Type} (kind : String) (node : α) (body : Unit → β) :
    frame kind node body = body () := rfl

@[simp] theorem scope_eq {β : Type} (label : String) (body : Unit → β) :
    scope label body = body () := rfl

@[simp] theorem frameWith_eq {α : Type} {β : Type} (kind : String) (node : α)
    (summary : Unit → β → List Nat × String × Nat) (body : Unit → β) :
    frameWith kind node summary body = body () := rfl

/-- Run `act` with the log on, returning its result and the events it logged. -/
def collect {β : Type} (act : Unit → β) : IO (β × Array Event) := do
  log.set (some #[])
  let result ← IO.lazyPure act
  let events ← log.modifyGet fun state => (state.getD #[], none)
  return (result, events)

end Semantics.Trace
