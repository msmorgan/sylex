//! Focused tests of the `.syl` printer: one or more per style rule and per
//! option setting. Every case also checks that the output reparses to the
//! same tree (see `same_meaning`) and formats to itself.

use sylex_syl::Closers;
use sylex_syl::FileId;
use sylex_syl::Style;
use sylex_syl::TrailingCommas;
use sylex_syl::format;
use sylex_syl::parse;
use sylex_syl::same_meaning;

fn fmt_with(source: &str, style: &Style) -> String {
    let file = parse(FileId(0), source).unwrap_or_else(|e| panic!("{}", e.render("in", source)));
    let out = format(&file, style);
    let reparsed =
        parse(FileId(0), &out).unwrap_or_else(|e| panic!("{}\n{out}", e.render("out", &out)));
    assert!(same_meaning(&reparsed, &file), "tree changed:\n{out}");
    assert_eq!(format(&reparsed, style), out, "not idempotent");
    out
}

fn fmt(source: &str) -> String {
    fmt_with(source, &Style::default())
}

/// A word of `n` letters.
fn word(n: usize) -> String {
    "w".repeat(n)
}

// Layout.

#[test]
fn a_node_that_fits_stays_on_one_line() {
    assert_eq!(
        fmt("card {\n  name = \"A\",\n  text = [\n    spell(draw(1))]}"),
        "card {\n  name = \"A\",\n  text = [spell(draw(1))]\n}\n"
    );
}

#[test]
fn a_node_that_does_not_fit_puts_every_child_on_its_own_line() {
    let long = word(45);
    let out = fmt(&format!("card {{ text = [f({long}, {long}, g(a, b))] }}"));
    assert_eq!(
        out,
        format!("card {{\n  text = [\n    f(\n      {long},\n      {long},\n      g(a, b))]\n}}\n")
    );
}

#[test]
fn outer_nodes_break_first() {
    let long = word(50);
    let out = fmt(&format!("card {{ text = [f({long}), g({long})] }}"));
    assert_eq!(
        out,
        format!("card {{\n  text = [\n    f({long}),\n    g({long})]\n}}\n")
    );
}

/// A line `    g(h(W)))` of exactly `width` characters: four spaces of
/// indent, then `g(h(` and the word, then `))` and `f`'s hugged `)`.
fn closer_case(width: usize) -> String {
    let w = word(width - 4 - "g(h()))".len());
    format!("card {{ x = f(a, g(h({w}))) }}")
}

#[test]
fn width_counts_the_trailing_closers() {
    // Exactly 100 columns with the three closers: stays flat.
    let out = fmt(&closer_case(100));
    let line = out.lines().find(|l| l.contains("g(")).unwrap();
    assert_eq!(line.chars().count(), 100, "{out}");
    assert!(line.ends_with(")))"), "{out}");
    // 101: `g` breaks.
    let out = fmt(&closer_case(101));
    assert!(out.contains("\n    a,\n    g(\n      h("), "{out}");
}

#[test]
fn width_counts_the_comma() {
    // `    g(h(W)),` is 100 columns: flat; one more and `g` breaks.
    let case = |width: usize| {
        let w = word(width - 4 - "g(h()),".len());
        (w.clone(), format!("card {{ x = [g(h({w})), b] }}"))
    };
    let (w, source) = case(100);
    assert_eq!(
        fmt(&source),
        format!("card {{\n  x = [\n    g(h({w})),\n    b]\n}}\n")
    );
    let (_, source) = case(101);
    let out = fmt(&source);
    assert!(out.contains("\n    g(\n      h("), "{out}");
    assert!(out.ends_with(")),\n    b]\n}\n"), "{out}");
}

#[test]
fn an_unbreakable_string_runs_past_the_width() {
    let long = "s".repeat(120);
    assert_eq!(
        fmt(&format!("card {{ name = \"{long}\" }}")),
        format!("card {{\n  name = \"{long}\"\n}}\n")
    );
}

