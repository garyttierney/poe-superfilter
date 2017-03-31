Path of Exile Superfilter
----------

A preprocessor for Path of Exile loot filters that adds variables, mixins, arithmetics and lots
of other useful things to GGG's loot filter syntax and compiles filters
written in the extended syntax down to a pure loot filter that can be used
in the game.

```
Usage:
  superfilter <path> [--output=<file>] [-p]
  superfilter (-h | --help)
  superfilter --version

Options:
  -h --help        Show this screen.
  --version, -v    Show version.
  --pretty, -p     Include indentation and other formatting in the output
  --output=<file>  Output file.
```

Check out the [syntax documentation](doc/syntax.md) to get started.
