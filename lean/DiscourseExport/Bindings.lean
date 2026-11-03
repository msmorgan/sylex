import DiscourseExport.Paths
import Semantics.Check

/-!
# DiscourseExport.Bindings

Aligns a traced `Card.check` (`Semantics.Trace`) with the card's positions (`DiscourseExport.Paths`)
and reads off its bindings: each referring noun phrase's mention, keyed by its path, with the
antecedents it resolved to, keyed by the path of the node that introduced each and a role tag.

A frame's node is found by its address, which `walk` made unique per position, or through the
checker's logged rewrite of a node (`Trace.rewrite`). A node with no fields is a boxed scalar
shared by every such node; it is placed among the positions of its constructor (`locate`). A
frame whose node is in no position is one the checker built itself (`synthesized`); what it
introduces belongs to the nearest aligned frame.
-/

open Lean Semantics

namespace DiscourseExport

structure Key where
  path : Path
  role : String
  /-- A synthesized frame's kind, when the node introducing this one is not in the card. -/
  via : Option String := none
  /-- The key of the referent a moved successor continues. -/
  source : Option Json := none
  /-- The introducing node is a scalar placed by reading order among equal candidates. -/
  placedByOrder : Bool := false
  deriving Inhabited

def Key.toJson (k : Key) : Json :=
  Json.mkObj <| [("path", ToJson.toJson k.path), ("role", ToJson.toJson k.role)] ++
    (k.via.toList.map fun v => ("via", ToJson.toJson v)) ++
    (k.source.toList.map fun s => ("from", s)) ++
    (if k.placedByOrder then [("placedByOrder", Json.bool true)] else [])

