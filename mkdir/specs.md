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


> About path, we handle the follwoing special cases:

    .. (dot-dot): Represents the parent directory.
    ~ (tilde): Represents the user's home directory.
    / (root): Represents the root directory.
    . (dot): Represents the current directory.
    (name) -> same as dot
    example: mkdir ./new_dir = mkdir new_dir 

