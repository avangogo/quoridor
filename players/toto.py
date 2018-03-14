#!/usr/bin/python3
import sys
import time

# Input :
# size : size of the board,
# walls : number of wall available,
# player : which player I am (0 or 1) (Player 0 starts)
size, walls, player = [int(i) for i in sys.argv[1:4]]

# My starting position
if player == 0 :
    start = [ int(size/2), 0 ]
else :
    start = [ int(size/2), size-1 ]

# If I am player 1, wait for the move of player 0
if player == 1:
    opponent_move = input()    

# Rush forward
pos = start
for i in range(size):
    pos[1] += 1-2*player

    # Print the move
    
    print("Move {} {}".format(pos[0], pos[1]))
    # print("Wall {} {} V".format(x, y))
    # print("Wall {} {} H".format(x, y))
    sys.stdout.flush()

    # Wait for oponent move
    opponent_move = input()
