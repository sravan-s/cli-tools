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

       -Z     set SELinux security context of each created directory to
              the default type

       --context[=CTX]
              like -Z, or if CTX is specified then set the  SELinux  or
              SMACK security context to CTX

       --help display this help and exit

       --version
              output version information and exit

