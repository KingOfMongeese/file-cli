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
