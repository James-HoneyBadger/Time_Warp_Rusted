# Examples

Sample programs for each of the seven languages supported by Time Warp Studio.

## Directory Structure

```
Examples/
  basic/     BASIC programs         (.bas)
  c/         C programs             (.c)
  forth/     Forth programs         (.fth / .f)
  logo/      Logo programs          (.logo)
  pascal/    Pascal programs        (.pas)
  pilot/     PILOT programs         (.pilot)
  prolog/    Prolog programs        (.pro)
  demo/      One demo per language  (mixed)
  fixtures/  Expected output files  (.in)
```

## Running Examples

Load any file through the **Examples** tab in the left panel of the IDE, or open the file in the editor and press **Run** (F5).

## Language / Extension Reference

| Language | Extension | Notes |
|----------|-----------|-------|
| BASIC    | `.bas`    | Line-numbered or free-format |
| PILOT    | `.pilot`  | CAI interaction language |
| Logo     | `.logo`   | Turtle geometry |
| C        | `.c`      | Subset interpreter |
| Pascal   | `.pas`    | Borland/ISO subset |
| Prolog   | `.pro`    | Facts, rules, queries |
| Forth    | `.fth` `.f` | Stack machine |

## Adding Examples

1. Create a file in the appropriate subdirectory.
2. Add an entry to `builtin_examples()` in `crates/tw_ui/src/feature_panels.rs` so it appears in the browser.
