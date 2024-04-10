## AWK clone

awk is a program that you can use to select particular records in a file and perform operations upon them.

awk — pattern scanning and processing language

Read -> https://www.gnu.org/software/gawk/manual/gawk.html for full specs

### We are implementing a subset of AWK that runs parallelly as map-reduce

awk -f program-file input-file1 input-file2 ...
awk -f program-file > applies awk to stdin

See grammar.md to see syntax of awk programs
Maybe I will scope down while implementing