#[test]
fn a_declaration_body_always_breaks_unless_empty() {
    assert_eq!(fmt("relation r {}"), "relation r {}\n");
    assert_eq!(fmt("relation r { a = 1 }"), "relation r {\n  a = 1\n}\n");
    assert_eq!(fmt("card {a = 1}"), "card {\n  a = 1\n}\n");
}

#[test]
fn separators_and_flat_records() {
    assert_eq!(
        fmt("card{lexical={ lemma=\"x\" ,plural = \"xs\"},cost=[ 1,2 ],t=f( a,b=c ) }"),
        "card {\n  lexical = {lemma = \"x\", plural = \"xs\"},\n  cost = [1, 2],\n  t = f(a, b = c)\n}\n"
    );
}

#[test]
fn declaration_header_with_lexical_spelling_and_body() {
    assert_eq!(
        fmt(
            "card_type battle { lexical = { lemma = \"battle\" }, spelling = \"battle\", \
             stats = [defense], }"
        ),
        "card_type battle {\n  lexical = {lemma = \"battle\"},\n  spelling = \"battle\",\n  \
         stats = [defense]\n}\n"
    );
}

// Hugging a first list or record argument: only the opener moves.

#[test]
fn a_sole_list_argument_hugs_the_call() {
    // The items stay two levels past the call's line, where they would be
    // unhugged.
    let long = word(40);
    assert_eq!(
        fmt(&format!("card {{ x = and([{long}, {long}, {long}]) }}")),
        format!("card {{\n  x = and([\n      {long},\n      {long},\n      {long}])\n}}\n")
    );
}

#[test]
fn a_first_list_argument_hugs_before_named_and_positional_arguments() {
    // The remaining arguments go one level past the call's line.
    let long = word(40);
    let items = format!("\n      {long},\n      {long},\n      {long}]");
    assert_eq!(
        fmt(&format!(
            "card {{ x = and([{long}, {long}, {long}], subject = it) }}"
        )),
        format!("card {{\n  x = and([{items},\n    subject = it)\n}}\n")
    );
    assert_eq!(
        fmt(&format!(
            "card {{ x = and([{long}, {long}, {long}], b, c) }}"
        )),
        format!("card {{\n  x = and([{items},\n    b,\n    c)\n}}\n")
    );
}

#[test]
fn a_sole_list_argument_hugs_at_depth() {
    let long = word(45);
    assert_eq!(
        fmt(&format!(
            "card {{ x = [spell(sequentially([{long}, {long}]))] }}"
        )),
        format!(
            "card {{\n  x = [\n    spell(\n      sequentially([\n          {long},\n          \
             {long}]))]\n}}\n"
        )
    );
}

#[test]
fn hugs_nest() {
    let long = word(45);
    let and = format!("and([\n          {long},\n          {long}])");
    assert_eq!(
        fmt(&format!(
            "card {{ x = or([and([{long}, {long}]), and([{long}, {long}])]) }}"
        )),
        format!("card {{\n  x = or([\n      {and},\n      {and}])\n}}\n")
    );
}

#[test]
fn a_sole_record_argument_hugs_the_call() {
    assert_eq!(
        fmt(
            "card split({ left = { name = \"Fire\", cost = [1, red], text = [spell(dealDamage(2))] }, \
             right = { name = \"Ice\", cost = [1, blue], text = [spell(tap(target(isPermanent)))] } })"
        ),
        "card split({\n    left = {name = \"Fire\", cost = [1, red], text = [spell(dealDamage(2))]},\n    \
         right = {name = \"Ice\", cost = [1, blue], text = [spell(tap(target(isPermanent)))]}})\n"
    );
}

#[test]
fn a_first_record_argument_hugs_the_call() {
    let long = word(44);
    assert_eq!(
        fmt(&format!(
            "card {{ x = f({{a = {long}, b = {long}}}, n = 1) }}"
        )),
        format!("card {{\n  x = f({{\n      a = {long},\n      b = {long}}},\n    n = 1)\n}}\n")
    );
}

