#!/usr/bin/python3
import sys
import time

size, walls, player = [int(i) for i in sys.argv[1:4]]

# My starting position
if player == 0 :
    start = [ int(size/2), 0 ]
else :
    start = [ int(size/2), size-1 ]

pos = start
for i in range(size):
    pos[1] += 1-2*player
    
    print("Move {} {}".format(pos[0], pos[1]))
    sys.stdout.flush()


print("Bad move")
