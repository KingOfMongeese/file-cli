A Simple CLi for working with files

# Usage

```console
.\file-cli.exe --help
A simple file util cli

Usage: file-cli.exe <COMMAND>

Commands:
  write        Write to a file
  read         Read data from a file, print it to console
  append       Append data to a file
  make-dirs    Make Directories
  remove-dirs  Remove Directories and Content
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

# Write
Write to a file

## Usage

```console
>.\file-cli.exe write --help
Write to a file

Usage: file-cli.exe write -o <OUT_FILE> <TEXT>

Arguments:
  <TEXT>  The text to write

Options:
  -o <OUT_FILE>      The file to write to, will create all paths if they dont exist
  -h, --help         Print help
```

## Example

```console
> .\file-cli.exe write "This is some data\nNext Line of Data\nYet another line" -o test.txt
```

File test.txt:
```console
This is some data
Next Line of Data
Yet another line
```

# Read

## Usage
```
> .\file-cli.exe read --help
Read data from a file, print it to console

Usage: file-cli.exe read [OPTIONS] <IN_FILE>

Arguments:
  <IN_FILE>  the file to read

Options:
  -n          show the line numbers when printing
  -h, --help  Print help
```

## Example

File test.txt:
```console
This is some data
Next Line of Data
Yet another line
```

```console
> .\file-cli.exe read .\test.txt
This is some data
Next Line of Data
Yet another line
```

or

```console
.\file-cli.exe read .\test.txt -n
1. This is some data
2. Next Line of Data
3. Yet another line
```
# Append

## Usage
```
>.\file-cli.exe append --help
Append data to a file

Usage: file-cli.exe append [OPTIONS] -o <OUT_FILE> <TEXT>

Arguments:
  <TEXT>  the text to write

Options:
  -o <OUT_FILE>      the file to append to
  -n                 include line numbers
  -h, --help         Print help
```

## Example

test.txt:
```console
This is some data
Next Line of Data
Yet another line
```

```.\file-cli.exe append -o .\test.txt "myline\nsome more lines"```

test.txt:
```console
This is some data
Next Line of Data
Yet another line
myline
some more lines
```

# Make-dirs

## Usage
```
>.\file-cli.exe make-dirs --help
Make Directories

Usage: file-cli.exe make-dirs <PATH>

Arguments:
  <PATH>  the path to make

Options:
  -h, --help  Print help
```

## Example

```.\file-cli.exe make-dirs dir/more/dirs/and/dirs```

file system now has: ```dir\more\dirs\and\dirs```

# Remove-dirs

## Usage

```console
.\file-cli.exe remove-dirs --help
Remove Directories and Content

Usage: file-cli.exe remove-dirs [OPTIONS] <PATH>

Arguments:
  <PATH>  the path to remove

Options:
  -p          print the contents that will be removed
  -h, --help  Print help
```

## Example

file system has: ```dir\more\dirs\and\dirs```

```console
 .\file-cli.exe remove-dirs dir -p
Removing: dir
Removing: dir\more
Removing: dir\more\dirs
Removing: dir\more\dirs\and
Removing: dir\more\dirs\and\dirs
```

All dirs have been removed

# Show-info

shows information about a file/dir

## Usage

```console
 .\file-cli.exe show-info --help
Show info about a file path

Usage: file-cli.exe show-info <PATH>

Arguments:
  <PATH>  the path to show info about

Options:
  -h, --help  Print help
```

## Example

```console
..\file-cli.exe show-info 'Rust\file-cli\'
+---------------------------------------------------------------------+-----------+-----------------------------------+-----------------------------------+------------------------------------------------------------------+
| Name                                                                | Size      | Created At                        | Modified At                       | sha256 Hash                                                      |
+---------------------------------------------------------------------+-----------+-----------------------------------+-----------------------------------+------------------------------------------------------------------+
| C:\Users\Mongoose\Desktop\Programing Files\Rust\file-cli\.git       | 64.5 KiB  | 2025-03-15 18:34:18.938631800 UTC | 2025-04-05 01:13:31.094656600 UTC |                                                                  |
+---------------------------------------------------------------------+-----------+-----------------------------------+-----------------------------------+------------------------------------------------------------------+
| C:\Users\Mongoose\Desktop\Programing Files\Rust\file-cli\.gitignore | 8 B       | 2025-03-15 18:34:19.038610400 UTC | 2025-03-15 18:34:19.038610400 UTC | 44c92e3a70ad3307b7056871c2bdb096d8bfa9373f5bf06a79bb6324a20ff2fb |
+---------------------------------------------------------------------+-----------+-----------------------------------+-----------------------------------+------------------------------------------------------------------+
| C:\Users\Mongoose\Desktop\Programing Files\Rust\file-cli\Cargo.lock | 24.2 KiB  | 2025-03-15 18:39:00.964683100 UTC | 2025-04-04 23:59:52.728084100 UTC | 26aaabe9be59cddd9d02dccd57be2beb7e49ceb124e2c726932ac2a83fc328dd |
+---------------------------------------------------------------------+-----------+-----------------------------------+-----------------------------------+------------------------------------------------------------------+
| C:\Users\Mongoose\Desktop\Programing Files\Rust\file-cli\Cargo.toml | 230 B     | 2025-04-05 00:01:41.633316100 UTC | 2025-04-05 00:01:41.633316100 UTC | facbfd997218cda2099f7116f444a5898550259cc2e593b6c56858d735ed372e |
+---------------------------------------------------------------------+-----------+-----------------------------------+-----------------------------------+------------------------------------------------------------------+
| C:\Users\Mongoose\Desktop\Programing Files\Rust\file-cli\README.md  | 3.4 KiB   | 2025-03-15 19:57:41.258585900 UTC | 2025-04-05 01:15:33.622973400 UTC | 74a30861fe71b210ac57f801bf88c2a287530e50769995ebc5c3c35c13ca1000 |
+---------------------------------------------------------------------+-----------+-----------------------------------+-----------------------------------+------------------------------------------------------------------+
| C:\Users\Mongoose\Desktop\Programing Files\Rust\file-cli\src        | 8.4 KiB   | 2025-03-15 18:34:19.038610400 UTC | 2025-03-15 18:53:07.593308100 UTC |                                                                  |
+---------------------------------------------------------------------+-----------+-----------------------------------+-----------------------------------+------------------------------------------------------------------+
| C:\Users\Mongoose\Desktop\Programing Files\Rust\file-cli\target     | 542.5 MiB | 2025-03-15 18:39:01.829034200 UTC | 2025-03-15 19:56:35.703829100 UTC |                                                                  |
+---------------------------------------------------------------------+-----------+-----------------------------------+-----------------------------------+------------------------------------------------------------------+
```