#[test]
fn hugging_falls_back_when_the_list_fits_on_its_own_line() {
    // `x = someCall([a, b])` overflows, but `[a, b])` fits one level in: the
    // unhugged layout takes two lines, the hugged one three.
    let long = word(44);
    let out = fmt(&format!("card {{ x = someCall([{long}, {long}]) }}"));
    assert_eq!(
        out,
        format!("card {{\n  x = someCall(\n    [{long}, {long}])\n}}\n")
    );
    // Followed by another argument, the list counts its comma: `    [W, b],`
    // is 100 columns and stays unhugged; one more column and it hugs.
    let case = |n| {
        let w = word(n);
        (
            w.clone(),
            fmt(&format!("card {{ x = someCall([{w}, b], subject = it) }}")),
        )
    };
    let (w, out) = case(90);
    assert_eq!(
        out,
        format!("card {{\n  x = someCall(\n    [{w}, b],\n    subject = it)\n}}\n")
    );
    let (w, out) = case(91);
    assert_eq!(
        out,
        format!("card {{\n  x = someCall([\n      {w},\n      b],\n    subject = it)\n}}\n")
    );
}

#[test]
fn a_single_item_list_hugs_even_when_it_fits_unhugged() {
    // Both layouts take two lines; the hugged one wins the tie.
    let long = word(92);
    let out = fmt(&format!("card {{ x = someCall([{long}]) }}"));
    assert_eq!(
        out,
        format!("card {{\n  x = someCall([\n      {long}])\n}}\n")
    );
}

#[test]
fn hugging_counts_both_closers() {
    // At `and([` the items are two levels in and the last one is followed by
    // `])`: `      f(` + 89 + `)])` is 100 columns and fits; one more does not.
    let fits = word(89);
    let out = fmt(&format!("card {{ x = and([a, f({fits})]) }}"));
    assert!(out.contains(&format!("\n      f({fits})])\n")), "{out}");
    let line = out.lines().find(|l| l.contains(&fits)).unwrap();
    assert_eq!(line.chars().count(), 100, "{out}");
    let over = word(90);
    let out = fmt(&format!("card {{ x = and([a, f({over})]) }}"));
    assert!(
        out.contains(&format!("\n      f(\n        {over})])\n")),
        "{out}"
    );
}

#[test]
fn a_sole_call_or_named_argument_does_not_hug() {
    let long = word(45);
    assert_eq!(
        fmt(&format!(
            "card {{ x = spell(sequentially([{long}, {long}])) }}"
        )),
        format!(
            "card {{\n  x = spell(\n    sequentially([\n        {long},\n        {long}]))\n}}\n"
        )
    );
    assert_eq!(
        fmt(&format!("card {{ x = f(named = [{long}, {long}]) }}")),
        format!("card {{\n  x = f(\n    named = [\n      {long},\n      {long}])\n}}\n")
    );
}

#[test]
fn interleaved_named_and_positional_arguments_keep_their_order() {
    assert_eq!(
        fmt(
            "card { x = when(does(actor(thisAura), enter), if = is(it, in(battlefield)), draw(1)) }"
        ),
        "card {\n  x = when(does(actor(thisAura), enter), if = is(it, in(battlefield)), draw(1))\n}\n"
    );
    let long = word(45);
    assert_eq!(
        fmt(&format!(
            "card {{ x = f(to = {long}, {long}, from = g, h) }}"
        )),
        format!("card {{\n  x = f(\n    to = {long},\n    {long},\n    from = g,\n    h)\n}}\n")
    );
}

#[test]
fn a_list_after_the_first_argument_does_not_hug() {
    let long = word(46);
    assert_eq!(
        fmt(&format!("card {{ x = f(a, [{long}, {long}]) }}")),
        format!("card {{\n  x = f(\n    a,\n    [\n      {long},\n      {long}])\n}}\n")
    );
}

