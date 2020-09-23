use std::fmt;
use std::collections::HashMap;

//pub mod movecheck;
//I can't figure out how to use this properly
//Where to put structs etc.

//Chess attempt #1
//This will be entirely written in 1 file
//If this ever gets close to working order I will probably start over

//Keyword: probably

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
    //state: GameState,
    //why store state when you can just compute it!
    white_turn: bool,
    awaiting_promotion: Option<Position>,
    board: Vec<Piecedata>
}

#[derive(Clone)]
pub struct Piecedata {
    //Struct containing data for pieces
    position: Position,
    is_alive: bool,
    is_white: bool,
    variant: String,
    moved: bool,
    enpassantable: u8
}

impl Piecedata {
    //Constructor for Piecedata
    fn new(position: Position, is_white: bool, variant: String) -> Self{
        Piecedata {position: position, is_alive: true, is_white: is_white, variant: variant, enpassantable: 0, moved: false}
    }
    
}

impl fmt::Debug for Piecedata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        f.debug_struct("Point")
        .field("Variant:", &self.variant)
        .field("Position:", &self.position.to_string())
        .field("Alive:", &self.is_alive)
        .field("Color:",if self.is_white {&"White"}else{&"Black"})
        .field("Moved: ", &self.moved)
        .field("Enpassantable:",if self.enpassantable > 0 {&"Yes"}else{&"No"})
        .finish()
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
    if data.variant == "pawn"{ 
        return Some(Pawn::new(data.position.clone(),data.is_white))
    }else {
        None
    }
}

fn make_rook(data: &Piecedata) -> Option<Rook>{
    if data.variant == "rook"{ 
        return Some(Rook::new(data.position.clone(),data.is_white))
    }else {
        None
    }
}

fn make_queen(data: &Piecedata) -> Option<Queen>{
    if data.variant == "queen"{ 
        return Some(Queen::new(data.position.clone(),data.is_white))
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
    if m.start_pos.x > 7 {
        return false
    }
    if m.start_pos.y > 7{
        return false
    }
    if m.end_pos.x > 7 {
        return false
    }
    if m.end_pos.y > 7{
        return false
    }
    
    //Find the piece to be moved
    let white_turn = game.white_turn;
    //let mut piece_opt:Option<&mut Piecedata>;
    
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
        let mut temp_game = game.clone();
        
        let is_white = temp_game.piece_at_pos(&m.end_pos).unwrap().is_white;
        
        temp_game.piece_at_pos(&m.start_pos).unwrap().position = m.end_pos.clone();
        
        checked = temp_game.check_for_check(is_white);
        
        //Revert (this function is only for checking if the move is valid)
        temp_game.piece_at_pos(&m.end_pos).unwrap().position = m.start_pos.clone();
        
    }
    checked
}



//Is this the closest thing to an abstract class in java or something?
trait Piece {
    fn new(position: Position, is_white: bool) -> Self;
    
    fn is_move_allowed(self, game: &Game, m: Move) -> bool;
    
    fn secondary_is_move_allowed(self, game: &Game, m: Move) -> bool;
    
    fn do_move(self, g: &mut Game, m: Move);
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
    
    /*fn to_string_vec(&self) -> Vec<String>{
        //Returns the move in string form, eg. a2-a3
        let mut vec: Vec<String> = Vec::new();
        vec.push(self.start_pos.to_string());
        vec.push(self.end_pos.to_string());
        vec
    }*/
}

#[derive(Clone)]
pub struct Position {
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
    
    fn are_identical(&self, pos: &Position) -> bool{
        if self.x == pos.x && self.y == pos.y{
            return true
        }
        false
    }
    
    pub fn clone(&self)->Position{
        Position::new(self.x,self.y)
    }
}

struct Pawn {
    piece: Piecedata
}

impl Piece for Pawn {
    fn new(position: Position, is_white: bool) -> Self {
        let piecedata = Piecedata::new(position, is_white, String::from("pawn"));
        Pawn {piece: piecedata}
    }
    
