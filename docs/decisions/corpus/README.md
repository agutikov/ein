# Decisions: the corpus and its tests

The manifest, fixtures and examples, the test harnesses of the engine and the stdlib, and regression tests. The
format and how a decision is cited: [`docs/decisions/README.md`](../README.md).

- [Q-M1a.9](Q-M1a.9-a-golden-lives-beside-the-crate-it-pins.md) — A golden lives beside the crate it pins
- [Q-M1e.25](Q-M1e.25-both-zebra-encodings-are-held-to-the-published.md) — Both Zebra encodings are held to the published grid
- [Q-M1e2.29](Q-M1e2.29-a-witness-of-a-fixed-behaviour-is-a-corpus.md) — A witness of a fixed behaviour is a corpus program where its check looks
- [Q-M1f.57](Q-M1f.57-every-fixture-the-loader-reads-carries-a-why.md) — Every fixture the loader reads carries a `:why`
- [Q-M1f.63](Q-M1f.63-a-program-a-test-writes-carries-a-why-on-every.md) — A program a test writes carries a `:why` on every form
- [Q-M1f.97](Q-M1f.97-a-test-closes-a-runs-stdout-before-the-run.md) — A test closes a run's stdout before the run starts
- [Q-M1f.154](Q-M1f.154-the-calculators-harness-is-exhaustive-to-120.md) — The calculator's harness is exhaustive to 120 and sampled to 30 digits
- [Q-M1f.162](Q-M1f.162-a-sampled-products-operands-have-at-most-19.md) — A sampled product's operands have at most 19 digits
- [Q-M1f.166](Q-M1f.166-the-generated-arithmetic-examples-are-sized-for.md) — The generated arithmetic examples are sized for every command
- [Q-M1f.178](Q-M1f.178-std-math-int-s-harness-is-a-file-of-its-own.md) — `std.math.int`'s harness is a file of its own, exhaustive on one-digit parts
- [Q-M1f.184](Q-M1f.184-std-math-rat-s-harness-exhausts-28-fractions.md) — `std.math.rat`'s harness exhausts 28 fractions, in programs of bounded size
- [Q-M1f.186](Q-M1f.186-what-the-calculators-tables-do-besides.md) — What the calculator's tables do besides arithmetic is beside `examples/arith/`
- [Q-M1f.187](Q-M1f.187-each-program-beyond-arithmetic-is-sized-for.md) — Each program beyond arithmetic is sized for the commands that read it
- [Q-M1f.190](Q-M1f.190-each-circuit-example-takes-the-simplest-form.md) — Each circuit example takes the simplest form its answer holds under
- [Q-M1f.193](Q-M1f.193-the-binary-examples-are-sized-for-every-command.md) — The binary examples are sized for every command
- [Q-M1f.194](Q-M1f.194-each-word-problem-is-one-file-written-in-one.md) — Each word problem is one file, written in one form
- [Q-M1f.195](Q-M1f.195-a-problem-that-asks-what-is-needed-imports.md) — A problem that asks what is needed imports the readings, whatever its numbers
- [Q-M1f.196](Q-M1f.196-each-module-of-the-word-theory-is-a-corpus-entry.md) — Each module of the word theory is a corpus entry of its own
- [Q-M1f.197](Q-M1f.197-the-corpus-holds-the-word-problems-answers.md) — The corpus holds the word problems' answers
- [Q-M1f.198](Q-M1f.198-the-thirteen-apple-problem-is-a-corpus-program.md) — The thirteen-apple problem is a corpus program
- [Q-M1f.329](Q-M1f.329-the-word-theory-works-backwards-from.md) — The word theory works backwards from the question
- [Q-M1f.330](Q-M1f.330-a-missing-quantity-is-a-fact-derived-by-absent.md) — A missing quantity is a fact derived by `absent`, in a module of its own
- [Q-M1f.331](Q-M1f.331-an-assumption-is-a-marked-fact-and-the-answer.md) — An assumption is a marked fact, and the answer says it rests on it
- [Q-M1f.332](Q-M1f.332-a-word-with-two-readings-always-has-one-chosen.md) — A word with two readings always has one chosen
- [Q-M1f.333](Q-M1f.333-what-is-not-listed-is-zero-only-where-a-problem.md) — What is not listed is zero only where a problem adopts it
- [Q-M1f.334](Q-M1f.334-the-word-theory-is-examples-not-a-module.md) — The word theory is examples, not a module of the stdlib
- [Q-M1f.335](Q-M1f.335-the-word-theory-holds-only-the-readings.md) — The word theory holds only the readings its problems need
- [Q-M1f.350](Q-M1f.350-a-fixed-wrong-answer-is-a-regression-test.md) — A fixed wrong answer is a regression test
