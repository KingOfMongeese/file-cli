A Simple CLi for working with files

# Usage

```console
>.\file-cli.exe --help
A simple file util cli

Usage: file-cli.exe <COMMAND>

Commands:
  write  Write to a file
  read   Read data from a file, print it to console
  help   Print this message or the help of the given subcommand(s)

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
