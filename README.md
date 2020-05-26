# game_of_life
Conway's Game of Life + bitboards.

This is a Rust port of a short Java program by Browne and Tavener (2012) that compacts an 8x8 representation of Conway's Game of Life into a 64 bit integer.

Includes a short function to turn a u64 into its chessboard representation, which is honestly my main takeaway from this program. Will come in handy for chess bitboards! 

Sample output of a glider:
```
  +------------------------+
8 |                        |
7 |                        |
6 |          █             |
5 |             █          |
4 |       █  █  █          |
3 |                        |
2 |                        |
1 |                        |
  +------------------------+
    a  b  c  d  e  f  g  h
  +------------------------+
8 |                        |
7 |                        |
6 |                        |
5 |       █     █          |
4 |          █  █          |
3 |          █             |
2 |                        |
1 |                        |
  +------------------------+
    a  b  c  d  e  f  g  h
  +------------------------+
8 |                        |
7 |                        |
6 |                        |
5 |             █          |
4 |       █     █          |
3 |          █  █          |
2 |                        |
1 |                        |
  +------------------------+
    a  b  c  d  e  f  g  h
  +------------------------+
8 |                        |
7 |                        |
6 |                        |
5 |          █             |
4 |             █  █       |
3 |          █  █          |
2 |                        |
1 |                        |
  +------------------------+
    a  b  c  d  e  f  g  h
  +------------------------+
8 |                        |
7 |                        |
6 |                        |
5 |             █          |
4 |                █       |
3 |          █  █  █       |
2 |                        |
1 |                        |
  +------------------------+
    a  b  c  d  e  f  g  h

```
