Usage: find [OPTIONS] [pattern] [path]

Arguments:
  [pattern]  the search pattern, a regular expression (optional)
  [path]...  the root directories for the filesystem search, default from cwd (optional)

Options:
  --hidden                     Search hidden files and directories
  --no-ignore                  Do not respect .(git)ignore files
  --ignore-case                Case-insensitive search (default: smart case)
  --follow                     Follow symbolic links
  --max-depth <depth>          Set maximum search depth (default: none)
  --exclude <pattern>          Exclude entries that match the given glob pattern
  --help                       Print help (see more with '--help')
  --version                    Print version