#[test]
fn a_list_holding_one_call_does_not_hug_it() {
    let long = word(46);
    assert_eq!(
        fmt(&format!("card {{ x = [f({long}, {long})] }}")),
        format!("card {{\n  x = [\n    f(\n      {long},\n      {long})]\n}}\n")
    );
}

#[test]
fn comments_inside_a_hugged_list() {
    // A comment keeps the list from going flat, so it hugs; a trailing one
    // puts the `]` on its own line where it goes unhugged.
    assert_eq!(
        fmt("card { x = f([ # first\n a, b]) }"),
        "card {\n  x = f([\n      # first\n      a,\n      b])\n}\n"
    );
    assert_eq!(
        fmt("card { x = f([a, b # last\n], c) }"),
        "card {\n  x = f([\n      a,\n      b\n      # last\n    ],\n    c)\n}\n"
    );
    // A comment before the list is the call's first item: no hug.
    assert_eq!(
        fmt("card { x = f( # before\n [a, b]) }"),
        "card {\n  x = f(\n    # before\n    [a, b])\n}\n"
    );
}

#[test]
fn hugging_falls_back_under_own_line_closers() {
    // Unhugged, `[a, b]` is followed by the trailing comma, if any, not by
    // the call's `)`, which has its own line.
    let own_line = |trailing_commas| Style {
        closers: Closers::OwnLine,
        trailing_commas,
        ..Style::default()
    };
    let with_comma = own_line(TrailingCommas::OwnLine);
    // `    [W, b],` is 100 columns: unhugged takes three lines, hugged five.
    let w = word(90);
    assert_eq!(
        fmt_with(&format!("card {{ x = f([{w}, b]) }}"), &with_comma),
        format!("card {{\n  x = f(\n    [{w}, b],\n  ),\n}}\n")
    );
    // One more column: the list breaks either way, so it hugs. Only the
    // opener moves: `]` keeps its own line one level past the call's line,
    // and `)` its own line at the call's indent.
    let w = word(91);
    assert_eq!(
        fmt_with(&format!("card {{ x = f([{w}, b]) }}"), &with_comma),
        format!("card {{\n  x = f([\n      {w},\n      b,\n    ],\n  ),\n}}\n")
    );
    // Without trailing commas the same list fits unhugged.
    assert_eq!(
        fmt_with(
            &format!("card {{ x = f([{w}, b]) }}"),
            &own_line(TrailingCommas::Never)
        ),
        format!("card {{\n  x = f(\n    [{w}, b]\n  )\n}}\n")
    );
    // A one-item list takes one line unhugged and two hugged, with its `]` on
    // its own line, so it does not hug.
    let w = word(94);
    assert_eq!(
        fmt_with(
            &format!("card {{ x = f([{w}]) }}"),
            &own_line(TrailingCommas::Never)
        ),
        format!("card {{\n  x = f(\n    [{w}]\n  )\n}}\n")
    );
}

#[test]
fn hug_first_list_off() {
    let long = word(40);
    let style = Style {
        hug_first_list: false,
        ..Style::default()
    };
    assert_eq!(
        fmt_with(
            &format!("card {{ x = and([{long}, {long}, {long}]) }}"),
            &style
        ),
        format!("card {{\n  x = and(\n    [\n      {long},\n      {long},\n      {long}])\n}}\n")
    );
}

// Comments.

#[test]
fn comments_in_every_sequence_kind() {
    let source = "\
# file comment
template t( # params
  x: A, # after x
  y: B = f(
    # default arg
    1)): S {
  # body
  a = [ # list
    1],
  b = { # record
    c = 1},
  d = g( # args
    2),
  # body end
}
";
    assert_eq!(
        fmt(source),
        "\
# file comment
template t(
  # params
  x: A,
  # after x
  y: B = f(
    # default arg
    1)): S {
  # body
  a = [
    # list
    1],
  b = {
    # record
    c = 1},
  d = g(
    # args
    2)
  # body end
}
"
    );
}

