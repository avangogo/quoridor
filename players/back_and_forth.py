#!/usr/bin/python3
import sys
import time

size, walls, player = [int(i) for i in sys.argv[1:4]]

# My starting position
if player == 0 :
    start = [ int(size/2), 0 ]
    end = [ int(size/2)+1, 0 ]
else :
    start = [ int(size/2), size-1 ]
    end = [ int(size/2)+1, size-1 ]
    
if player == 1:
    opponent_move = input()    

i = 0
x = [ start, end ]
while True:
    i += 1

    pos = x[i % 2]
    print("Move {} {}".format(pos[0], pos[1]))
    sys.stdout.flush()

    opponent_move = input()
