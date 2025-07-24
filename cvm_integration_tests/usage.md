# Usage of this test
## Purpose of this test
The executable located on the file `src/main.rs` of this crate has as purpose check
whether all the steps (parsing, typechecking and SSA form Control Flow Graph
[CFG] creation) are correct for a certain cvm file.

As input it takes a cvm file and as output it returns whether the compiler could
execute all the phases correctly and, in the case that it could, it also returns
the CFGs (one per template and function in the file) in JSON and DOT (for
visualization) format. These artefacts are then located in the same folder as the
cvm file taken as input.

## Run a single cvm file
To run a single file there are two options.
- With `cargo` is as simple as executing the following inside the project folder:
```
cargo run -p cvm_integration_tests <route/to/the/file.cvm>
```
- Compiling and running the main file of the crate `cvm_integration_tests`
    using as argument the route to the cvm file.

## Run a folder with multiple cvm files
To run all the cvm files (recursively) that are inside a folder there is a bash
script (`cvm_integration_tests/run_all_cvm.sh`) that works as follows:
1. It must be executed in the root folder of the project (the parent folder of
   `target/debug/`).
2. It needs as argument the folder where the cvm files to be compiled are
   located.
3. Once executed it will print which files where successfully compiled and which
   ones were not. Furthermore, it will create a `logs/` folder with the
   information about the files which failed in its compilation.

The execution command is:
```
bash cvm_integration_tests/run_all_cvm.sh <route/to/the/folder/>
```
The `logs/` folder will be located in the root folder of the project and the
`.json` and `.dot` with the information about the SSA form CFGs
of the successfully compiled files will be at the same folder as those cvm files.
