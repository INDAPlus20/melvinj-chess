use std::fmt;
use std::collections::HashMap;

pub mod movecheck;//I can't figure out how to use this properly
//Where to put structs etc.

//Chess attempt #1
//This will be entirely written in 1 file
//If this ever gets close to working order I will probably start over

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

/* IMPORTANT:
* - Document well!
* - Write well structured and clean code!
*/

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    white_turn: bool,
    board: Vec<Piecedata>
}

#[derive(Clone)]
struct Piecedata {
    //Struct containing data for pieces
    position: Position,
    is_alive: bool,
    is_white: bool,
    variant: String,
    moved: bool,
    enpassantable: bool
}

impl Piecedata {
    //Constructor for Piecedata
    fn new(position: Position, is_white: bool, variant: String) -> Self{
        Piecedata {position: position, is_alive: true, is_white: is_white, variant: variant, enpassantable: false, moved: false}
    }
    fn specific_new(position: Position, is_alive: bool, is_white: bool, variant: String, enpassantable: bool, moved: bool) -> Self{
        Piecedata {position: position, is_alive: is_alive, is_white: is_white, variant: variant, enpassantable: false, moved: false}
    }
}

fn make_king(data: &Piecedata) -> Option<King>{
    if data.variant == "king"{ 
        return Some(King::new(data.position.clone(),data.is_white))
    }else {
        None
    }
}

fn make_pawn(data: &Piecedata) -> Option<Pawn>{
    if data.variant == "king"{ 
        return Some(Pawn::new(data.position.clone(),data.is_white))
    }else {
        None
    }
}

fn move_check_a(game: &Game, m: &Move) -> bool{
    //Elementary checks for making a move
    /*
    * Coords out of bounds
    * There is a piece to be moved
    * The piece belongs to the current player
    * Is the targeted piece of the same color as the attacking piece?
    * 
    
    */
    
    //Check if all positions are in bounds
    if m.start_pos.x < 0 || m.start_pos.x > 7 {
        return false
    }
    if m.start_pos.y < 0 || m.start_pos.y > 7{
        return false
    }
    if m.end_pos.x < 0 || m.end_pos.x > 7 {
        return false
    }
    if m.end_pos.y < 0 || m.end_pos.y > 7{
        return false
    }
    
    //Find the piece to be moved
    let white_turn = game.white_turn;
    let mut piece_opt:Option<&mut Piecedata>;
    
    //In order to reduce the scope of the temp_game reference
    
    if white_turn == game.piece_at_pos_is_white(&m.start_pos){
        return false;
    }
    if game.piece_at_pos_bool(&m.end_pos){
        if white_turn != game.piece_at_pos_is_white(&m.end_pos){
            return false;
        }
    }
    true
}

fn move_check_b(game: &Game, n: &Move) -> bool{
    let m:Move = Move::new(n.start_pos.clone(),n.end_pos.clone());
    //Try the move
    let target_exists = game.piece_at_pos_bool(&m.end_pos);
    let checked: bool;
    let mut temp_game: Game = game.clone();
    
    if target_exists {
        //There is a piece at the target
        let attacker_board_index = &temp_game.index_of_piece_in_board(&m.start_pos);
        let target_board_index = &temp_game.index_of_piece_in_board(&m.end_pos);
        let mut attacker_white: bool = true;
        {
            let board = &mut temp_game.board;
            if attacker_board_index > target_board_index{
                let (target_component, attacker_component) = board.split_at_mut(attacker_board_index.unwrap()+1);
                for maybe_target in target_component{
                    if maybe_target.position.to_string() == m.end_pos.to_string(){
                        for maybe_attacker in attacker_component{
                            if maybe_attacker.position.to_string() == m.start_pos.to_string(){
                                maybe_target.is_alive = false;
                                
                                maybe_attacker.position = m.end_pos.clone();
                                attacker_white = maybe_attacker.is_white;
                                break;
                            }
                        }
                        
                        break;
                    }
                }
            }else {
                let (attacker_component, target_component) = board.split_at_mut(target_board_index.unwrap()+1);
                for maybe_target in target_component{
                    if maybe_target.position.to_string() == m.end_pos.to_string(){
                        for maybe_attacker in attacker_component{
                            if maybe_attacker.position.to_string() == m.start_pos.to_string(){
                                maybe_target.is_alive = false;
                                
                                maybe_attacker.position = m.end_pos.clone();
                                attacker_white = maybe_attacker.is_white;
                                break;
                            }
                        }
                        
                        break;
                    }
                }
            }
            checked = temp_game.check_for_check(attacker_white);
        }
        /*let mut target = game.piece_at_pos(&m.end_pos).unwrap();
        target.is_alive = false;
        
        game.piece_at_pos(&m.start_pos).unwrap().position = m.end_pos.clone();
        
        checked = game.check_for_check(game.piece_at_pos(&m.end_pos).unwrap().is_white);
        
        game.piece_at_pos(&m.end_pos).unwrap().position = m.start_pos.clone();
        
        target.is_alive = true;*/
    }else{
        game.piece_at_pos(&m.start_pos).unwrap().position = m.end_pos.clone();
        
        
        checked = game.check_for_check(game.piece_at_pos(&m.end_pos).unwrap().is_white);
        
        //Revert (this function is only for checking if the move is valid)
        game.piece_at_pos(&m.end_pos).unwrap().position = m.start_pos.clone();
        
    }
    checked
}