#[test]
fn a_trailing_comment_puts_the_closer_on_its_own_line() {
    assert_eq!(
        fmt("card { a = [1, 2 # two\n] }"),
        "card {\n  a = [\n    1,\n    2\n    # two\n  ]\n}\n"
    );
    assert_eq!(
        fmt("card { a = [# only\n] }"),
        "card {\n  a = [\n    # only\n  ]\n}\n"
    );
}

#[test]
fn a_comment_inside_an_item_moves_before_it() {
    assert_eq!(
        fmt("card {\n  a = # about a\n    1\n}"),
        "card {\n  # about a\n  a = 1\n}\n"
    );
}

#[test]
fn comments_are_verbatim() {
    assert_eq!(
        fmt("#no space\n#   indented \"x\"\ncard {}"),
        "#no space\n#   indented \"x\"\ncard {}\n"
    );
}

// Blank lines.

#[test]
fn a_recorded_blank_line_is_kept_as_one() {
    assert_eq!(
        fmt("relation a {}\n\n\n\nrelation b {}\nrelation c {}"),
        "relation a {}\n\nrelation b {}\nrelation c {}\n"
    );
    assert_eq!(
        fmt("card {\n  a = 1,\n\n\n  b = 2\n}"),
        "card {\n  a = 1,\n\n  b = 2\n}\n"
    );
}

#[test]
fn no_blank_line_at_the_start_or_end() {
    assert_eq!(
        fmt("\n\n# c\n\nrelation a {}\n\n\n"),
        "# c\n\nrelation a {}\n"
    );
    assert_eq!(
        fmt("card {\n\n  a = 1,\n  b = [\n\n    x,\n    y\n\n  ]\n\n}"),
        "card {\n  a = 1,\n  b = [x, y]\n}\n"
    );
}

#[test]
fn a_blank_line_inside_a_list_forces_it_to_break() {
    assert_eq!(
        fmt("card { a = [x,\n\ny] }"),
        "card {\n  a = [\n    x,\n\n    y]\n}\n"
    );
}

#[test]
fn a_doc_comment_stays_attached() {
    assert_eq!(
        fmt("# detached\n\n# doc\n# more\nrelation a {}"),
        "# detached\n\n# doc\n# more\nrelation a {}\n"
    );
}

// Parameter lists.

#[test]
fn a_parameter_list_that_fits_stays_flat() {
    assert_eq!(
        fmt("primitive p(\n  x: Option(A),\n  y: List(B) = []): S = Mk"),
        "primitive p(x: Option(A), y: List(B) = []): S = Mk\n"
    );
}

#[test]
fn a_broken_parameter_list_follows_the_call_rule() {
    // The header after `)` counts toward the last parameter's line.
    let long = word(30);
    let source = format!("primitive p(a: {long}, b: {long} = none): {long} = Mk");
    assert_eq!(
        fmt(&source),
        format!("primitive p(\n  a: {long},\n  b: {long} = none): {long} = Mk\n")
    );
    let style = Style {
        closers: Closers::OwnLine,
        trailing_commas: TrailingCommas::OwnLine,
        ..Style::default()
    };
    assert_eq!(
        fmt_with(&source, &style),
        format!("primitive p(\n  a: {long},\n  b: {long} = none,\n): {long} = Mk\n")
    );
    assert_eq!(
        fmt(&format!(
            "template t(a: {long}, b: {long}, c: {long}): S {{ body = $a }}"
        )),
        format!("template t(\n  a: {long},\n  b: {long},\n  c: {long}): S {{\n  body = $a\n}}\n")
    );
}

// Atoms.