    fn is_move_allowed(self, game: &Game, m: Move) -> bool{
        if !self.secondary_is_move_allowed(game, Move::new(m.start_pos.clone(),m.end_pos.clone())){
            return false
        }
        
        return move_check_b(game,&m)
        //Return result from checkCheck
    }
    fn secondary_is_move_allowed(self, game: &Game, m: Move) -> bool{
        
        //Boiler plate
        if !move_check_a(game, &m) {
            return false
        }
        
        //Unique code for piece movement
        
        if (m.start_pos.y < m.end_pos.y) != game.white_turn{
            return false;//Moving in the wrong direction
        }
        
        //Double-step
        if distance(m.start_pos.y, m.end_pos.y) == 2{
            if self.piece.moved{
                return false;
            }
            if distance(m.start_pos.x, m.end_pos.x) != 0{
                //Askew with double step == nono
                return false;
            }
            let mut temp_game = game.clone();
            match temp_game.piece_at_pos(&Position::new(m.start_pos.x,(m.start_pos.y+m.end_pos.y)/2)){//Position being stepped over
                Some(_) => return false,
                None => ()
            }
            match temp_game.piece_at_pos(&m.end_pos){
                Some(_) => return false,
                None => ()
            }
        }else if distance(m.start_pos.y, m.end_pos.y) == 1{
            if distance(m.start_pos.x, m.end_pos.x) == 1{
                //Attacking
                let mut temp_game = game.clone();
                match temp_game.piece_at_pos(&m.end_pos){
                    None => {
                        //En passant
                        match temp_game.piece_at_pos(&Position::new(m.start_pos.y,m.end_pos.x)){
                            Some(enpassant) => {
                                if enpassant.enpassantable > 0{
                                    return true
                                    //There is a bug in this implementation
                                    //Since the enpassantable pawn does not disappear in move_check_b for check_checking
                                    //It could allow some move which is actually illegal
                                    //This could be fixed by putting the sensing code into move_check_b and killing the pawn
                                }
                            },
                            None => ()
                        }
                        //En passant but no enpassantable piece
                        return false;
                    }
                    Some(_) => ()//Controlls for this movement done elsewhere
                }
            }else if m.end_pos.x != m.start_pos.x{
                return false;
            }else{
                if m.end_pos.y == if game.white_turn {7} else {0}{
                    //promotion. We have a &-reference here, so done elsewhere for now
                }
            }
        }
        return true
        
        //Check intermediary positions
        
        //Everything except placing one's own king in check controlled.
    }
    