//Is this the closest thing to an abstract class in java or something?
trait Piece {
    fn new(position: Position, is_white: bool) -> Self;
    
    fn is_alive(&self) -> bool;
    
    fn is_move_allowed(self, game: &Game, m: Move) -> bool;

    fn secondary_is_move_allowed(self, game: &Game, m: Move) -> bool;
    
    fn doMove(self, g: &Game, m: Move);
}

struct Move {
    start_pos: Position,
    end_pos: Position
}

/*struct MoveProperties {
    //Booleans remembering if this is a special move
    //Can be replaced with enums, if I can get around
    //to understanding them
    is_castling: bool,
    is_en_passant: bool,
    is_double_step: bool
}*/

impl Move {
    fn new(p1: Position, p2: Position) -> Self{
        Move {start_pos: p1, end_pos: p2}
    }
    
    fn to_string_vec(&self) -> Vec<String>{
        //Returns the move in string form, eg. a2-a3
        let mut vec: Vec<String> = Vec::new();
        vec.push(self.start_pos.to_string());
        vec.push(self.end_pos.to_string());
        vec
    }
}

#[derive(Clone)]
struct Position {
    x: u8,
    y: u8
}

impl Position {
    fn new(x: u8, y: u8)->Self{
        Position{x:x,y:y}
    }
    fn to_string(&self) -> String{
        let mut string = String::with_capacity(2);
        match self.x {
            0 => string.push('a'),
            1 => string.push('b'),
            2 => string.push('c'),
            3 => string.push('d'),
            4 => string.push('e'),
            5 => string.push('f'),
            6 => string.push('g'),
            7 => string.push('h'),
            _ => eprintln!("VALUE OUT OF RANGE, POS->STRING")
        }
        match self.y {
            0 => string.push('1'),
            1 => string.push('2'),
            2 => string.push('3'),
            3 => string.push('4'),
            4 => string.push('5'),
            5 => string.push('6'),
            6 => string.push('7'),
            7 => string.push('8'),
            _ => eprintln!("VALUE OUT OF RANGE, POS->STRING")
        }
        
        string
    }
    
    pub fn clone(&self)->Position{
        Position::new(self.x,self.y)
    }
}

struct Pawn {
    piece: Piecedata,
    moved: bool
}

impl Piece for Pawn {
    fn new(position: Position, is_white: bool) -> Self {
        let piecedata = Piecedata::new(position, is_white, String::from("pawn"));
        Pawn {piece: piecedata, moved: false}
    }
    
    fn is_alive(&self)->bool{
        self.piece.is_alive
    }
    
    fn is_move_allowed(self, game: &Game, m: Move) -> bool{
        //Boiler plate
        if !move_check_a(game, &m) {
            return false
        }
        
        //Unique code for pawn movement
        if distance(m.start_pos.y, m.end_pos.y) == 2 && distance(m.start_pos.x,m.end_pos.x) == 0{
            //Initial double step
            if self.moved{
                return false
            }
            let avg_y: u8 = (m.start_pos.y + m.end_pos.y)/2; 
            match game.piece_at_pos(&m.end_pos){
                Some(_) => return false,
                None => ()
            }
            let temp_pos: Position = Position::new(m.start_pos.x, avg_y);
            match game.piece_at_pos(&temp_pos) {
                Some(_) => return false,
                None => ()
            }
            
        }
        //TODO fix
        if !distance(m.start_pos.x, m.end_pos.x) <= 1 && !distance(m.start_pos.y, m.end_pos.y) <= 1{
            return false
        }
        
        //Check intermediary positions
        //-No intermediary positions for regular pawn movement
        
        return move_check_b(game, &m);
        //Return result from checkCheck
    }