#[test]
fn parameters_strings_and_integers() {
    assert_eq!(
        fmt("card { a = f($x, $y_2), s = \"say \\\"hi\\\" \\\\ bye\", n = [-12, 0, 7] }"),
        "card {\n  a = f($x, $y_2),\n  s = \"say \\\"hi\\\" \\\\ bye\",\n  n = [-12, 0, 7]\n}\n"
    );
}

#[test]
fn a_nameless_call_card() {
    let long = word(40);
    assert_eq!(
        fmt(&format!(
            "card transforming({{front = {{name = \"{long}\"}}, back = {{name = \"{long}\"}}}})"
        )),
        format!(
            "card transforming({{\n    front = {{name = \"{long}\"}},\n    back = {{name = \"{long}\"}}}})\n"
        )
    );
}

#[test]
fn an_empty_file_prints_nothing() {
    assert_eq!(fmt(""), "");
    assert_eq!(fmt("\n\n"), "");
}

// Options.

#[test]
fn closers_on_their_own_line() {
    let long = word(46);
    let source = format!("card {{ x = and([{long}, f({long}, {long})]), y = [] }}");
    let style = Style {
        closers: Closers::OwnLine,
        ..Style::default()
    };
    assert_eq!(
        fmt_with(&source, &style),
        format!(
            "card {{\n  x = and([\n      {long},\n      f(\n        {long},\n        {long}\n      )\n    \
             ]\n  ),\n  y = []\n}}\n"
        )
    );
}

#[test]
fn trailing_commas_settings() {
    let long = word(45);
    let source = format!("card {{ x = [{long}, {long}], y = 1 }}");
    let with = |closers, trailing_commas| {
        fmt_with(
            &source,
            &Style {
                closers,
                trailing_commas,
                ..Style::default()
            },
        )
    };
    assert_eq!(
        with(Closers::Hugged, TrailingCommas::Never),
        format!("card {{\n  x = [\n    {long},\n    {long}],\n  y = 1\n}}\n")
    );
    assert_eq!(
        with(Closers::Hugged, TrailingCommas::Bodies),
        format!("card {{\n  x = [\n    {long},\n    {long}],\n  y = 1,\n}}\n")
    );
    // With hugged closers only bodies close on their own line.
    assert_eq!(
        with(Closers::Hugged, TrailingCommas::OwnLine),
        with(Closers::Hugged, TrailingCommas::Bodies)
    );
    assert_eq!(
        with(Closers::OwnLine, TrailingCommas::Bodies),
        format!("card {{\n  x = [\n    {long},\n    {long}\n  ],\n  y = 1,\n}}\n")
    );
    assert_eq!(
        with(Closers::OwnLine, TrailingCommas::OwnLine),
        format!("card {{\n  x = [\n    {long},\n    {long},\n  ],\n  y = 1,\n}}\n")
    );
    assert_eq!(
        with(Closers::OwnLine, TrailingCommas::Never),
        format!("card {{\n  x = [\n    {long},\n    {long}\n  ],\n  y = 1\n}}\n")
    );
}

#[test]
fn a_trailing_comma_before_a_trailing_comment() {
    let style = Style {
        trailing_commas: TrailingCommas::OwnLine,
        ..Style::default()
    };
    assert_eq!(
        fmt_with("card { a = [1, 2 # two\n] }", &style),
        "card {\n  a = [\n    1,\n    2,\n    # two\n  ],\n}\n"
    );
}

#[test]
fn record_spaces_setting() {
    let style = Style {
        record_spaces: true,
        ..Style::default()
    };
    assert_eq!(
        fmt_with("card { a = {b = 1, c = {}}, d = [] }", &style),
        "card {\n  a = { b = 1, c = {} },\n  d = []\n}\n"
    );
}

#[test]
fn width_setting() {
    let style = Style {
        width: 20,
        ..Style::default()
    };
    assert_eq!(
        fmt_with("card { a = f(xxxxxx, yyyyyy) }", &style),
        "card {\n  a = f(\n    xxxxxx,\n    yyyyyy)\n}\n"
    );
}
