# atcoder-judger
This is a simple judge tool for atcoder.

# What is it?
This tool can download sample cases and run them locally.

**Currently none of today works properly! It is under development so please wait for the release.**

**We are not currently checking Windows support.**

# Usage
## Download samplecases
```
atcoder-judger --download <ID>
```
`ID` means contest types and numbers.

ex: `abc111`, `arc001` etc...

## Run samplecases
```
atcoder-judger --test <PROBLEM>
```
`PROBLEM` means problem number.

You can choose one from a, b, c, d, e, f, g.

# Build
1. Clone repository
2. run `cargo build` in your terminal
3. run `./target/debug/atcoder-judger`
