# Glassy

A simple CLI tool to apply glass-like overlay effect to images.
It adds variable blur and noise to image as overlay to give them a glassy look.

```
~ $ glassy -h

A simple CLI tool to apply glass-like overlay effect to images

Usage: glassy [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to image file

Options:
  -e, --effect-strength <EFFECT_STRENGTH>
          Strength of the glass effect [default: medium] [possible values: low, medium, high]
      --no-grain
          Apply effect without grain
  -v, --verbose
          Explain what is being done
  -h, --help
          Print help
  -V, --version
          Print version
```