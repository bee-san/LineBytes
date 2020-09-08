# LineBytes
**Update** I have no idea how to implement this as an iterator, so this project is on pause until then.

.lines() for non-UTF8 files by reading bytes and breaking on 0x0A (linefeed)

You want to read a file line by line.
That file isn't UTF8, so you cannot read into a string or use `.lines()`. 

This little library solves that problem.

This is something I made to solve my own problem, so if you have any requirements / want to improve it please submit a PR.
