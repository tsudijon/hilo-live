# hilo-live
Online implementation of Hi-Lo.

## Game Description

N players play the game; best played if $N > 3, N < 8.$ N cards are dealt randomly from a standard deck, one to each player.

If there are any duplicates at the start of the game, the number of duplicate cards are identified -- for example, 1 group of 3 duplicates, or 2 groups of 2 duplicates, and given as information to all the players.

There are two rounds to the game. The first round is as follows: going around the circle, the players each state which numerical rank $1,\dots,N$ they think they are. Each of the other players can see others' cards, **but not their own.** They also have access to the other players' guesses about their ranks.

In the second round, in the same order as before, each player guesses the number of their card $(2,3,\dots,K,A)$. Each players' guess is public information. The card is then revealed to the player and the absolute difference between the guess and truth is calcualted, and added to a running total. The score of the group is the running total of these differences.