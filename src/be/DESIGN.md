# Design of the Fig Backend
The backend takes in an AST and Symbol Table.

## Tasks of the Backend
 - [**DONE**] Generating high level intermediate representation (three address code/HLIR)
 - [**DONE**] Breaking down the HLIR into basic blocks
 - Performing control flow analysis with those basic blocks
- Proper register allocation
 - Converting three address code into SSA
 - Optimize!
 - [**DONE**] Convert SSA into assembly
 - Peephole optimization
 - Invoking the linker (*manual right now*)

