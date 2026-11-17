%grmtools {allow_wholeline_comments}
// Ein's tokens, read by lrlex; `ein.y` is the grammar over them.
//
// lrlex takes the longest match at the cursor, and of two matches of one
// length the earlier rule. Every rule below leans on that:
//   - a reserved word is a whole token: `not` is the word, and `not-a`,
//     `nota` and `neq-1` are names, being longer (Q-M1e2.17,
//     Q-M1e2.24);
//   - a keyword this grammar knows is its own token, and any other `:word`
//     is KEYWORD, which no production uses: an unknown keyword is a parse
//     error at its position (Q-M1e2.28);
//   - `_` and `?_` are tokens of their own, used by no production, so each
//     is refused where it stands (Q-M1e2.22);
//   - a token runs to a separator: a space, a paren, `;`, `#|` or the end.
//     BAD, BADTEXT and BADESCAPE are the catch-alls that make it so:
//     whatever ASCII is glued to a token makes one longer, bad, token;
//   - a character that is not ASCII is written only inside a string (TEXT,
//     and BADESCAPE's, a string still) or a comment, the skip rule's
//     (Q-M1f.65): no other
//     rule matches one, so the lexer stops at it, and the parse error names
//     it, its code point and where Unicode is allowed (`ein_ir::grammar`);
//   - the engine's three writeback names are a token of their own, used by no
//     production, so each is refused where it stands (Q-M1f.7).
//   - after any token but a paren, only `(`, `)` and the rule that skips
//     whitespace and comments can match, by the bullet above; the state
//     AFTER_WORD tries those three alone, below.
// No production uses UNDERSCORE, VARUNDERSCORE, WRITEBACK, KEYWORD, BAD,
// BADTEXT or BADESCAPE; ein.y lists them under %expect-unused.
//
// AFTER_WORD, an exclusive start condition: entered after every token but
// `(` and `)`, and left for INITIAL after whatever matches in it. lrlex runs
// the regex of every rule the current state allows at every position, so a
// rule left out of a state costs nothing there; this one halves the lexing
// and moves no token. A token other than a paren is a run of characters BAD
// matches (or, after a string, BADTEXT does), which would go on, and so be
// the longer match, wherever an ASCII character other than whitespace, \x1c
// to \x1f, `(`, `)`, `;` or `#` followed it, or a `#` followed by a character
// BAD takes. So after one comes the end, one of those six where no rule but
// `(`, `)` and the skip rule can start (a `#` there opens a comment or starts
// no token at all, in either state), or a character that is not ASCII, where
// no rule starts in either state. Every rule but those three is tried in
// INITIAL alone, and the tokens, kind and span, are the ones the same rules give
// tried in one state (`generated_parser::the_lexer_gives_the_tokens_its_plain_form_does`).
%x AFTER_WORD
%%
<INITIAL,AFTER_WORD>\( <INITIAL>"("
<INITIAL,AFTER_WORD>\) <INITIAL>")"
// The declarators, the kernel's connectives and, below, `open`, the verdict
// (Q-M1f.66). Never a name.
not <AFTER_WORD>"not"
and <AFTER_WORD>"and"
or <AFTER_WORD>"or"
neq <AFTER_WORD>"neq"
rule <AFTER_WORD>"rule"
hrule <AFTER_WORD>"hrule"
query <AFTER_WORD>"query"
solve <AFTER_WORD>"solve"
config <AFTER_WORD>"config"
trace <AFTER_WORD>"trace"
macro <AFTER_WORD>"macro"
import <AFTER_WORD>"import"
// Words a position gives a meaning to. Each is also a name (ein.y's Name),
// so `(p model)` and `(relation step T)` mean what a name means; all but
// `open`, which is a name nowhere and is written only as a rule's whole
// :assert, `(open)` or `(open ?R)` (ein.y's Assert). Its rule stays among
// these, so that no token moves.
relation <AFTER_WORD>"relation"
step <AFTER_WORD>"step"
eq <AFTER_WORD>"eq"
absent <AFTER_WORD>"absent"
false <AFTER_WORD>"false"
true <AFTER_WORD>"true"
open <AFTER_WORD>"open"
model <AFTER_WORD>"model"
region <AFTER_WORD>"region"
merged <AFTER_WORD>"merged"
owing <AFTER_WORD>"owing"
// The keywords, each the token of the forms that take it.
:why <AFTER_WORD>":why"
:source <AFTER_WORD>":source"
:goal\-text <AFTER_WORD>":goal-text"
:match <AFTER_WORD>":match"
:assert <AFTER_WORD>":assert"
:rule <AFTER_WORD>":rule"
:using <AFTER_WORD>":using"
:goal <AFTER_WORD>":goal"
:over <AFTER_WORD>":over"
:expect <AFTER_WORD>":expect"
:hrules <AFTER_WORD>":hrules"
:hypothesis\-relations <AFTER_WORD>":hypothesis-relations"
:no\-hypothesis <AFTER_WORD>":no-hypothesis"
:no\-search <AFTER_WORD>":no-search"
:lattice\-sanity\-check <AFTER_WORD>":lattice-sanity-check"
:record\-alternative\-justifications <AFTER_WORD>":record-alternative-justifications"
:symbols <AFTER_WORD>":symbols"
:as <AFTER_WORD>":as"
:derives <AFTER_WORD>":derives"
:assumes <AFTER_WORD>":assumes"
// Reserved and refused: a bare `_`, and `?_`.
_ <AFTER_WORD>"UNDERSCORE"
\?_ <AFTER_WORD>"VARUNDERSCORE"
// Reserved and refused: the engine's three writeback rule names, which it
// writes as the rule of a conclusion no rule drew (`ein_core::terms::ENGINE`).
// Each is a NAME of the same length, so this rule, being earlier, wins; a
// longer run (`<forced-positive>x`) is the NAME it spells. They are reserved
// names (Q-M1f.7): no meaning a program can write, so refused wherever
// written. `[<]` and `[>]`, since a rule that starts with `<` names a start
// condition and `\<` is a word boundary.
[<](forced\-positive|monotonic\-unconditional|lookahead\-dies\-immediately)[>] <AFTER_WORD>"WRITEBACK"
// The name rule (Q-M1e2.24): a run of digits is a name like any other, and
// no token of its own: no keyword of this grammar takes digits alone. ASCII
// letters and digits, the connectors `_ - .`, the decorators
// `$ % ^ * + = < > ! & @ ~ /`, and, for now, `' ` [ ] { }`. Nothing else,
// ever: no `? : ; # | , \ " ( )`, no space, no character outside ASCII.
[A-Za-z0-9_.\-$%^*+=<>!&@~\/'`\[\]{}]+ <AFTER_WORD>"NAME"
\?[A-Za-z0-9_.\-$%^*+=<>!&@~\/'`\[\]{}]+ <AFTER_WORD>"VAR"
:[A-Za-z0-9_.\-$%^*+=<>!&@~\/'`\[\]{}]+ <AFTER_WORD>"KEYWORD"
// A string: any Unicode, the escapes \n \t \r \" \\ and \u{…}.
// \u{…} is one to six hex digits naming a Unicode scalar value: 0 to 10FFFF
// and not a surrogate, D800 to DFFF, which names no character
// (W38). The six alternatives are the values of one to six significant
// digits, each after the leading zeros that keep it within six; the fourth
// leaves out D800 to DFFF.
// Any other escape is BADESCAPE; a string glued to what follows, BADTEXT,
// whose tail is ASCII, as BAD's is.
"([^"\\]|\\[ntr"\\]|\\u\{(0{0,5}[0-9A-Fa-f]|0{0,4}[1-9A-Fa-f][0-9A-Fa-f]|0{0,3}[1-9A-Fa-f][0-9A-Fa-f]{2}|0{0,2}([1-9A-Ca-cEeFf][0-9A-Fa-f]{3}|[Dd][0-7][0-9A-Fa-f]{2})|0?[1-9A-Fa-f][0-9A-Fa-f]{4}|10[0-9A-Fa-f]{4})\})*" <AFTER_WORD>"TEXT"
"([^"\\]|\\[^\n])*" <AFTER_WORD>"BADESCAPE"
"([^"\\]|\\[^\n])*"[\x00-\x7F--[\s();\x1c-\x1f]]+ <AFTER_WORD>"BADTEXT"
// Everything else up to a separator. `#` ends a token only where `#|`
// opens a comment, as `;` does. ASCII only: a character that is not ASCII
// outside a string and a comment matches no rule, so the lexer stops there,
// and the parse error names it (`ein_ir::grammar`'s NOT_ASCII), rather than
// BAD taking it into a token that is not a name.
([\x00-\x7F--[\s();#\x1c-\x1f]]|#[\x00-\x7F--[|\s();\x1c-\x1f]])+ <AFTER_WORD>"BAD"
// What separates tokens and names nothing: a run of whitespace (ASCII's,
// \t \n \v \f \r and the space, and \x1c to \x1f), `;` comments to the end
// of the line and `#| … |#` comments, as one rule, so that a run of them is
// one position, where three rules, one for whitespace, `;[^\n]*` and
// `#\|([^|]|\|+[^|#])*\|+#`, would make it one per piece. No named rule
// matches where any of the three does, so the tokens are the ones the three
// give (`generated_parser::the_lexer_gives_the_tokens_its_plain_form_does`). A comment
// holds any character, Unicode's among them (Q-M1f.65); whitespace is
// ASCII's, so a no-break space, U+3000 or U+2028 is no separator: a reader
// could not tell one from a space or a line break (W73).
<INITIAL,AFTER_WORD>([\x09-\x0D\x20\x1C-\x1F]|;[^\n]*|#\|([^|]|\|+[^|#])*\|+#)+ <INITIAL>;
