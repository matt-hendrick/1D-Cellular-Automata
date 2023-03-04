# 1D Cellular Automata in the browser built with Rust/Yew/WASM

This repo generates and displays evolutions of [1D Cellular Automata in the browser](https://one-d-cellular-automata.web.app/). This project was built in Rust with the Yew framework and is compiled into WebAssembly (WASM).

For more info on 1D cellular automata and for visualizations of the outputs of the 256 rules, see [this Wolfram article](https://mathworld.wolfram.com/ElementaryCellularAutomaton.html).

---

## 1D Cellular Automata in the Command Line

![1D Cellular Automata GIF](1DCellularAutomataDemo.gif)

This repo also contains a previous iteration of this project that was a CLI app in "cmdLineAutomata.rs".

Three optional parameters can be passed in to affect the displayed output in the terminal.

## First Parameter (usize) = Evolution Rule (0-255)

A number between 0-255 can be passed in. This will have the automata evolve based upon the corresponding binary equivalent. If no parameter is provided, a random rule will be chosen.

## Second Parameter (u32) = Size of the Vector to Display in the Terminal

A number specifying the size of the vector that is displayed in the terminal. If no parameter is provided, a vector with 155 elements will be used.

## Third Parameter (bool) = Randomize Starting Vector Values

If a parameter is passed in, the starting vector will have a random mix of 0s and 1s. If no parameter is provided, only the vector's midpoint will be set to 1 (all other elements will be 0).
