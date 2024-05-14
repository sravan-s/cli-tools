## AWK clone

awk is a program that you can use to select particular records in a file and perform operations upon them.

awk — pattern scanning and processing language

Read -> https://www.gnu.org/software/gawk/manual/gawk.html for full specs

### We are implementing a subset of AWK that runs parallelly as map-reduce

awk -f program-file input-file1 input-file2 ...
awk -f program-file > applies awk to stdin

See ./grammar.md to see syntax of awk programs
See https://pubs.opengroup.org/onlinepubs/9699919799/utilities/awk.html#tab41 for operator precedence

### work done & Todo 
* Upto tokenizer
* Todo - Syntax tree from token list

#### Scoped out
- regexes
- Syntax sugar
    AddAssign +=
    SubAssign -=
    MulAssign *=
    DivAssign /=
    ModAssign %=
    PowAssign ^=
    Inc ++
    Dec --
- Code comments
