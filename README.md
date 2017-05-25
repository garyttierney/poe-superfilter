Path of Exile Superfilter
-------------------------

[![crates.io](https://img.shields.io/crates/v/poe-superfilter.svg)](https://crates.io/crates/poe-superfilter)
[![Travis CI](https://travis-ci.org/skaufhold/poe-superfilter.svg)](https://travis-ci.org/skaufhold/poe-superfilter)

A preprocessor for Path of Exile loot filters that adds variables, mixins, arithmetics and lots
of other useful things to GGG's loot filter syntax and compiles filters
written in the extended syntax down to a pure loot filter that can be used
in the game.

**You can download the current version on the [releases page](https://github.com/skaufhold/poe-superfilter/releases) of this repository.**

# Syntax

## Mixins

Mixins are reusable blocks of instructions that can be included anywhere.
They can be used to minimize duplication in repeating sections of filters.

```
Mixin FlaskMagic($type)
	Class Flask
	Rarity Magic
	BaseType $type
	SetTextColor 100 100 255 255
	SetBorderColor 100 100 100
	SetFontSize 38

Show
	+FlaskMagic("Small")
	ItemLevel <= 5

Show
	+FlaskMagic("Medium")
	ItemLevel <= 8
	ItemLevel >= 3
```
This compiles to:

```
Show
	Class Flask
	Rarity Magic
	BaseType "Small"
	SetTextColor 100 100 255 255
	SetBorderColor 100 100 100
	SetFontSize 38
	ItemLevel <= 5

Show
	Class Flask
	Rarity Magic
	BaseType "Medium"
	SetTextColor 100 100 255 255
	SetBorderColor 100 100 100
	SetFontSize 38
	ItemLevel <= 8
	ItemLevel >= 3
```


## Variables

You can define variables like this:

```
$var = "value"
$var2 = "this" "is" "a" "test"
$num = 10
```
They can contain any value that could be passed to filter instructions, including
lists of values.

Variable definitions are allowed in two places:

1. Before the first block

    Variables defined here will be visible globally.

2. Inside of a block

    If placed inside of a block, the variable will only be visible
    in that block and blocks nested inside of it, and only to instructions
    after the variable definition.

## Arithmetics

You can use simple mathematical expressions basically anywhere.

```
$scale = 1.2
Show
    SetFontSize 38 * $scale
```

Keep in mind that while you can use non-integer numbers, the end results of all calculations will be
rounded to integers during rendering, since GGG's loot filter syntax only supports integers.

This is also valid syntax, however you might want to add clarifying parentheses.

```
Show
    SetTextColor 100 * $foo 100 255 + $bar 255
    # Probably better like that:
    SetTextColor (100 * $foo) 100 (255 + $bar) 255
```

## Imports

You can import other Superfilter files with the import statement. Mixins and global variables defined
there will also be available in the including file.

```
Import "some_file.sf"
```

## Conditional Blocks

You can include or exclude blocks based on conditions. To express conditions, you can use simple equality and comparison
checks or boolean values.

```
$yes = True
$no = 1 > 2

Show if True # This block will be included
    SetStatement 123

Show if False # this won't
    SetStatement 234

Show if 2 > 3
    SetStatement 345

Show if 3 > 2
    SetStatement 456

Show if $yes
    SetStatement 567

Show if $no
    SetStatement 678
```

## Comments

Depending on your usage you might want the comments to pass through to the output or not. You can change this behaviour
using the `--comments` option. If it is given, comments are passed through to the output, otherwise they whill be discarded.

There is a small caveat for comments preceding Show/Hide blocks if you want the output to be formatted correctly:

```
Show
    # comment
    Class Flask
    # another comment

# comments for this following block
# more comments
Show
    Class Flask
    
---- This will actually produce the following output:

Show
    # comment
    Class Flask
    # another comment
    # comments for this following block
    # more comments

Show
    Class Flask
```

To avoid that behaviour, you can use a special syntax for "block comments":

```
Show
    # comment
    Class Flask
    # another comment

#! comment for this following block
#! more comments
Show
    Class Flask

----- Output:
Show
    # comment
    Class Flask
    # another comment
    
# comments for this following block
# more comments
Show
    Class Flask
```

It's done this way to make the syntax less ambiguous - without the explicit block comments, there could never be a comment
line at the end of a block, since there would be no way to differentiate it from one intended to describe the next block.

# Command Line Usage

```
USAGE:
    superfilter.exe [FLAGS] [OPTIONS] <PATH>

FLAGS:
    -c, --comments    Include comments in the output
    -h, --help        Prints help information
    -p, --pretty      Include indentation and other formatting in the output
    -V, --version     Prints version information

OPTIONS:
    -l, --line-endings <LINE_ENDING>    Type of line ending used (LF OR CRLF) defaults to the platform line ending
                                        [values: lf, crlf]
    -o, --output <FILE>                 Output file. If this option is omitted, the output will be printed to the
                                        console.

ARGS:
    <PATH>    Path of the input file
```

# Notes for script creators

If you intend to distribute a filter you created to other players, you might want to 
provide an easy way for them to recompile your script if they want to make some
changes to your filter.

How exactly you do this is up to you, but there is a PowerShell script
([compile-script.ps1](compile-script.ps1)) to help with this in the repository
that you can adapt and include with your filter. It simply compiles a specified file, but also
checks whether Superfilter is installed and whether it is a sufficiently recent version. In case
it isn't, it prints an error message and directs the user to the download page to install the tool.
To use the script, you just need to change the filenames in it to whatever your filter needs and then
execute it to recompile your filter.

# Roadmap

If you have suggestions, feel free to open an issue telling me all about it!

Things that are already planned, in no particular order:

* improved support for boolean expressions (add and/or operations on booleans)
