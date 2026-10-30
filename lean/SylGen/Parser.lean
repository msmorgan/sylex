import Lean

namespace SylGen

structure Position where
  file : String
  line : Nat := 1
  column : Nat := 1
  deriving Inhabited, Repr

def Position.error (p : Position) (message : String) : String :=
  s!"{p.file}:{p.line}:{p.column}: {message}"

inductive Value where
  | word (pos : Position) (name : String) (arguments : Array (Option String × Value))
  | string (pos : Position) (text : String)
  | integer (pos : Position) (text : String)
  | list (pos : Position) (items : Array Value)
  | record (pos : Position) (fields : Array (String × Value))
  deriving Inhabited, Repr

def Value.position : Value → Position
  | .word p .. | .string p .. | .integer p .. | .list p .. | .record p .. => p

structure SortRef where
  name : String
  arguments : Array SortRef := #[]
  deriving Inhabited, Repr, BEq

partial def SortRef.text (s : SortRef) : String :=
  if s.arguments.isEmpty then s.name
  else s.name ++ "(" ++ String.intercalate ", " (s.arguments.toList.map SortRef.text) ++ ")"

structure Parameter where
  pos : Position
  name : String
  sort : SortRef
  defaultValue : Option Value := none
  deriving Inhabited, Repr

structure Declaration where
  pos : Position
  kind : String
  name : String
  parameters : Array Parameter := #[]
  sort : SortRef
  fields : Array (String × Value) := #[]
  constructor : Option String := none
  cardValue : Option Value := none
  documentation : String := ""
  deriving Inhabited, Repr

private structure Token where
  pos : Position
  text : String
  quoted : Bool := false
  comment : Bool := false
  deriving Inhabited, Repr

private def tokenize (file source : String) : Except String (Array Token) := do
  let chars := source.toList.toArray
  let mut i := 0
  let mut pos : Position := { file }
  let mut out := #[]
  while i < chars.size do
    let c := chars[i]!
    let start := pos
    if c == '\n' then
      pos := { pos with line := pos.line + 1, column := 1 }
      i := i + 1
    else if c.isWhitespace then
      pos := { pos with column := pos.column + 1 }
      i := i + 1
    else if c == '#' then
      i := i + 1
      pos := { pos with column := pos.column + 1 }
      let mut comment := ""
      while i < chars.size && chars[i]! != '\n' do
        comment := comment.push chars[i]!
        i := i + 1
        pos := { pos with column := pos.column + 1 }
      out := out.push { pos := start, text := comment, comment := true }
    else if c == '"' then
      i := i + 1
      pos := { pos with column := pos.column + 1 }
      let mut value := ""
      while i < chars.size && chars[i]! != '"' do
        let mut c := chars[i]!
        if c == '\n' then throw (start.error "newline in string literal")
        if c == '\\' then
          i := i + 1
          pos := { pos with column := pos.column + 1 }
          if i ≥ chars.size then throw (start.error "unterminated escape")
          c := chars[i]!
          unless c == '\\' || c == '"' do throw (pos.error "unsupported string escape")
        value := value.push c
        i := i + 1
        pos := { pos with column := pos.column + 1 }
      if i == chars.size then throw (start.error "unterminated string literal")
      i := i + 1
      pos := { pos with column := pos.column + 1 }
      out := out.push { pos := start, text := value, quoted := true }
    else if "()[]{},:=".contains c then
      out := out.push { pos := start, text := c.toString }
      i := i + 1
      pos := { pos with column := pos.column + 1 }
    else
      let mut value := ""
      if c == '-' then
        value := "-"
        i := i + 1
        pos := { pos with column := pos.column + 1 }
      while i < chars.size && (chars[i]!.isAlphanum || chars[i]! == '_' || chars[i]! == '.') do
        value := value.push chars[i]!
        i := i + 1
        pos := { pos with column := pos.column + 1 }
      if value.isEmpty || value == "-" then throw (start.error s!"unexpected character '{c}'")
      out := out.push { pos := start, text := value }
  return out.push { pos := pos, text := "<eof>" }

private abbrev Parser := StateT (Array Token × Nat) (Except String)

private def peekRaw : Parser Token := do
  let (tokens, i) ← get
  return tokens[i]!

private def peek : Parser Token := do
  while (← peekRaw).comment do modify fun (ts, i) => (ts, i + 1)
  peekRaw

private def take : Parser Token := do
  let t ← peek
  modify fun (ts, i) => (ts, i + 1)
  return t

private def accept (s : String) : Parser Bool := do
  let saved ← get
  let t ← peek
  if !t.quoted && t.text == s then discard take; return true
  set saved
  return false

private def expect (s : String) : Parser Unit := do
  unless ← accept s do throw ((← peek).pos.error s!"expected '{s}'")

private def word : Parser Token := do
  let t ← take
  unless !t.quoted && (t.text.toList.head?.any fun c => c.isAlpha || c == '_') do
    throw (t.pos.error "expected identifier")
  return t

