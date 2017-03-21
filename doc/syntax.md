
Syntax
-------------

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

You can define a variable like this:

```
$var = "value"
```

Variable definitions are allowed in two places.

1. Anywhere between blocks

    Variables defined here will be visible in all blocks following the variable definition.
    Note that the order is significant here, a variable won't be available in a block that precedes it.

2. Inside of a block

    If placed inside of a block, the variable will only be visible
    in that block and blocks nested inside of it, and only to instructions
    after the variable definition

## Arithmetics

You can use simple mathematical expressions basically anywhere.

```
$scale = 1.2
Show
    SetFontSize 38 * $scale
```

This is also valid syntax, however you might want to add clarifying parentheses.

```
Show
    SetTextColor 100 * $foo 100 255 + $bar 255
    # Probably better like that:
    SetTextColor (100 * $foo) 100 (255 + $bar) 255
```
