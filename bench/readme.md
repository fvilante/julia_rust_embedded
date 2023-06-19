
# Presentation

The purpose of this script is to benchmark the variation of flash memory consumption along-side the development of the code between each commit to the repository.

This effect is obtained by running `make build size` on each commit
in the repository history.

# How to interpret the output

> For details see: <https://mcuoneclipse.com/2013/04/14/text-data-and-bss-code-and-data-size-explained/>

In summary:

* ‘text’ is all the code, vector table and constants that is stored in the FLASH memory.

* ‘data’ is for initialized variables (static/global variables), and it counts for RAM and FLASH. The linker allocates the `static` data in FLASH which then is copied from ROM to RAM in the startup code.

* ‘bss’ is for the uninitialized data in RAM which is initialized with zero in the startup code.

# Execution

Change to the script directory and type `cargo run` to execute the script.

By default the script emits the output to the screen (stdout) but you can redirect it to a file. For example:

```powershell
> cd scripts
> cargo run > output_filename.csv
```

The output is in the `csv` format.

# Example of output

```csv
hash,message,.text,.data,.bss
1135f2d,exclude `scripts` from workspace,30858,438,9
e3bf322,`const` in some functions,30858,438,9
08ed715,formating,30858,438,9
f88bde1,implemented more than one `Arquivo de Eixo`,30846,438,9
3fbae9a,naming,29458,438,9
d78e489,minor change,29458,438,9
4ddeb00,minor change,29458,438,9
cb72ce6,removed unused code,29458,438,9
8b324d6,doc,29458,438,9

// ... etc ...

```