private partial def value : Parser Value := do
  let t ← take
  if t.quoted then return .string t.pos t.text
  if t.text == "[" then
    let mut items := #[]
    unless ← accept "]" do
      repeat
        items := items.push (← value)
        if ← accept "]" then break
        expect ","
        if ← accept "]" then break
    return .list t.pos items
  if t.text == "{" then
    let mut fields := #[]
    unless ← accept "}" do
      repeat
        let key ← word
        if fields.any (·.1 == key.text) then throw (key.pos.error "duplicate field")
        expect "="
        fields := fields.push (key.text, ← value)
        if ← accept "}" then break
        expect ","
        if ← accept "}" then break
    return .record t.pos fields
  if t.text.toInt?.isSome then return .integer t.pos t.text
  unless t.text.toList.head?.any (fun c => c.isAlpha || c == '_') do
    throw (t.pos.error "expected expression")
  let mut args := #[]
  if ← accept "(" then
    if ← accept ")" then throw (t.pos.error "empty call: write a bare word")
    repeat
      discard peek
      let (ts, i) ← get
      let name ← if !ts[i]!.quoted && ts[i+1]!.text == "=" then do
          let key ← word
          expect "="
          pure (some key.text)
        else pure none
      args := args.push (name, ← value)
      if ← accept ")" then break
      expect ","
      if ← accept ")" then break
  return .word t.pos t.text args

private partial def sort : Parser SortRef := do
  let t ← word
  let mut args := #[]
  if ← accept "(" then
    args := args.push (← sort)
    while ← accept "," do args := args.push (← sort)
    expect ")"
  return { name := t.text, arguments := args }

def kindSort (kind : String) : Option String :=
  match kind with
  | "action" => some "Action" | "ability" | "ability_word" => some "Ability"
  | "counter" => some "CounterKind" | "designation" => some "DesignationLabel"
  | "subtype" => some "Subtype" | "card_type" => some "CardType"
  | "relation" => some "RelationLabel" | "token" => some "PredefinedToken"
  | "sba" => some "SbaRule" | "conferral" => some "ConferralRule" | "card" => some "Card"
  | _ => none

private partial def printedName (v : Value) : Except String String := do
  let fields ← match v with
    | .record _ fields => pure fields
    | .word p ctor #[(none, .record _ fields)] =>
      let parts ← match ctor with
        | "transforming" | "modalDfc" => pure ["front", "back"]
        | "split" | "sharedLineSplit" => pure ["left", "right"]
        | "adventurer" => pure ["normal", "adventure"]
        | "flip" => pure ["normal", "alternative"]
        | "leveler" | "prototype" => pure ["inner"]
        | _ => throw (p.error "unknown card form")
      let names ← parts.mapM fun part => do
        let some (_, value) := fields.find? (·.1 == part)
          | throw (p.error s!"missing card part '{part}'")
        printedName value
      return " // ".intercalate names
    | _ => throw (v.position.error "expected named card parts")
  match fields.find? (·.1 == "name") with
  | some (_, .string _ name) => pure name
  | _ => throw (v.position.error "card face requires a string name")

private def declaration : Parser Declaration := do
  let k ← word
  unless k.text ∈ ["primitive", "template"] || (kindSort k.text).isSome do
    throw (k.pos.error s!"unknown declaration kind '{k.text}'")
  let n ← if k.text == "card" then pure "" else pure (← word).text
  if n ∈ ["none", "true", "false"] then throw (k.pos.error "reserved declaration name")
  let mut parameters := #[]
  if ← accept "(" then
    repeat
      let p ← word
      if p.text ∈ ["none", "true", "false"] then
        throw (p.pos.error "reserved parameter name")
      if parameters.any (·.name == p.text) then throw (p.pos.error "duplicate parameter")
      expect ":"
      let s ← sort
      let d ← if ← accept "=" then pure (some (← value)) else pure none
      parameters := parameters.push { pos := p.pos, name := p.text, sort := s, defaultValue := d }
      if ← accept ")" then break
      expect ","
      if ← accept ")" then break
  let result ← if k.text ∈ ["primitive", "template"] then do
      expect ":"
      sort
    else pure { name := (kindSort k.text).getD "" }
  let constructor ← if ← accept "=" then do
      unless k.text == "primitive" do
        throw (k.pos.error "only a primitive may name a constructor")
      let t ← take
      let name := (t.text.drop 1).toString
      unless !t.quoted && t.text.startsWith "." &&
          (name.toList.head?.any fun c => c.isAlpha || c == '_') &&
          name.toList.all (fun c => c.isAlphanum || c == '_') do
        throw (t.pos.error "expected '.constructor'")
      if name == n then throw (t.pos.error "redundant constructor binding")
      pure (some name)
    else pure none
  let cardValue ← if k.text == "card" then pure (some (← value)) else pure none
  let fields ← if k.text == "card" then pure #[] else if k.text == "primitive" then pure #[] else do
    let v ← value
    match v with
    | .record _ fs => pure fs
    | _ => throw (v.position.error "expected declaration block")
  let n ← if let some v := cardValue then
      match printedName v with
      | .ok name => pure name
      | .error message => throw message
    else pure n
  return { pos := k.pos, kind := k.text, name := n, parameters, sort := result,
           fields, constructor, cardValue }

def parse (file source : String) : Except String (Array Declaration) := do
  let tokens ← tokenize file source
  let parser : Parser (Array Declaration) := do
    let mut out := #[]
    repeat
      let mut documentation := ""
      let mut lastLine := 0
      while (← peekRaw).comment do
        let comment ← peekRaw
        if comment.pos.line > lastLine + 1 then documentation := ""
        documentation := documentation ++ comment.text ++ "\n"
        lastLine := comment.pos.line
        modify fun (ts, i) => (ts, i + 1)
      if ← accept "<eof>" then break
      if (← peekRaw).pos.line > lastLine + 1 then documentation := ""
      let d ← declaration
      out := out.push { d with documentation }
    return out
  return (← parser.run (tokens, 0)).1

end SylGen
