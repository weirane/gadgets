# x-sudoku

This is a simple diagonal sudoku solver with forward checking and a little
degree heuristic (during forward checking, the value is placed if it's the only
possible one).

To compile use `make`, or with optimization `make RELEASE=1`.

To run:

```sh
make run << EOF
6 7 0 0 0 0 0 0 9
0 9 0 6 0 0 0 0 3
0 3 0 0 7 0 0 0 0
0 0 1 0 0 0 0 0 0
7 0 0 0 0 0 0 0 1
0 0 0 0 0 0 4 0 0
0 0 0 0 6 0 0 2 0
4 0 0 0 0 3 0 8 0
3 0 0 0 0 0 0 5 7
EOF
```