/-- One way a mention resolved: its rule and antecedents, with the passes that stepped it so. -/
structure Reading where
  rule : String
  antecedents : Array Key
  /-- `check` (the line's refusals, `ops.ab`) and/or `context` (the context the line leaves for
  the lines after it, `lineTextContextWith`). -/
  passes : Array String := #[]
  /-- Where each stepping that read the mention so began, below the line (its ability): the
  first node the pass stepped. The line's own top node is the in-order reading; a deeper one is a
  node read again out of order, such as a `doIf`'s then-branch read ahead of its condition for
  the targets it announces [CR#601.2c]. -/
  entries : Array Path := #[]
  /-- Some stepping began at the ability's own top node (its `action`, `spec`, …), with no other
  stepped node between: an in-order stepping rather than a node read ahead. -/
  inOrder : Bool := false
  /-- The passes whose last stepping of the mention read it so. Each pass's last stepping is the
  one whose context the pass goes on with: a `doIf`'s then-branch is stepped to announce its
  targets and to reserve its ids before the stepping whose refusals count. The reading with
  `check` here is the one Lean's verdict rests on, and the one `discourse` is expected to
  produce. -/
  lastIn : Array String := #[]
  deriving Inhabited

def Reading.toJson (r : Reading) : Json :=
  Json.mkObj [("rule", ToJson.toJson r.rule),
    ("antecedents", Json.arr (r.antecedents.map (·.toJson))),
    ("passes", ToJson.toJson r.passes), ("entries", ToJson.toJson r.entries),
    ("inOrder", ToJson.toJson r.inOrder), ("lastIn", ToJson.toJson r.lastIn)]

/-- A mention: a referring noun phrase, with each distinct way the checker resolved it. -/
structure Binding where
  mention : Path
  node : Name
  placedByOrder : Bool := false
  readings : Array Reading := #[]
  flagDefect : Bool := false
  deriving Inhabited

def Binding.toJson (b : Binding) : Json :=
  Json.mkObj [
    ("mention", Json.mkObj <| [("path", ToJson.toJson b.mention),
      ("node", ToJson.toJson b.node.toString)] ++
      (if b.placedByOrder then [("placedByOrder", Json.bool true)] else [])),
    ("readings", Json.arr (b.readings.map (·.toJson))),
    ("flagDefect", ToJson.toJson b.flagDefect)]

private structure Frame where
  kind : String
  pos : Option Nat := none
  synthesized : Bool := false
  /-- A scalar node placed by reading order among several candidates. -/
  byOrder : Bool := false
  scope : Option String := none
  mints : Nat := 0
  moves : Nat := 0
  deriving Inhabited

structure Alignment where
  bindings : Array Binding := #[]
  /-- The card has a stepped `laterMention`, whose use the Lean checker never checks against its
  own slot (Lean multi-use parameter defect, which `discourse-core` fixes), so its verdict may
  differ. -/
  verdictFlagDefect : Bool := false
  frames : Nat := 0
  aligned : Nat := 0
  scalars : Nat := 0
  /-- Scalar nodes placed by reading order among several candidates. -/
  scalarsByOrder : Nat := 0
  /-- Mentions among those. -/
  mentionsByOrder : Nat := 0
  synthesized : Nat := 0
  /-- Frames over a node the checker rewrote, aligned to the card's original. -/
  rewritten : Nat := 0
  mintsViaSynthesized : Nat := 0
  outOfRegion : Nat := 0
  failures : Array String := #[]

def Alignment.fail (a : Alignment) (message : String) : Alignment :=
  { a with failures := a.failures.push message }

/-- Where each `enter` event's node is: its position, and whether reading order placed it. -/
private structure Located where
  pos : Array (Option Nat)
  byOrder : Array Bool
  out : Alignment

private structure Pending where
  event : Nat
  key : String
  ctor : Name
  type : String
  deriving Inhabited

private structure Region where
  kind : String
  pos : Option Nat := none
  aligned : Bool := false
  rewrites : Std.HashMap USize USize := {}
  /-- The address a frame over a rewritten node entered with: the rewrite ends with the last
  frame over it, after which the address may be reused. -/
  rewrittenAt : Option USize := none
  pending : Array Pending := #[]
  /-- Positions of the aligned non-scalar frames this frame owns. -/
  children : Array Nat := #[]
  deriving Inhabited

/-- Pass 1: place every frame. A non-scalar node is found by address (through the checker's
rewrites made inside an enclosing frame). A scalar node with one candidate in its owner is placed
at once; otherwise, when its owner ends, the owner's scalar visits of each constructor are placed
in reading order on the candidates outside the nodes the owner's other frames covered, and their
numbers must agree. -/
private def locate (env : Environment) (positions : Array Position)
    (byAddress : Std.HashMap (String × USize) Nat) (events : Array Trace.Event)
    (out : Alignment) : Located := Id.run do
  let mut out := out
  let mut located : Array (Option Nat) := Array.replicate events.size none
  let mut byOrder : Array Bool := Array.replicate events.size false
  let ctorsOf := fun (type : String) =>
    match env.find? (`Semantics ++ type.toName) with
    | some (.inductInfo info) => info.ctors.toArray
    | _ => #[]
  let pathAt := fun (p : Option Nat) => match p with
    | some p => positions[p]!.path
    | none => #[]
  let owner := fun (stack : Array Region) => Id.run do
    let mut i := stack.size
    while i > 0 do
      i := i - 1
      if stack[i]!.aligned then return i
    return 0
  let candidates := fun (region : Path) (covered : Array Nat) (type : String) (ctor : Name) =>
    (Array.range positions.size).filter fun i =>
      positions[i]!.type == type && positions[i]!.ctor == ctor &&
        region.isPrefixOf positions[i]!.path &&
        !covered.any fun c => positions[c]!.path.isPrefixOf positions[i]!.path
  let mut stack : Array Region := #[{ kind := "root", aligned := true }]
  let mut i := 0
  -- One extra step past the end closes the root.
  while i ≤ events.size do
    let event := events[i]?.getD (.exit [] "" 0)
    let closing := i == events.size
    match event with
    | .enter kind addr =>
      if kind.startsWith "scope:" then
        stack := stack.push { kind }
      else
        out := { out with frames := out.frames + 1 }
        let type := (kind.takeWhile (· != '/')).toString
        let o := owner stack
        let region := pathAt stack[o]!.pos
        if addr % 2 == 1 then
          let ctor := (ctorsOf type)[(addr / 2).toNat]?.getD .anonymous
          let cands := candidates region #[] type ctor
          if cands.size == 1 then
            located := located.set! i (some cands[0]!)
            out := { out with scalars := out.scalars + 1 }
          else if cands.isEmpty then
            out := { out with synthesized := out.synthesized + 1 }
          else
            let pending : Pending := { event := i, key := s!"{kind}/{ctor}", ctor, type }
            stack := stack.modify o fun f => { f with pending := f.pending.push pending }
          stack := stack.push { kind }
        else
          let raw := addr
          let mut addr := addr
          let mut hops := 0
          let rewriteOf := fun (a : USize) => stack.reverse.findSome? (·.rewrites[a]?)
          while !byAddress.contains (type, addr) && hops < 64 do
            let some original := rewriteOf addr | break
            addr := original
            hops := hops + 1
          if hops > 0 then out := { out with rewritten := out.rewritten + 1 }
          match byAddress[(type, addr)]? with
          | some p =>
            out := { out with aligned := out.aligned + 1 }
            unless region.isPrefixOf positions[p]!.path do
              out := { out with outOfRegion := out.outOfRegion + 1 }
              out := out.fail
                s!"{kind} at {(toJson positions[p]!.path).compress} outside {(toJson region).compress}"
            located := located.set! i (some p)
            stack := stack.modify o fun f => { f with children := f.children.push p }
            stack := stack.push { kind, pos := some p, aligned := true
                                  rewrittenAt := if hops > 0 then some raw else none }
          | none =>
            out := { out with synthesized := out.synthesized + 1 }
            stack := stack.push { kind }
    | .exit .. =>
      if stack.size == 1 && !closing then
        out := out.fail "unbalanced exit"
      else
        let frame := stack.back!
        if !closing then stack := stack.pop
        if let some raw := frame.rewrittenAt then
          unless stack.any (·.rewrittenAt == some raw) do
            stack := stack.map fun f => { f with rewrites := f.rewrites.erase raw }
        let mut keys : Array String := #[]
        for p in frame.pending do
          unless keys.contains p.key do keys := keys.push p.key
        for key in keys do
          let group := frame.pending.filter (·.key == key)
          let first := group[0]!
          let cands := candidates (pathAt frame.pos) frame.children first.type first.ctor
          if group.size == cands.size then
            for (p, c) in group.zip cands do
              located := located.set! p.event (some c)
              byOrder := byOrder.set! p.event true
            out := { out with
              scalars := out.scalars + group.size, scalarsByOrder := out.scalarsByOrder + group.size }
          else
            out := out.fail
              s!"{frame.kind} at {(toJson (pathAt frame.pos)).compress}: {group.size} visits of {key}, {cands.size} positions"
        if closing && stack.size != 1 then
          out := out.fail s!"{stack.size - 1} frames left open"
    | .rewritten node original =>
      stack := stack.modify (stack.size - 1) fun f =>
        { f with rewrites := f.rewrites.insert node original }
    | _ => pure ()
    i := i + 1
  return { pos := located, byOrder, out }

/-- Align `events` to `positions` (`addresses[j]` the node of `positions[slots[j]]`). -/
def align (env : Environment) (positions : Array Position) (slots : Array Nat)
    (addresses : List USize) (events : Array Trace.Event) : Alignment := Id.run do
  let mut out : Alignment := {}
  let mut byAddress : Std.HashMap (String × USize) Nat := {}
  for (slot, addr) in slots.toList.zip addresses do
    let key := (positions[slot]!.type, addr)
    if addr % 2 == 1 then continue
    if byAddress.contains key then
      out := out.fail s!"shared allocation at slot {slot}"
    byAddress := byAddress.insert key slot
  let located := locate env positions byAddress events out
  out := located.out
  let pathKey := fun (p : Path) => (ToJson.toJson p).compress
  let mut byPath : Std.HashMap String Nat := {}
  for h : q in [:positions.size] do
    byPath := byPath.insert (pathKey positions[q].path) q
  -- Binding Identities with a stepped later mention.
  let mut steppedLater : Std.HashSet String := {}
  for p in located.pos do
    if let some p := p then
      if positions[p]!.ctor == ``Semantics.NounPhrase.laterMention then
        steppedLater := steppedLater.insert positions[p]!.detail
  out := { out with verdictFlagDefect := !steppedLater.isEmpty }
  let child := fun (q : Nat) (field : String) =>
    byPath[pathKey (positions[q]!.path.push { ctor := positions[q]!.ctor, field })]?
  -- A mention's argument: its `phrase`, through `inCaller`.
  let argument := fun (q : Nat) => (child q "phrase").bind fun a =>
    if positions[a]!.ctor == ``Semantics.NounPhrase.inCaller then child a "body" else some a
  let barePro := fun (q : Nat) =>
    positions[q]!.ctor == ``Semantics.NounPhrase.pro && positions[q]!.detail == "bare"
  let isMention := fun (q : Nat) =>
    positions[q]!.ctor == ``Semantics.NounPhrase.firstMention ||
      positions[q]!.ctor == ``Semantics.NounPhrase.laterMention
  -- A twice-used parameter's bare pronoun argument resolves at its first use only
  -- (`discourse-core`): the mention, or the pronoun directly in it.
  let flagged := fun (q : Nat) =>
    let twiceUsed := fun (m : Nat) => isMention m && steppedLater.contains positions[m]!.detail &&
      (argument m).any barePro
    if isMention q then twiceUsed q
    else if !barePro q then false
    else
      let path := positions[q]!.path
      let parent := byPath[pathKey path.pop]?
      let mention := parent.bind fun a =>
        if isMention a then some a
        else if positions[a]!.ctor == ``Semantics.NounPhrase.inCaller then
          byPath[pathKey path.pop.pop]?.filter isMention
        else none
      mention.any fun m => twiceUsed m && argument m == some q
  let mut stack : Array Frame := #[{ kind := "root" }]
  let mut ids : Std.HashMap Nat Key := {}
  let mut mentions : Array Binding := #[]
  let mut mentionIndex : Std.HashMap String Nat := {}
  let mut lastReading : Std.HashMap (Nat × String) Nat := {}
  -- The nearest placed frame (the root counts as placed at the empty path).
  let owner := fun (stack : Array Frame) => Id.run do
    let mut i := stack.size
    while i > 0 do
      i := i - 1
      if stack[i]!.pos.isSome || i == 0 then return i
    return 0
  let pathOf := fun (f : Frame) => match f.pos with
    | some p => positions[p]!.path
    | none => #[]
  for h : i in [:events.size] do
    match events[i] with
    | .enter kind _ =>
      if kind.startsWith "scope:" then
        stack := stack.push { kind, scope := some (kind.drop 6).toString }
      else
        match located.pos[i]! with
        | some p => stack := stack.push { kind, pos := some p, byOrder := located.byOrder[i]! }
        | none => stack := stack.push { kind, synthesized := true }
    | .exit resolved rule before =>
      let some frame := stack.back? | continue
      if stack.size == 1 then continue
      stack := stack.pop
      if frame.kind != "NounPhrase" || rule.isEmpty || resolved.isEmpty then continue
      let antecedentIds := match rule with
        | "firstMention" | "laterMention" => some resolved
        | "definite" | "participial" => if resolved.all (· < before) then some resolved else none
        | _ => some resolved
      let some antecedentIds := antecedentIds | continue
      let some p := frame.pos
        | out := out.fail s!"{rule} mention of a synthesized node"
          continue
      let mut keys := #[]
      for id in antecedentIds do
        match ids[id]? with
        | some k => keys := keys.push k
        | none =>
          out := out.fail
            s!"{rule} at {(toJson positions[p]!.path).compress}: unknown antecedent {id}"
      let pass := match stack.find? (·.kind.startsWith "Ability") with
        | some f => if f.kind == "Ability/text" then "context" else "check"
        | none => "card"
      -- The innermost ability stepping the mention, and the first node it stepped toward it.
      let abilityAt := stack.size - 1 -
        ((stack.reverse.findIdx? (·.kind.startsWith "Ability")).getD (stack.size - 1))
      let chain := ((stack.extract abilityAt stack.size).push frame).filterMap (·.pos)
      let entry : Path := match chain[1]? <|> chain[0]? with
        | some q => positions[q]!.path
        | none => #[]
      let ordered := match chain[0]? with
        | some a =>
          let top := positions[a]!.path.size
          entry.size > top && (List.range (entry.size - top - 1)).all fun j =>
            !byPath.contains (pathKey (entry.extract 0 (top + 1 + j)))
        | none => false
      let path := positions[p]!.path
      let k := pathKey path
      let index ← match mentionIndex[k]? with
        | some index => pure index
        | none =>
          let b : Binding := {
            mention := path, node := positions[p]!.ctor
            placedByOrder := frame.byOrder, flagDefect := flagged p }
          mentions := mentions.push b
          if frame.byOrder then out := { out with mentionsByOrder := out.mentionsByOrder + 1 }
          mentionIndex := mentionIndex.insert k (mentions.size - 1)
          pure (mentions.size - 1)
      let antecedentsText := (Json.arr (keys.map (·.toJson))).compress
      let readingAt := mentions[index]!.readings.findIdx? fun r => r.rule == rule &&
        (Json.arr (r.antecedents.map (·.toJson))).compress == antecedentsText
      lastReading := lastReading.insert (index, pass)
        (readingAt.getD mentions[index]!.readings.size)
      mentions := mentions.modify index fun b =>
        match b.readings.findIdx? (fun r => r.rule == rule &&
            (Json.arr (r.antecedents.map (·.toJson))).compress == antecedentsText) with
        | some r => { b with readings := b.readings.modify r fun r => { r with
            passes := if r.passes.contains pass then r.passes else r.passes.push pass
            entries := if r.entries.contains entry then r.entries else r.entries.push entry
            inOrder := r.inOrder || ordered } }
        | none =>
          let reading : Reading := {
            rule, antecedents := keys, passes := #[pass], entries := #[entry]
            inOrder := ordered }
          { b with readings := b.readings.push reading }
    | .mint entries =>
      let o := owner stack
      let top := stack.back!
      let scopes := (stack.extract (o + 1) stack.size).toList.filterMap (·.scope)
      let via := if top.synthesized then some top.kind else none
      if via.isSome then out := { out with mintsViaSynthesized := out.mintsViaSynthesized + 1 }
      if scopes.contains "RelativeHead" then
        -- The head a relative clause's `which` binds is the noun phrase the clause restricts.
        let head := (stack.reverse.find? fun f =>
          f.kind == "NounPhrase" && f.pos.isSome).getD stack[o]!
        for (id, _) in entries do
          let key : Key := {
            path := pathOf head, role := "RelativeHead", placedByOrder := head.byOrder }
          ids := ids.insert id key
        continue
      for (id, tag) in entries do
        let k := stack[o]!.mints
        stack := stack.modify o fun f => { f with mints := f.mints + 1 }
        let role := match scopes.head? with
          | some label => label
          | none => if tag == "Intro" || tag == "Created" then s!"{tag}({k})" else tag
        let key : Key := {
          path := pathOf stack[o]!, role, via, placedByOrder := stack[o]!.byOrder }
        ids := ids.insert id key
    | .successors moves =>
      let o := owner stack
      for (old, new) in moves do
        let k := stack[o]!.moves
        stack := stack.modify o fun f => { f with moves := f.moves + 1 }
        let key : Key := {
          path := pathOf stack[o]!, role := s!"MovedSuccessor({k})"
          source := ids[old]?.map (·.toJson), placedByOrder := stack[o]!.byOrder }
        ids := ids.insert new key
    | .rewritten .. => pure ()
  for ((index, pass), r) in lastReading.toList do
    mentions := mentions.modify index fun b =>
      { b with readings := b.readings.modify r fun r => { r with lastIn := r.lastIn.push pass } }
  mentions := mentions.map fun b =>
    { b with readings := b.readings.map fun r => { r with lastIn := r.lastIn.qsort (· < ·) } }
  return { out with bindings := mentions }

end DiscourseExport
