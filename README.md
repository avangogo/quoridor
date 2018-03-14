A programm that make program that play the game corridor play together*

# Usage

$ cargo run script_name1 script_name2

Launches the match script_name1 vs script_name2.

A missing argument is replaced by a random player (for now).
For instance the following command plays random vs random:

$ cargo run

# Example

$ cargo run players/toto.py
(will probably crash because of a forbiden move)

# Script

The playing script are called with three arguments : size, wall, player, where
- size is the length AND width of the board (so the board is a square)
- wall is the number of wall the player can play
- player is 0 if the script starts the game, 1 otherwise

For instance : 'script 9 10 0' for the starting player on a 9x9 board and each player has 10 walls

# Board notation
- Both cordinates range from 0 to size-1
- Player 0 start on (size/2, 0) and has to go to y=size-1
- Player 1 start on (size/2, size-1) and has to go to y=0

# Moves notation
There are two types of moves: moving and building a wall
A move is denoted by a single line:

'Move x y'

for moving to cell (x, y)

'Wall x y V' or 'Wall x y H'

for building a Vertical/Horizontal wall centered on center (x,y)
where 0 <= x,y < size - 2.

Ex: Wall 0 0 V 

(0, 0) | (1, 0)
       |
(0, 1) | (1, 1)
	
# Script behaviour

Once launched, the script has:
- write a move (with a line break) on stout if it is the active player
- wait for the oponent move (with a line break) on stdin otherwise