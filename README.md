# escape

Run arbitrary commands on your pc via http

## Example 

`cargo r --release`

Then open firefox `0.0.0.0:40566/ls` to execute `ls`

The command output will be both printed to the pc and the browser

Needless to say this is not safe xD

## Changelog

***0.1.3***

- handle pipes in commands, example:

		0.0.0.0:40566/ls | wc -l
