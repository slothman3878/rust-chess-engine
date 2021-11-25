# Simple Chess Engine in Rust

For learning Rust

## Design

A chess move is considered as a transition function on the game state. The game state can be represented as

1. play stage (ex. ongoing, checkmate, stalemate)
2. board state (ex. the board grid plus misc information such as castling possibility, check state)


## Considerations

### Determining Check state

The engine keeps track of the position of the two kings.
For a given king's position, there are at most 22 potential positions for a opposition piece to check a king.

## Valid Move Conditions

A Move is Valid if

1. The pieces moves to a reachable square
2. Does **NOT** result in an opponent's check