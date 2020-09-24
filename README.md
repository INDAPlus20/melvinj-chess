# I can explain
I intended for this to be a prototype for creating chess, based on a Java project I made this spring
I underestimated how much time I would spend debugging
### Some known weak points
The java project was based on inheritance which was cumbersome to implement in Rust
It could be improved by completely scrapping the Piece trait and all it's inheritors
They could simply be replaced by functions for individual piece movements.
### API changes
- make_move, takes two strings of the format "xy", x belonging to {a,b,c,d,e,f,g,h} and y belonging to {1,2,3,4,5,6,7,8}. The Strings are represent from and to positions, respectively.
- set_promotion, when a pawn has reached the other end of the board, it will await a promotion. All other actions are blocked. The fn has one parameter, a String such as "rook", "nkight" or "queen"
- get_possible_moves, uses the same string format as make_move. The same format is returned in the Option<Vec<String>>
- print_game_state, prints the board to the console. 
### Proven test cases
- Skolmatt (Scholar's mate). In swedish to annoy everybody
- Promotion
- GameState after creation
- Check but no Checkmate
- Pawn Movement (including en passant)
