
## Variables

Use variables to keep values like colors or font sizes that are shared by
many different items in your filter in a central place.

```
$scale = 1.2
$volume = 1.0

Show
	Class Divination
	BaseType "The Void"
	SetTextColor 0 0 0
	SetBorderColor 0 0 0 255
	SetBackgroundColor 150 0 255 255
	SetFontSize 42 * $scale
	PlayAlertSound 2 (300 * $volume)
```

## Mixins

Mixins are reusable blocks of instructions that can be included anywhere.
They can be used to minimize duplication in repeating sections of filters:

```
@mixin FlaskMagic($type)
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
Compiles to:

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
