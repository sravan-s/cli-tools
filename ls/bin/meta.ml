let version = "0.0.1"

let help = "
TLDR -

SYNOPSIS         top
       ls [OPTION]... [FILE]...

DESCRIPTION         top
       List information about the FILEs (the current directory by
       default).  Sort entries alphabetically if none of -cftuvSUX nor
       --sort is specified.

       for now, support single args ie -> `ls -l -a`
       ie, no support for `ls -la`

       Mandatory arguments to long options are mandatory for short
       options too.

       -a, --all
              do not ignore entries starting with .

       -i, --inode
              print the index number of each file

       -l     use a long listing format

       -R, --recursive
              list subdirectories recursively

       -s, --size
              print the allocated size of each file, in blocks

       --sort=WORD
              sort by WORD instead of name: none, size, time,
              extension

       --help display this help and exit

       --version
              output version information and exit
"
