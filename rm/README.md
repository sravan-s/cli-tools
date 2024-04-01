## rm - remove files or directories

This is an attempt at rewriting rm for educational purposes

### Synopsis
rm [OPTION]... FILE...

### Description
rm removes each specified file. By default, it does not remove directories.

If the -I or --interactive=once option is given, and there are more than three files or the -r, -R, or --recursive are given, then rm prompts the user for whether to proceed with the entire operation. If the response is not affirmative, the entire command is aborted.

Otherwise, if a file is unwritable, standard input is a terminal, and the -f or --force option is not given, or the -i or --interactive=always option is given, rm prompts the user for whether to remove the file. If the response is not affirmative, the file is skipped.
Options

Remove (unlink) the FILE(s).

-f, --force
    ignore nonexistent files, never prompt 
-i
    prompt before every removal 
--one-file-system
    when removing a hierarchy recursively, skip any directory that is on a file system different from that of the corresponding command line argument 
--no-preserve-root
    do not treat '/' specially 
-r, -R, --recursive
    remove directories and their contents recursively 
-v, --verbose
    explain what is being done 
--help
    display this help and exit 
--version
    output version information and exit

By default, rm does not remove directories. Use the --recursive (-r or -R) option to remove each listed directory, too, along with all of its contents.

To remove a file whose name starts with a '-', for example '-foo', use one of these commands:

rm -- -foo
rm ./-foo

Note that if you use rm to remove a file, it is usually possible to recover the contents of that file. If you want more assurance that the contents are truly unrecoverable, consider using shred. 

From: https://linux.die.net/man/1/rm
Original authors -> 
Written by Paul Rubin, David MacKenzie, Richard M. Stallman, and Jim Meyering.

