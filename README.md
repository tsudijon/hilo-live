# hilo-live
Online implementation of Hi-Lo.

## Game Description

N players play the game; the game is best played if there are between 3 and 8 players. N cards are dealt randomly from a standard deck, one to each player.

If there are any duplicates at the start of the game, the number of duplicate cards are identified -- for example, 1 group of 3 duplicates, or 2 groups of 2 duplicates. This information is revealed to all the players.

There are two rounds to the game. The first round is as follows: going around the circle, the players each state which numerical rank (from 1 to N) they think they are. Each of the other players can see the others' cards, **but not their own.** They also have access to the other players' guesses about their ranks.

In the second round, in the same order as before, each player guesses the number of their card (2,3,...,K,A). Each player's guess is public information. The card is then revealed to the player. The absolute difference between the guess and truth is calculated, and added to a running total. The score of the group is the running total of these differences.