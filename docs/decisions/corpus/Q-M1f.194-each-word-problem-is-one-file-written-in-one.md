# Q-M1f.194 — Each word problem is one file, written in one form

`examples/word_problems/` holds one file for each distinct problem, and each problem in one form: where a word, a
gap or an assumption could be written several ways, the directory holds the problem once, in the form its theory
takes. The theory is four modules beside the problems, each imported by file name with one entry rule: `words.ein`,
what the words mean; `gaps.ein`, a quantity no sentence states; `readings.ein`, a word with two readings; and
`listed_is_all.ein`, the assumption *what is not listed is zero*.

**Why.** The directory is read to learn how a school problem becomes an Ein program; a problem written three ways
is one problem read three times. The modules are named for what they hold, so a problem's imports say which parts of
the theory it uses.

**Held by.** `examples/word_problems/README.md`, the table of problems and the forms the theory takes; the files'
entries in `corpus/corpus.toml`.
