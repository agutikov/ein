%grmtools {allow_wholeline_comments}
// Ein's tokens: S1f.5.5's final grammar, step 2 (T1f.5.5.2), 2026-11-08.
//
// lrlex takes the longest match at the cursor, and of two matches of one
// length the earlier rule. Every rule below leans on that:
//   - a reserved word is a whole token: `not` is the word, and `not-a`,
//     `nota` and `neq-1` are names, being longer (Q-M1e2.24, overriding
//     Q-M1e2.17 (i)(a));
//   - a keyword this grammar knows is its own token, and any other `:word`
//     is KEYWORD, which no production uses: an unknown keyword is a parse
//     error at its position (Q-M1e2.28, S1f.5.11's lists);
//   - `_` and `?_` are tokens of their own, used by no production, so each
//     is refused where it stands (Q-M1e2.22, S1f.5.1);
//   - a token runs to a separator: a space, a paren, `;`, `#|` or the end.
//     BAD, BADTEXT and BADESCAPE are the catch-alls that make it so (F2):
//     whatever is glued to a token makes one longer, bad, token.
// No production uses UNDERSCORE, VARUNDERSCORE, KEYWORD, BAD, BADTEXT or
// BADESCAPE; ein.y lists them under %expect-unused.
%%
\( "("
\) ")"
// The declarators and the kernel's connectives. Never a name.
not "not"
and "and"
or "or"
neq "neq"
rule "rule"
hrule "hrule"
query "query"
solve "solve"
config "config"
trace "trace"
macro "macro"
import "import"
// Words a position gives a meaning to. Each is also a name (ein.y's Name),
// so `(p model)` and `(relation step T)` still mean what they mean today.
relation "relation"
step "step"
eq "eq"
absent "absent"
false "false"
true "true"
open "open"
model "model"
region "region"
merged "merged"
owing "owing"
// The keywords, each the token of the forms that take it (S1f.5.11).
:why ":why"
:source ":source"
:goal\-text ":goal-text"
:match ":match"
:assert ":assert"
:priority ":priority"
:rule ":rule"
:using ":using"
:goal ":goal"
:over ":over"
:expect ":expect"
:hrules ":hrules"
:hypothesis\-relations ":hypothesis-relations"
:no\-hypothesis ":no-hypothesis"
:lattice\-sanity\-check ":lattice-sanity-check"
:record\-alternative\-justifications ":record-alternative-justifications"
:symbols ":symbols"
:as ":as"
:derives ":derives"
:assumes ":assumes"
// Reserved and refused: a bare `_`, and `?_`.
_ "UNDERSCORE"
\?_ "VARUNDERSCORE"
// A run of digits is a name (Q-M1e2.24); it is its own token only so that
// `:priority` can take digits and nothing else, until S1f.5.15 deletes it.
[0-9]+ "DIGITS"
// The name rule (Q-M1e2.24): ASCII letters and digits, the connectors
// `_ - .`, the decorators `$ % ^ * + = < > ! & @ ~ /`, and, for now,
// `' ` [ ] { }`. Nothing else, ever: no `? : ; # | , \ " ( )`, no space, no
// character outside ASCII.
[A-Za-z0-9_.\-$%^*+=<>!&@~\/'`\[\]{}]+ "NAME"
\?[A-Za-z0-9_.\-$%^*+=<>!&@~\/'`\[\]{}]+ "VAR"
:[A-Za-z0-9_.\-$%^*+=<>!&@~\/'`\[\]{}]+ "KEYWORD"
// A string: any Unicode, the escapes \n \t \r \" \\ and \u{…} (S1f.5.2).
// Any other escape is BADESCAPE; a string glued to what follows, BADTEXT.
"([^"\\]|\\[ntr"\\]|\\u\{[0-9A-Fa-f]{1,6}\})*" "TEXT"
"([^"\\]|\\[^\n])*" "BADESCAPE"
"([^"\\]|\\[^\n])*"[^\s();\x1c-\x1f]+ "BADTEXT"
// Everything else up to a separator. `#` ends a token only where `#|`
// opens a comment, as `;` does.
([^\s();#\x1c-\x1f]|#[^|\s();\x1c-\x1f])+ "BAD"
[\s\x1c-\x1f]+ ;
;[^\n]* ;
#\|([^|]|\|+[^|#])*\|+# ;
