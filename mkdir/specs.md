NAME
       mkdir - make directories

SYNOPSIS
       mkdir [OPTION]... DIRECTORY...

DESCRIPTION
       Create the DIRECTORY(ies), if they do not already exist.

       Mandatory  arguments to long options are mandatory for short op‐
       tions too.

       -m=MODE
    set file mode (as in chmod), not a=rwx - umask

       -p
              no error if existing, make parent directories as  needed,
              with their file modes unaffected by any -m option.

       -v
              print a message for each created directory

       --help display this help and exit

       --version
              output version information and exit


> About path, we only create one directory at a time.
For complex paths, we use PathClean
See - https://docs.rs/path-clean/latest/path_clean/trait.PathClean.html