    fn do_move(mut self, g: &mut Game, m: Move){
        if g.piece_at_pos(&m.start_pos).unwrap().variant == String::from("pawn"){
            if distance(m.start_pos.y, m.end_pos.y) == 1{
                if distance(m.start_pos.x, m.end_pos.x) == 1{
                    //Attacking
                    match g.piece_at_pos(&m.end_pos){
                        None => {
                            //En passant
                            match g.piece_at_pos(&Position::new(m.start_pos.y,m.end_pos.x)){
                                Some(enpassant) => {
                                    if enpassant.enpassantable > 0{
                                        enpassant.is_alive = false;
                                        g.piece_at_pos(&m.start_pos).unwrap().position = m.end_pos;
                                        self.piece.moved = true;
                                        return
                                    }
                                    
                                    
                                }
                                None => eprintln!("ERR: NO ENPASSANTABLE PIECE IN do_move!")
                            }
                            
                        }
                        Some(_) => ()//Ehhh, not enpassant, is done further down in this function
                    }
                }
            }
        }
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

struct Rook {
    piece: Piecedata
}

impl Piece for Rook {
    fn new(position: Position, is_white: bool)-> Self {
        let piecedata = Piecedata::new(position, is_white,String::from("rook"));
        Rook {piece: piecedata}
    }

    fn is_move_allowed(self, game: &Game, m: Move) -> bool{
        if !self.secondary_is_move_allowed(game, Move::new(m.start_pos.clone(),m.end_pos.clone())){
            return false
        }
        
        return move_check_b(game,&m)
        //Return result from checkCheck
    }
    fn secondary_is_move_allowed(self, game: &Game, m: Move) -> bool{
        
        //Boiler plate
        if !move_check_a(game, &m) {
            return false
        }
        
        //Unique code for piece movement
        let mut clear_positions: Vec<Position> = Vec::new();

        if m.start_pos.x != m.end_pos.x{
            if m.start_pos.y != m.end_pos.y{
                return false;
            }else{
                for i in m.start_pos.x..m.end_pos.x{
                    if i != m.start_pos.x && i != m.end_pos.x{
                        clear_positions.push(Position::new(i,m.start_pos.y));
                    }
                }
            }
        }else{
            //Straight line or same position (should be checked before calling this function)
            for i in m.start_pos.y..m.end_pos.y{
                if i != m.start_pos.y && i != m.end_pos.y{
                    clear_positions.push(Position::new(m.start_pos.x,i));
                }
            }
        }
        //Check intermediary positions
        for clear_pos in clear_positions{
            if !game.piece_at_pos_bool(&clear_pos){
                return false
            }
        }
        
        //Everything except placing one's own king in check controlled.
        true
    }
    fn do_move(mut self, g: &mut Game, m: Move){
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

struct Queen {
    piece: Piecedata
}

impl Piece for Queen {
    fn new(position: Position, is_white: bool)-> Self {
        let piecedata = Piecedata::new(position, is_white,String::from("queen"));
        Queen {piece: piecedata}
    }

    fn is_move_allowed(self, game: &Game, m: Move) -> bool{
        if !self.secondary_is_move_allowed(game, Move::new(m.start_pos.clone(),m.end_pos.clone())){
            return false
        }
        
        return move_check_b(game,&m)
        //Return result from checkCheck
    }
    fn secondary_is_move_allowed(self, game: &Game, m: Move) -> bool{
        
        //Boiler plate
        if !move_check_a(game, &m) {
            return false
        }
        
        //Unique code for piece movement
        let mut clear_positions: Vec<Position> = Vec::new();

        if m.start_pos.y == m.end_pos.y{
            //Horizontal
            for i in m.start_pos.x..m.end_pos.x{
                clear_positions.push(Position::new(i,m.start_pos.y));
            }
        }else if m.start_pos.x == m.end_pos.x{
            //Vertical
            for i in m.start_pos.y..m.end_pos.y{
                clear_positions.push(Position::new(m.start_pos.x,i));
            }
        }else{
            //Diagonal

            //Messy code for generating intermediary positions:
            let right = m.start_pos.x < m.end_pos.x;
            let up = m.start_pos.y < m.end_pos.y;
            if right && up{
                for i in 0..distance(m.start_pos.x,m.end_pos.x){
                    clear_positions.push(Position::new(m.start_pos.x+i,m.start_pos.y+i));
                }
            }
            if right && !up{
                for i in 0..distance(m.start_pos.x,m.end_pos.x){
                    clear_positions.push(Position::new(m.start_pos.x+i,m.start_pos.y-i));
                }
            }
            if !right && up{
                for i in 0..distance(m.start_pos.x,m.end_pos.x){
                    clear_positions.push(Position::new(m.start_pos.x-i,m.start_pos.y+i));
                }
            }
            if !right && !up{
                for i in 0..distance(m.start_pos.x,m.end_pos.x){
                    clear_positions.push(Position::new(m.start_pos.x-i,m.start_pos.y-i));
                }
            }
        }
        //Check intermediary positions
        for clear_pos in clear_positions{
            if !game.piece_at_pos_bool(&clear_pos){
                return false
            }
        }
        
        //Everything except placing one's own king in check controlled.
        true
    }
    fn do_move(mut self, g: &mut Game, m: Move){
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
    
    fn is_move_allowed(self, game: &Game, m: Move) -> bool{
        if !self.secondary_is_move_allowed(game, Move::new(m.start_pos.clone(),m.end_pos.clone())){
            return false
        }
        
        return move_check_b(game,&m)
        //Return result from checkCheck
    }
    fn secondary_is_move_allowed(self, game: &Game, m: Move) -> bool{
        
        //Boiler plate
        if !move_check_a(game, &m) {
            return false
        }
        
        //Unique code for piece movement
        if !distance(m.start_pos.x, m.end_pos.x) <= 1 && !distance(m.start_pos.y, m.end_pos.y) <= 1{
            return false
        }
        
        //Check intermediary positions
        
        //Everything except placing one's own king in check controlled.
        true
    }
    
    fn do_move(mut self, g: &mut Game, m: Move){
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
    
    fn clone(&self) -> Game{
        let mut vec:Vec<Piecedata> = Vec::with_capacity(32);
        for i in 0..self.board.len() {
            vec.push(self.board[i].clone());
        }
        
        return Game {
            white_turn: self.white_turn,
            awaiting_promotion: self.awaiting_promotion.clone(),
            board: vec
        }
    }
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        /* initialise board, set active colour to white, ... */
        let game = Game {
            white_turn: true,
            awaiting_promotion: None,
            board:Vec::new()
        };
        
        let realgame = game.create_pieces();
        return realgame;
        
    }
    
    fn next_turn(&mut self){
        self.white_turn = !self.white_turn;
        for i in 0..32{
            if self.board[i].enpassantable > 0{
                self.board[i].enpassantable -= 1;
            }
        }
    }
    
    /// If the current game state is InProgress and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, from: Position, to: Position) -> Option<GameState> {
        let maybe_vec = (&self).get_possible_moves(from.clone());
        let mut move_allowed:bool = false;
        match maybe_vec{
            None => return None,
            Some(v) => {
                for element in v{
                    if element.are_identical(&to){
                        move_allowed = true;
                        break;
                    }
                }
            }
        }
        if !move_allowed{
            return None;
        }
        let m: Move = Move::new(from.clone(),to);
        match self.piece_at_pos(&from){
            None => return None,//No piece at position, can't make move
            Some(piece) => {
                let cloned_piece = piece.clone();
                let literal_variant: &str = &piece.variant;
                match literal_variant {//Convert the Piecedata instance into it's struct
                    //Then check if the move is allowed
                    //If it is, the king is in check
                    "king" => {
                        //if make_king(&cloned_piece).unwrap().is_move_allowed(&self, Move::new(m.start_pos.clone(),m.end_pos.clone())) {
                        make_king(&cloned_piece).unwrap().do_move(self, m);
                        self.next_turn();
                        return Some(self.get_game_state());
                        //}
                        }
                    "pawn" => {
                        make_pawn(&cloned_piece).unwrap().do_move(self, Move::new(m.start_pos.clone(),m.end_pos.clone()));
                        if m.end_pos.y == if self.white_turn {7} else {0}{
                            self.awaiting_promotion = Some(m.end_pos);
                        }else{
                            self.next_turn();
                        }
                        return Some(self.get_game_state());
                    }
                    "rook" => {
                        make_rook(&cloned_piece).unwrap().do_move(self, Move::new(m.start_pos.clone(),m.end_pos.clone()));
                        self.next_turn();
                        return Some(self.get_game_state());
                    }
                    "queen" => {
                        make_queen(&cloned_piece).unwrap().do_move(self, Move::new(m.start_pos.clone(),m.end_pos.clone()));
                        self.next_turn();
                        return Some(self.get_game_state());
                    }
                    _ => ()
                }
            } 
        }
        
        None
        
    }
    
    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, piece: String) -> () {
        match &self.awaiting_promotion{
            None => return,
            Some(position) => {
                let cloned_position = position.clone();
                match self.piece_at_pos(&cloned_position){
                    None => eprintln!("ERR: Awaiting promotion for piece which is not at designated position"),
                    Some(pawn) => {
                        let promote_to: &str = &piece;
                        match promote_to{
                            "king" => eprintln!("CANT PROMOTE PAWN TO KING"),
                            "pawn" => eprintln!("PAWN CAN'T BE PROMOTED TO ANOTHER PAWN, right?"),
                            /*variant => {
                                Feel free to swap this for following cases
                                Could lead to illegal variants without proper support
                                pawn.variant = String::from(variant);
                                self.next_turn();
                            }*/
                            "rook" => {
                                pawn.variant = String::from("rook");
                                self.next_turn();
                            },
                            "queen" => {
                                pawn.variant = String::from("queen");
                                self.next_turn();
                            },
                            "nkight" => {
                                //Transform the pawn to a nkight. 
                                //If success:
                                self.awaiting_promotion = None;
                                self.next_turn();
                                return
                            }
                            _ => eprintln!("CANT PROMOTE PAWN TO [object Object]")
                        }
                    }
                }
            }
        }
    }
    
    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        //Compute the GameState
        let checked = self.check_for_check(self.white_turn);
        if checked{
            let mut temp_game = self.clone();
            let white_turn = temp_game.white_turn;
            let mut checkmate = true;
            let offset: usize = if white_turn {0} else {16};//Offset index in order to only get the piecedata of one color
            'piece: for i in 0+offset..16+offset{
                for x in 0..8{
                    for y in 0..8{
                        if temp_game.board[i].position.clone().to_string() == Position::new(x,y).to_string(){
                            continue;
                        }
                        //let m: Move = Move::new(temp_game.board[i].position.clone(),Position::new(x,y));
                        temp_game.make_move(temp_game.board[i].position.clone(),Position::new(x,y));
                        if !temp_game.check_for_check(white_turn){
                            checkmate = false;
                            break 'piece;
                        }
                    }
                }
            }
            if checkmate{
                return GameState::GameOver;
            }else{
                return GameState::Check;
            }
        }
        GameState::InProgress
    }
    
    fn create_pieces(mut self) -> Game{
        //Generate all pieces at default starting positions
        //This is ugly, but it should work

        //Todo: check if white king is at index 4, otherwise reverse the vec
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
    pub fn get_possible_moves(&self, position: Position) -> Option<Vec<Position>> {
        match self.awaiting_promotion{
            None => (),
            Some(_) => return None
        }
        let mut vec:Vec<Position> = Vec::new();
        let mut temp_game = self.clone();
        let mut identical_game = temp_game.clone();
        match identical_game.piece_at_pos(&position){
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
                                if make_king(&piece).unwrap().is_move_allowed(&mut temp_game, temp_move) {
                                    vec.push(Position::new(x,y));
                                }
                            }
                            "pawn" => {
                                if make_pawn(&piece).unwrap().is_move_allowed(&mut temp_game, temp_move) {
                                    vec.push(Position::new(x,y));   
                                }
                            }
                            "rook" => {
                                if make_rook(&piece).unwrap().is_move_allowed(&mut temp_game, temp_move) {
                                    vec.push(Position::new(x,y));   
                                }
                            }
                            "queen" => {
                                if make_queen(&piece).unwrap().is_move_allowed(&mut temp_game, temp_move) {
                                    vec.push(Position::new(x,y));   
                                }
                            }
                            _ => ()
                        }
                        break;
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
    
    pub fn print_game_state(&self){
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
            println!();
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
                    }
                    "rook" => {
                        if make_rook(&pieced).unwrap().secondary_is_move_allowed(self, temp_move) {
                            return true    
                        }
                    }
                    "queen" => {
                        if make_queen(&pieced).unwrap().secondary_is_move_allowed(self, temp_move) {
                            return true    
                        }
                    }
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
                    "rook" => {
                        if make_rook(&pieced).unwrap().secondary_is_move_allowed(self, temp_move) {
                            return true    
                        }
                    }
                    "queen" => {
                        if make_queen(&pieced).unwrap().secondary_is_move_allowed(self, temp_move) {
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
        f.debug_struct("Point")
        .field("Board:", &self.board)
        .field("GameState:", &self.get_game_state())
        .finish()
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