    fn secondary_is_move_allowed(self, game: &Game, m: Move) -> bool{
        //Boiler plate
        if !move_check_a(&mut game, &m) {
            return false
        }
        
        //Unique code for pawn movement
        if distance(m.start_pos.y, m.end_pos.y) == 2 && distance(m.start_pos.x,m.end_pos.x) == 0{
            //Initial double step
            if self.moved{
                return false
            }
            let avg_y: u8 = (m.start_pos.y + m.end_pos.y)/2; 
            match game.piece_at_pos(&m.end_pos){
                Some(_) => return false,
                None => ()
            }
            let temp_pos: Position = Position::new(m.start_pos.x, avg_y);
            match game.piece_at_pos(&temp_pos) {
                Some(_) => return false,
                None => ()
            }
            
        }
        //TODO fix
        if !distance(m.start_pos.x, m.end_pos.x) <= 1 && !distance(m.start_pos.y, m.end_pos.y) <= 1{
            return false
        }
        
        //Check intermediary positions
        //-No intermediary positions for regular pawn movement
        true
    }
    
    fn doMove(mut self, g: &Game, m: Move){
        let killed_piece = g.piece_at_pos(&m.end_pos);
        match killed_piece{
            //Kill the target, if it exists
            Some(mut kp) => kp.is_alive = false,
            None => ()
        }
        g.piece_at_pos(&m.start_pos).unwrap().position = m.end_pos;
        self.piece.moved = true;
    }
}

struct King {
    piece: Piecedata
}

