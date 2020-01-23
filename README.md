# Connect 4 with an implementation of Minimax + Alpha-Beta Pruning

* Occasionally functioning version of connect 4
* Has an implementation of minimax
* Has implementation of alpha-beta-pruning with minimax

## Flaws

 Cannot really see far enough ahead, in a reasonable amount of time.
Pit the minimax algorithm against itself to see it fall into the same trap every time.

### To Do

* If the algorithm believes that it is going to lose then it should try to delay the loss (in progress)
* Definitely needs some usability improvements.
* Code needs to be DRY-ied (especially minimax and alpha-beta methods which are nearly identical, but with a small difference. They need to streamlined into methods)
* The methods for get bounds of the evaluation function can theoretically be eliminated (primitive types have their own bounds).  This was required due to an sub-optimal implementation of the minimax algorithm.

[![Run on Repl.it](https://repl.it/badge/github/Andrew-Day-Coder/Connect-4-Rust)](https://repl.it/github/Andrew-Day-Coder/Connect-4-Rust)
