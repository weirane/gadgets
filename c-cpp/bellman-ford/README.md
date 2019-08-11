# Bellman-Ford Algorithm
An implementation of graph using adjacency list and bellman-ford algorithm performed on the graph.

This project uses [catch](https://github.com/catchorg/Catch2) as the unit test tool, in Arch-based Linux distributions, use

    sudo pacman -S catch2

to install it, or you can download the [single header version](https://raw.githubusercontent.com/catchorg/Catch2/master/single_include/catch2/catch.hpp) to `catch2/catch.hpp` and then add `-I.` to `CXXFLAGS` in the `Makefile`.

To run the algorithm (not needed for testing), the graph data should be put in the file `./data`, or specified in `argv[1]`, which should have the following structure:

    <graph node count> <source node for B-F>
    <edge-from> <edge-to> <edge-weight>
    <edge-from> <edge-to> <edge-weight>
    ...

Example:

    5 0
    0 1 -1
    0 2 4
    1 2 3
    1 3 2
    1 4 2
    3 2 5
    3 1 1
    4 3 -3