impl King {
    }
        
        impl Piece for King {
            
            fn new(position: Position, is_white: bool) -> Self {
                let piecedata = Piecedata::new(position, is_white, String::from("king"));
                King {piece: piecedata}
            }
            
            fn is_alive(&self) -> bool {
                self.piece.is_alive
            }
            
            fn is_move_allowed(self, game: &Game, m: Move) -> bool{
                //A king should never move more than one step in any direction in one move¨
                //Except for castling, which can be added later.
                
                //Boiler plate
                if !move_check_a(game, &m) {
                    return false
                }
                
                //Unique code for king movement
                if !distance(m.start_pos.x, m.end_pos.x) <= 1 && !distance(m.start_pos.y, m.end_pos.y) <= 1{
                    return false
                }
                
                //Write castling code here
                
                //Check intermediary positions
                //-No intermediary positions for regular king movement
                
                move_check_b(game,&m)
                //Return result from checkCheck
            }

            fn secondary_is_move_allowed(self, game: &Game, m: Move) -> bool{
                //A king should never move more than one step in any direction in one move¨
                //Except for castling, which can be added later.
                
                //Boiler plate
                if !move_check_a(game, &m) {
                    return false
                }
                
                //Unique code for king movement
                if !distance(m.start_pos.x, m.end_pos.x) <= 1 && !distance(m.start_pos.y, m.end_pos.y) <= 1{
                    return false
                }
                
                //Write castling code here
                
                //Check intermediary positions
                //-No intermediary positions for regular king movement
                
                true
            }
            
            fn doMove(mut self, g: &Game, m: Move){
                let killed_piece = g.piece_at_pos(&m.end_pos);
                match killed_piece{
                    //Kill the target, if it exists
                    Some(mut kp) => kp.is_alive = false,
                    None => ()
                }
                g.piece_at_pos(&m.start_pos).unwrap().position = m.end_pos;
                self.piece.moved = true;
            }
        }
        
        fn distance(c1: u8, c2: u8) -> u8{
            c1.max(c2) - c1.min(c2)
        }
        
        impl Game {
            
            pub fn clone(&self) -> Game{
                let mut vec:Vec<Piecedata> = Vec::with_capacity(32);
                for i in 0..self.board.len() {
                    vec.push(self.board[i].clone());
                }

                return Game {
                    state: self.get_game_state(),
                    white_turn: self.white_turn,
                    board: vec
                }
            }
            /// Initialises a new board with pieces.
            pub fn new() -> Game {
                /* initialise board, set active colour to white, ... */
                let game = Game {
                    state: GameState::InProgress,
                    white_turn: true,
                    board:Vec::new()
                };
                
                let realgame = game.createPieces();
                return realgame;
                
            }
            
            /// If the current game state is InProgress and the move is legal, 
            /// move a piece and return the resulting state of the game.
            pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
                None
            }
            
            /// Set the piece type that a peasant becames following a promotion.
            pub fn set_promotion(&mut self, _piece: String) -> () {
                ()
            }
            
            /// Get the current game state.
            pub fn get_game_state(&self) -> GameState {
                self.state
            }
            
            fn createPieces(mut self) -> Game{
                //Generate all pieces at default starting positions
                //This is ugly, but it should work
                self.board.push(Piecedata::new(Position::new(0,0),true,String::from("rook")));
                self.board.push(Piecedata::new(Position::new(1,0),true,String::from("nkight")));
                self.board.push(Piecedata::new(Position::new(2,0),true,String::from("bishop")));
                self.board.push(Piecedata::new(Position::new(3,0),true,String::from("queen")));
                self.board.push(Piecedata::new(Position::new(4,0),true,String::from("king")));
                self.board.push(Piecedata::new(Position::new(5,0),true,String::from("bishop")));
                self.board.push(Piecedata::new(Position::new(6,0),true,String::from("nkight")));
                self.board.push(Piecedata::new(Position::new(7,0),true,String::from("rook")));
                for i in 0..8{
                    self.board.push(Piecedata::new(Position::new(i,1),true,String::from("pawn")));
                }
                
                self.board.push(Piecedata::new(Position::new(0,7),false,String::from("rook")));
                self.board.push(Piecedata::new(Position::new(1,7),false,String::from("nkight")));
                self.board.push(Piecedata::new(Position::new(2,7),false,String::from("bishop")));
                self.board.push(Piecedata::new(Position::new(3,7),false,String::from("queen")));
                self.board.push(Piecedata::new(Position::new(4,7),false,String::from("king")));
                self.board.push(Piecedata::new(Position::new(5,7),false,String::from("bishop")));
                self.board.push(Piecedata::new(Position::new(6,7),false,String::from("nkight")));
                self.board.push(Piecedata::new(Position::new(7,7),false,String::from("rook")));
                for i in 0..8{
                    self.board.push(Piecedata::new(Position::new(i,6),false,String::from("pawn")));
                }
                self
            }
            
            /// If a piece is standing on the given tile, return all possible 
            /// new positions of that piece. Don't forget to the rules for check. 
            /// 
            /// (optional) Don't forget to include en passent and castling.
            pub fn get_possible_moves(self, position: Position) -> Option<Vec<Position>> {
                let mut vec:Vec<Position> = Vec::new();
                match self.piece_at_pos(&position){
                    Some(piece) => {
                        for x in 0..8{
                            for y in 0..8{
                                if Position::new(x,y).to_string() == position.to_string(){
                                    continue;
                                }
                                let variant: &str = &(piece.variant);
                                let temp_move = Move::new(position.clone(), Position::new(x,y));
                                match variant{
                                    "king" => {
                                        if make_king(&piece).unwrap().is_move_allowed(&mut self, temp_move) {
                                            vec.push(Position::new(x,y));
                                        }
                                    }
                                    "pawn" => {
                                        if make_pawn(&piece).unwrap().is_move_allowed(&mut self, temp_move) {
                                            vec.push(Position::new(x,y));   
                                        }
                                    }
                                    _ => ()
                                }
                            }
                        }
                    }
                    None => ()
                }
                match vec.len(){
                    0 => return None,
                    _ => return Some(vec)
                }
            }
            
            pub fn piece_at_pos(&mut self, pos: &Position) -> Option<&mut Piecedata>{
                //Returns the Piecedata of the piece at a given position
                for i in 0..self.board.len(){
                    if !self.board[i].is_alive{
                        continue;
                    }
                    if pos.to_string() == self.board[i].position.to_string() {
                        return Some(&mut self.board[i]);
                    }
                }
                None
            }
            
            pub fn piece_at_pos_bool(&self, pos: &Position) -> bool{
                let board = &self.board;
                let length = board.len();
                for i in 0..length{
                    if !(&board)[i].is_alive{
                        continue;
                    }
                    if pos.to_string() == board[i].position.to_string() {
                        return true
                    }
                }
                false
            }
            
            pub fn piece_at_pos_is_white(&self, pos: &Position) -> bool{
                let board = &self.board;
                let length = board.len();
                for i in 0..length{
                    if !(&board)[i].is_alive{
                        continue;
                    }
                    if pos.to_string() == board[i].position.to_string() {
                        return board[i].is_white;
                    }
                }
                eprintln!("ERR: Piece at pos is white-function can't find piecedata with correct position");
                false
            }
            
            pub fn index_of_piece_in_board(&self, pos: &Position) -> Option<usize>{
                let board = &self.board;
                let length = board.len();
                for i in 0..length{
                    if !(&board)[i].is_alive{
                        continue;
                    }
                    if pos.to_string() == board[i].position.to_string() {
                        return Some(i);
                    }
                }
                None
            }
            
            fn print_game_state(&self){
                //Prints the current game state into the console
                let mut board: Vec<Piecedata> = Vec::new();
                board.clone_from_slice(&(self.board));
                let mut set : HashMap<String, char> = HashMap::new();
                for i in 0..32 {
                    if board[i].is_alive {
                        set.insert(board[i].position.to_string(),(&board[i].variant).chars().next().unwrap());
                    }
                }
                for x in 0..8{
                    for y in 0..8{
                        let key = Position::new(x,y).to_string();
                        if set.contains_key(&key){
                            print!(" {} ", set[&key]);
                        }else{
                            print!(" * ");
                        }
                    }
                }
            }
            
            //TODO: Make this return an option of vec of checking pieces (or positions)
            //In order to be able to check for checkmate
            fn check_for_check(&self, check_white_king: bool) -> bool{
                //true means that the king of the specified color is in check.
                //let mut_ref_game = self;
                let mut temp_game = self.clone();
                let board = &mut temp_game.board;
                self.check_for_check_board(board, check_white_king)
            }
            
            fn check_for_check_board(&self, board: &mut Vec<Piecedata>, check_white_king: bool) -> bool{
                if check_white_king{
                    //Check all black pieces
                    for i in 16..32{
                        let pieced: &Piecedata;
                        {
                            pieced = &board[i];
                        }
                        if !pieced.is_alive{
                            continue;
                        }
                        //Create a move from the attacking piece to the king, which we want to know the check-status of
                        let temp_move: Move = Move::new(
                            pieced.position.clone(),
                            board[4].position.clone());//king pos
                            let variant: &str = &pieced.variant;
                            match variant {//Convert the Piecedata instance into it's struct
                            //Then check if the move is allowed
                            //If it is, the king is in check
                            "king" => {
                                if make_king(&pieced).unwrap().secondary_is_move_allowed(self, temp_move) {
                                    return true    
                                }
                            },
                            "pawn" => {
                                if make_pawn(&pieced).unwrap().secondary_is_move_allowed(self, temp_move) {
                                    return true    
                                }
                            },
                            _ => ()
                        }
                        
                    }
                }
                else{
                    //Check all white pieces
                    for i in 0..16{
                        let pieced = &board[i];
                        if !pieced.is_alive{
                            continue;
                        }
                        //Create a move from the attacking piece to the king, which we want to know the check-status of
                        let temp_move: Move = Move::new(
                            pieced.position.clone(),
                            board[20].position.clone());//king pos
                            let variant: &str = &pieced.variant;
                            match variant {//Convert the Piecedata instance into it's struct
                            //Then check if the move is allowed
                            //If it is, the king is in check
                            "king" => {
                                if make_king(&pieced).unwrap().secondary_is_move_allowed(self, temp_move) {
                                    return true    
                                }
                            }
                            "pawn" => {
                                if make_pawn(&pieced).unwrap().secondary_is_move_allowed(self, temp_move) {
                                    return true    
                                }
                            }
                            _ => ()
                        }
                        
                    }
                }
                //We have checked every piece. If noone can kill the king, the king is not in check.
                false
            }
        }
        
        /// Implement print routine for Game.
        /// 
        /// Output example:
        /// |:----------------------:|
        /// | R  Kn B  K  Q  B  Kn R |
        /// | P  P  P  P  P  P  P  P |
        /// | *  *  *  *  *  *  *  * |
        /// | *  *  *  *  *  *  *  * |
        /// | *  *  *  *  *  *  *  * |
        /// | *  *  *  *  *  *  *  * |
        /// | P  P  P  P  P  P  P  P |
        /// | R  Kn B  K  Q  B  Kn R |
        /// |:----------------------:|
        impl fmt::Debug for Game {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                /* build board representation string */
                
                write!(f, "")
            }
        }
        
        // --------------------------
        // ######### TESTS ##########
        // --------------------------
        
        #[cfg(test)]
        mod tests {
            use super::Game;
            use super::GameState;
            
            // check test framework
            #[test]
            fn it_works() {
                assert_eq!(2 + 2, 4);
            }
            
            // example test
            // check that game state is in progress after initialisation
            #[test]
            fn game_in_progress_after_init() {
                
                let game = Game::new();
                
                assert_eq!(game.get_game_state(), GameState::InProgress);
            }
        }