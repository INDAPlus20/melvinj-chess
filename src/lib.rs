use std::fmt;

/*
Important notes: Knight --> nKight for easier debug printing
You can just get the first char of their type
#lifehack
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

/* IMPORTANT:
* - Document well!
* - Write well structured and [somewhat] clean code!
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
        /* Piecedata debugging information */
        f.debug_struct("Piecedata")
        .field("Variant:", &self.variant)
        .field("Position:", &self.position.to_string())
        .field("Alive:", &self.is_alive)
        .field("Color:",if self.is_white {&"White"}else{&"Black"})
        .field("Moved: ", &self.moved)
        .field("Enpassantable:",if self.enpassantable > 0 {&"Yes"}else{&"No"})
        .finish()
    }
}

/*
*The following functions make_[piecetype] are used when checking movement
*Each piece has unique functions but since they all are just stored as Piecedata in game.board
*They need to be converted
*I could also write methods for movement not linked to structs and simply
*Not have Piece, King or similar structs. Maybe next time
*They return None only when they are given bad data, of the incorrect type
*/
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

fn make_bishop(data: &Piecedata) -> Option<Bishop>{
    if data.variant == "bishop"{ 
        return Some(Bishop::new(data.position.clone(),data.is_white))
    }else {
        None
    }
}

fn make_nkight(data: &Piecedata) -> Option<Nkight>{
    if data.variant == "nkight"{ 
        return Some(Nkight::new(data.position.clone(),data.is_white))
    }else {
        None
    }
}

fn horizontal_positions(m: Move) -> Vec<Position>{
    let mut clear_positions: Vec<Position> = Vec::new();
    if m.start_pos.y == m.end_pos.y{
        //Horizontal
        if m.start_pos.x+1 > m.end_pos.x{
            for i in m.end_pos.x..m.start_pos.x{
                clear_positions.push(Position::new(i,m.start_pos.y));
            }
        }else{
            for i in m.start_pos.x+1..m.end_pos.x{
                clear_positions.push(Position::new(i,m.start_pos.y));
            }
        }
        
    }else if m.start_pos.x == m.end_pos.x{
        //Vertical
        if m.start_pos.y > m.end_pos.y{
            for i in m.end_pos.y+1..m.start_pos.y{
                clear_positions.push(Position::new(m.start_pos.x,i));
            }
        }else{
            for i in m.start_pos.y+1..m.end_pos.y{
                clear_positions.push(Position::new(m.start_pos.x,i));
            }
        }
    }
    return clear_positions
}

//Converts a String to a position. 
//"a1" will become Position::new(0,0)
fn string_to_pos(string: String)->Position{
    if string.len() != 2{
        panic!()//Wrong string length
    }else{
        let pos_x: u8;
        let pos_y: u8;
        let mut chars = string.chars();
        match chars.next().unwrap(){
            'a' => pos_x = 0,
            'b' => pos_x = 1,
            'c' => pos_x = 2,
            'd' => pos_x = 3,
            'e' => pos_x = 4,
            'f' => pos_x = 5,
            'g' => pos_x = 6,
            'h' => pos_x = 7,
            _ => panic!()
        }
        match chars.next().unwrap(){
            '1' => pos_y = 0,
            '2' => pos_y = 1,
            '3' => pos_y = 2,
            '4' => pos_y = 3,
            '5' => pos_y = 4,
            '6' => pos_y = 5,
            '7' => pos_y = 6,
            '8' => pos_y = 7,
            _ => panic!()
        }
        return Position::new(pos_x,pos_y);
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
    
    //White turn?
    let white_turn = game.white_turn;
    if white_turn != game.piece_at_pos_is_white(&m.start_pos){
        //Moving enemy piece
        return false;
    }
    if game.piece_at_pos_bool(&m.end_pos){
        if white_turn == game.piece_at_pos_is_white(&m.end_pos){
            //Piece at end pos of same color as attacker
            return false;
        }
    }
    
    true
}

fn move_check_b(game: &Game, n: &Move) -> bool{
    //Attempt the move and see if the player's king is checked
    
    let m:Move = Move::new(n.start_pos.clone(),n.end_pos.clone());
    //Try the move
    let target_exists = game.piece_at_pos_bool(&m.end_pos);
    let checked: bool;
    let mut temp_game: Game = game.clone();
    
    if target_exists {
        //There is a piece at the target
        let attacker_board_index = &temp_game.index_of_piece_in_board(&m.start_pos);
        let target_board_index = &temp_game.index_of_piece_in_board(&m.end_pos);
        
        //BIG nasty block for checking killing the target piece and moving the attacker
        //Could be accomplished with indexes instead, but this works
        let board = &mut temp_game.board;
        if attacker_board_index > target_board_index{
            let (target_component, attacker_component) = board.split_at_mut(attacker_board_index.unwrap()+1);
            for maybe_target in target_component{
                if maybe_target.position.to_string() == m.end_pos.to_string(){//Target found
                    for maybe_attacker in attacker_component{
                        if maybe_attacker.position.to_string() == m.start_pos.to_string(){
                            
                            //Both target and attacker have been found
                            maybe_target.is_alive = false;//Kill target
                            maybe_attacker.position = m.end_pos.clone();
                            break;
                            
                        }
                    }
                    
                    break;
                }
            }
        }else {
            let (attacker_component, target_component) = board.split_at_mut(target_board_index.unwrap()+1);
            for maybe_target in target_component{
                if maybe_target.position.to_string() == m.end_pos.to_string(){//Target found
                    for maybe_attacker in attacker_component{
                        if maybe_attacker.position.to_string() == m.start_pos.to_string(){
                            
                            //Both target and attacker found
                            maybe_target.is_alive = false;
                            maybe_attacker.position = m.end_pos.clone();
                            break;
                        }
                    }
                    
                    break;
                }
            }
        }
        checked = temp_game.check_for_check();//Check check status
    }else{
        //No target piece to kill
        let mut temp_game = game.clone();
        temp_game.piece_at_pos(&m.start_pos)
        .unwrap()
        .position = m.end_pos.clone();
        
        checked = temp_game.check_for_check();
        
        //Revert (this function is only for checking if the move is valid)
        temp_game.piece_at_pos(&m.end_pos).unwrap().position = m.start_pos.clone();
        
    }
    !checked//This exclamation point is very important
}



//Is this the closest thing to an abstract class in java or something?
trait Piece {
    fn new(position: Position, is_white: bool) -> Self;
    
    fn is_move_allowed(self, game: &Game, m: Move) -> bool;
    
    fn secondary_is_move_allowed(self, game: &Game, m: Move) -> bool;
    
    fn do_move(self, g: &mut Game, m: Move);
}

//Struct of two positions
struct Move {
    start_pos: Position,
    end_pos: Position
}

impl Move {
    fn new(p1: Position, p2: Position) -> Self{
        Move {start_pos: p1, end_pos: p2}
    }
}

//Struct of two coordinates
#[derive(Clone)]
pub struct Position {
    x: u8,
    y: u8
}

impl Position {
    fn new(x: u8, y: u8)->Self{
        Position{x:x,y:y}
    }
    //(0,0) -> "a1"
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
    
    fn clone(&self)->Position{
        Position::new(self.x,self.y)
    }
}

//All the Type-structs simply contain a single Piecedata
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
                        match temp_game.piece_at_pos(&Position::new(m.end_pos.x,m.start_pos.y)){
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
        }else{
            return false;
        }
        return true
        
        //Check intermediary positions
        
        //Everything except placing one's own king in check controlled.
    }
    
    fn do_move(mut self, g: &mut Game, m: Move){
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
        }else if distance(m.start_pos.y,m.end_pos.y) == 2{
            g.piece_at_pos(&m.start_pos).unwrap().enpassantable = 2;
        }
        let killed_piece = g.piece_at_pos(&m.end_pos);
        match killed_piece{
            //Kill the target, if it exists
            Some(mut kp) => kp.is_alive = false,
            None => ()
        }
        g.piece_at_pos(&m.start_pos).unwrap().moved = true;
        g.piece_at_pos(&m.start_pos).unwrap().position = m.end_pos;
        
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
        let clear_positions = horizontal_positions(m);
        
        //Check intermediary positions
        for clear_pos in clear_positions{
            if game.piece_at_pos_bool(&clear_pos){
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

struct Bishop {
    piece: Piecedata
}

impl Piece for Bishop {
    fn new(position: Position, is_white: bool)-> Self {
        let piecedata = Piecedata::new(position, is_white,String::from("bishop"));
        Bishop {piece: piecedata}
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
        
        if distance(m.start_pos.x,m.end_pos.x) == distance(m.start_pos.y, m.end_pos.y){
            //Messy code for generating intermediary positions:
            let right = m.start_pos.x < m.end_pos.x;
            let up = m.start_pos.y < m.end_pos.y;
            if right && up{
                for i in 1..distance(m.start_pos.x,m.end_pos.x){
                    clear_positions.push(Position::new(m.start_pos.x+i,m.start_pos.y+i));
                }
            }
            if right && !up{
                for i in 1..distance(m.start_pos.x,m.end_pos.x){
                    clear_positions.push(Position::new(m.start_pos.x+i,m.start_pos.y-i));
                }
            }
            if !right && up{
                for i in 1..distance(m.start_pos.x,m.end_pos.x){
                    clear_positions.push(Position::new(m.start_pos.x-i,m.start_pos.y+i));
                }
            }
            if !right && !up{
                for i in 1..distance(m.start_pos.x,m.end_pos.x){
                    clear_positions.push(Position::new(m.start_pos.x-i,m.start_pos.y-i));
                }
            }
        }else{
            return false;
        }
        //Check intermediary positions
        for clear_pos in clear_positions{
            if game.piece_at_pos_bool(&clear_pos){
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

struct Nkight {
    piece: Piecedata
}

impl Piece for Nkight {
    fn new(position: Position, is_white: bool)-> Self {
        let piecedata = Piecedata::new(position, is_white,String::from("nkight"));
        Nkight {piece: piecedata}
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
        //let mut clear_positions: Vec<Position> = Vec::new();
        
        if distance(m.start_pos.x,m.end_pos.x) == 1 && distance(m.start_pos.y, m.end_pos.y) == 2{
        }else if distance(m.start_pos.x,m.end_pos.x) == 2 && distance(m.start_pos.y, m.end_pos.y) == 1{
            
        }else{
            return false;
        }
        //Check intermediary positions
        //Nkights do not check intermediary positions
        
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
        
        if m.start_pos.y == m.end_pos.y || m.start_pos.x == m.end_pos.x{
            clear_positions = horizontal_positions(m);
        }else if distance(m.start_pos.x,m.end_pos.x) == distance(m.start_pos.y, m.end_pos.y){
            //Diagonal
            
            //Messy code for generating intermediary positions:
            let right = m.start_pos.x < m.end_pos.x;
            let up = m.start_pos.y < m.end_pos.y;
            if right && up{
                for i in 1..distance(m.start_pos.x,m.end_pos.x){
                    clear_positions.push(Position::new(m.start_pos.x+i,m.start_pos.y+i));
                }
            }
            if right && !up{
                for i in 1..distance(m.start_pos.x,m.end_pos.x){
                    clear_positions.push(Position::new(m.start_pos.x+i,m.start_pos.y-i));
                }
            }
            if !right && up{
                for i in 1..distance(m.start_pos.x,m.end_pos.x){
                    clear_positions.push(Position::new(m.start_pos.x-i,m.start_pos.y+i));
                }
            }
            if !right && !up{
                for i in 1..distance(m.start_pos.x,m.end_pos.x){
                    clear_positions.push(Position::new(m.start_pos.x-i,m.start_pos.y-i));
                }
            }
        }else{
            return false
        }
        //Check intermediary positions
        for clear_pos in clear_positions{
            if game.piece_at_pos_bool(&clear_pos){
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
        if distance(m.start_pos.x, m.end_pos.x) > 1 || distance(m.start_pos.y, m.end_pos.y) > 1{
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
            Some(_) => {
                g.piece_at_pos(&m.end_pos).unwrap().is_alive = false;
                if m.end_pos.to_string() == String::from("f7"){
                    //println!("Nasty q: {:?}", g.piece_at_pos(&m.end_pos).unwrap());
                }    
            },
            None => ()
        }
        g.piece_at_pos(&m.start_pos).unwrap().position = m.end_pos;
        self.piece.moved = true;
    }
}

//Returns the absolute distance between two numbers
//2, 6 -> 4 and 6,2 -> 4
fn distance(c1: u8, c2: u8) -> u8{
    c1.max(c2) - c1.min(c2)
}


//Main game code

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
        //A player completed their turn
        self.white_turn = !self.white_turn;
        
        //Since enpassant is only valid the turn directly following the double-step
        for i in 0..32{
            if self.board[i].enpassantable > 0{
                self.board[i].enpassantable -= 1;
            }
        }
    }
    
    /// If the current game state is InProgress and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, from_str: String, to_str: String) -> Option<GameState> {
        if self.make_move_private(from_str,to_str){
            return Some(self.get_game_state());
        }else{
            return None
        }
    }
    fn make_move_private(&mut self, from_str: String, to_str: String) -> bool {
        let from = string_to_pos(from_str);
        let to = string_to_pos(to_str);
        
        //Generate all possible moves from the starting position
        let maybe_vec = (&self).get_possible_moves(from.clone().to_string());
        let mut move_allowed:bool = false;
        match maybe_vec{
            None => return false,
            Some(v) => {
                for element in v{
                    if element == to.to_string(){
                        //The desired end position is possible
                        move_allowed = true;
                        break;
                    }
                }
            }
        }
        if !move_allowed{
            //get_possible_moves does not think the move is possible, return
            return false;
        }
        
        //Do the move
        
        let m: Move = Move::new(from.clone(),to);
        match self.piece_at_pos(&from){
            None => return false,//No piece at position, can't make move. Should have returned in previous step
            Some(piece) => {
                let cloned_piece = piece.clone();
                let literal_variant: &str = &piece.variant;
                match literal_variant {
                    
                    //Convert the Piecedata instance into it's struct
                    //Then check if the move is allowed
                    
                    "king" => {
                        make_king(&cloned_piece).unwrap().do_move(self, m);
                        self.next_turn();
                        return true;
                    }
                    "pawn" => {
                        make_pawn(&cloned_piece).unwrap().do_move(self, Move::new(m.start_pos.clone(),m.end_pos.clone()));
                        if m.end_pos.y == if self.white_turn {7} else {0}{
                            //Promotion, the game awaits what piece to promote the pawn to
                            self.awaiting_promotion = Some(m.end_pos);
                        }else{
                            self.next_turn();
                        }
                        return true;
                    }
                    "rook" => {
                        make_rook(&cloned_piece).unwrap().do_move(self, Move::new(m.start_pos.clone(),m.end_pos.clone()));
                        self.next_turn();
                        return true;
                    }
                    "queen" => {
                        make_queen(&cloned_piece).unwrap().do_move(self, Move::new(m.start_pos.clone(),m.end_pos.clone()));
                        self.next_turn();
                        return true;
                    }
                    "bishop" => {
                        make_bishop(&cloned_piece).unwrap().do_move(self, Move::new(m.start_pos.clone(),m.end_pos.clone()));
                        self.next_turn();
                        return true;
                    }
                    "nkight" => {
                        make_nkight(&cloned_piece).unwrap().do_move(self, Move::new(m.start_pos.clone(),m.end_pos.clone()));
                        self.next_turn();
                        return true;
                    }
                    _ => ()
                }
            } 
        }
        
        false
        
    }
    
    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, piece: String) -> () {
        match &self.awaiting_promotion{
            None => return,//No piece awaiting promotion
            Some(position) => {
                let cloned_position = position.clone();
                match self.piece_at_pos(&cloned_position){
                    None => eprintln!("ERR: Awaiting promotion for piece which is not at designated position"),
                    Some(pawn) => {
                        //Pawn found
                        let promote_to: &str = &piece;
                        match promote_to{
                            "king" => eprintln!("CANT PROMOTE PAWN TO KING"),//Would be rather worthless since check-checking is only done on the original king
                            "pawn" => eprintln!("PAWN CAN'T BE PROMOTED TO ANOTHER PAWN, right?"),
                            /*variant => {
                                Feel free to swap this for following cases
                                Could lead to illegal variants without proper support
                                pawn.variant = String::from(variant);
                                self.next_turn();
                            }*/
                            "rook" => {
                                pawn.variant = String::from("rook");
                                self.awaiting_promotion = None;
                                self.next_turn();
                            },
                            "queen" => {
                                pawn.variant = String::from("queen");
                                self.awaiting_promotion = None;
                                self.next_turn();
                            },
                            "nkight" => {
                                pawn.variant = String::from("queen");
                                self.awaiting_promotion = None;
                                self.next_turn();
                                return
                            }
                            "bishop" => {
                                pawn.variant = String::from("bishop");
                                self.awaiting_promotion = None;
                                self.next_turn();
                                return
                            }
                            _ => eprintln!("CANT PROMOTE PAWN TO [object Object]")//Unknown type
                        }
                    }
                }
            }
        }
    }
    
    /// Get the current game state. Returns GameState::WhatEver
    pub fn get_game_state(&self) -> GameState {
        
        //Compute the GameState
        let checked = self.check_for_check();
        //println!("Get game state for {} and checked {}",self.white_turn, checked);
        if checked{
            let mut temp_game = self.clone();
            let white_turn = temp_game.white_turn;
            let mut checkmate = true;
            let offset: usize = if white_turn {0} else {16};//Offset index in order to only get the piecedata of one color
            'piece: for i in 0+offset..16+offset{//For every friendly piece
                for x in 0..8{
                    for y in 0..8{//For every x, y in [0,7]
                        if temp_game.board[i].position.clone().to_string() == Position::new(x,y).to_string(){
                            continue;//Can't move to same position
                        }
                        let start = temp_game.board[i].position.clone().to_string();
                        let end = Position::new(x,y).to_string();
                        //Attempt every possible move by the player and see if the king still is in check
                        
                        match temp_game.make_move(start.to_string(), end.to_string()){
                            None => continue,//Move not allowed
                            Some(_) => ()
                        }
                        
                        //Turn now reverted to original attacker
                        
                        temp_game.white_turn = !temp_game.white_turn;

                        if !temp_game.check_for_check(){
                            //Some move allows the king to survive, break the outermost loop
                            println!("{} {} Not checkmate, {:?}, can do move {}-{}",white_turn, i,temp_game.board[i],start,end);
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
        self.board.push(Piecedata::new(Position::new(1,0),true,String::from("nkight")));//Intentional typo
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
    /// (optional) Don't forget to include en passant and [redacted].
    pub fn get_possible_moves(&self, string_position: String) -> Option<Vec<String>> {
        let position = string_to_pos(string_position);
        match self.awaiting_promotion{
            None => (),
            Some(_) => return None
        }
        let mut vec:Vec<String> = Vec::new();
        let mut temp_game = self.clone();
        let mut identical_game = temp_game.clone();
        match identical_game.piece_at_pos(&position){
            Some(piece) => {
                for x in 0..8{
                    for y in 0..8{
                        //Attempt to move to every position on the board...
                        if Position::new(x,y).to_string() == position.to_string(){
                            //Except the starting position
                            continue;
                        }
                        
                        //Create a struct-instance in order to check move
                        let variant: &str = &(piece.variant);
                        let temp_move = Move::new(position.clone(), Position::new(x,y));
                        match variant{
                            "king" => {
                                if make_king(&piece).unwrap().is_move_allowed(&mut temp_game, temp_move) {
                                    vec.push(Position::new(x,y).to_string());
                                }
                            }
                            "pawn" => {
                                if make_pawn(&piece).unwrap().is_move_allowed(&mut temp_game, temp_move) {
                                    vec.push(Position::new(x,y).to_string());   
                                }
                            }
                            "rook" => {
                                if make_rook(&piece).unwrap().is_move_allowed(&mut temp_game, temp_move) {
                                    vec.push(Position::new(x,y).to_string());   
                                }
                            }
                            "queen" => {
                                if make_queen(&piece).unwrap().is_move_allowed(&mut temp_game, temp_move) {
                                    vec.push(Position::new(x,y).to_string());   
                                }
                            }
                            "bishop" => {
                                if make_bishop(&piece).unwrap().is_move_allowed(&mut temp_game, temp_move) {
                                    vec.push(Position::new(x,y).to_string());   
                                }
                            }
                            "nkight" => {
                                if make_nkight(&piece).unwrap().is_move_allowed(&mut temp_game, temp_move) {
                                    vec.push(Position::new(x,y).to_string());   
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
            0 => return None, //No possible moves
            _ => return Some(vec) //Some possible moves
        }
    }
    
    pub fn piece_at_pos(&mut self, pos: &Position) -> Option<&mut Piecedata>{
        //Returns a mutable Piecedata of the piece at a given position
        //If there is no Piecedata at the position, returns None
        for i in 0..self.board.len(){
            if !self.board[i].is_alive{
                //Piece dead
                continue;
            }
            if pos.to_string() == self.board[i].position.to_string() {
                //Found the Piecedata
                return Some(&mut self.board[i]);
            }
        }
        None
    }
    
    pub fn piece_at_pos_bool(&self, pos: &Position) -> bool{
        //Returns true if there is a piece at the position
        let board = &self.board;
        let length = board.len();
        for i in 0..length{
            if !(&board)[i].is_alive{
                //Piece dead, ignore
                continue;
            }
            if pos.to_string() == board[i].position.to_string() {
                //There is a piece and it is alive, return true
                return true
            }
        }
        false
    }
    
    pub fn piece_at_pos_is_white(&self, pos: &Position) -> bool{
        //Returns true if the piece at the position is white
        //Panics if there is no piece there
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
        panic!()
    }
    
    pub fn index_of_piece_in_board(&self, pos: &Position) -> Option<usize>{
        //Gets the index in board of the piece at the specified position. 
        //Returns none if there is no Piece at the given position
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
        for x in 0..8{
            'y: for y in 0..8{
                //println!("i");
                for piece in &self.board{
                    //println!("1: {}, 2: {}",piece.position.to_string(),Position::new(x,y).to_string());
                    if piece.position.to_string() == Position::new(y,7-x).to_string(){
                        //print!("a");
                        print!(" {} ", piece.variant.chars().next().unwrap());
                        continue 'y;
                    }
                }
                print!(" * ");
            }
            print!("| {}",7-x+1);
            
            println!();
        }
        println!("------------------------");
        println!(" a  b  c  d  e  f  g  h");
    }
    
    //TODO: Make this return an option of vec of checking pieces (or positions)
    //In order to be able to check for checkmate
    fn check_for_check(&self) -> bool{
        //true means that the king of the specified color is in check.
        
        let mut temp_game = self.clone();
        //temp_game.white_turn = !temp_game.white_turn;
        let board = temp_game.board.clone();
        let offset = if temp_game.white_turn {16} else {0};
        let kingpos = if temp_game.white_turn {4} else {20};
        
        temp_game.white_turn = !temp_game.white_turn;

        for i in 0+offset..16+offset{
            let pieced: &Piecedata = &board[i].clone();
            if !pieced.is_alive{
                continue;
            }
            //Create a move from the attacking piece to the king, which we want to know the check-status of
            let temp_move: Move = Move::new(
                pieced.position.clone(),//Piece current pos
                board[kingpos].position.clone());//king pos
                let variant: &str = &pieced.variant;
                match variant {
                    //Convert the Piecedata instance into it's struct
                    //Then check if the move is allowed
                    //If it is, the king is in check
                    "king" => {
                        if make_king(&pieced).unwrap().secondary_is_move_allowed(&temp_game, temp_move) {
                            //println!("{:?} can check",pieced);
                            return true    
                        }
                    },
                    "pawn" => {
                        if make_pawn(&pieced).unwrap().secondary_is_move_allowed(&temp_game, temp_move) {
                            //println!("{:?} can check",pieced);
                            return true    
                        }
                    }
                    "rook" => {
                        if make_rook(&pieced).unwrap().secondary_is_move_allowed(&temp_game, temp_move) {
                            //println!("{:?} can check",pieced);
                            return true    
                        }
                    }
                    "queen" => {
                        if make_queen(&pieced).unwrap().secondary_is_move_allowed(&temp_game, temp_move) {
                            //println!("{:?} can check",pieced);
                            return true    
                        }
                    }
                    "bishop" => {
                        if make_bishop(&pieced).unwrap().secondary_is_move_allowed(&temp_game, temp_move) {
                            //println!("{:?} can check",pieced);
                            return true    
                        }
                    }
                    "nkight" => {
                        if make_nkight(&pieced).unwrap().secondary_is_move_allowed(&temp_game, temp_move) {
                            //println!("{:?} can check",pieced);
                            return true    
                        }
                    }
                    _ => ()
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
        
        #[test]
        fn print_state() {//Does not assert, simply for manual control of the printing-function
            let mut game = Game::new();
            game.make_move(String::from("a2"), String::from("a4"));//double-step
            game.print_game_state();
        }
        
        #[test]
        fn pawn_movement() {
            //Test all pawn movement except promotion
            let mut game = Game::new();
            assert_eq!(game.get_possible_moves(String::from("a2")).unwrap().contains(&String::from("a4")),true);
            game.make_move(String::from("a2"), String::from("a4"));//double-step
            assert_eq!(game.get_possible_moves(String::from("h7")).unwrap().contains(&String::from("h6")),true);
            game.make_move(String::from("h7"), String::from("h6"));//Regular 1-step(black)
            assert_eq!(game.get_possible_moves(String::from("a4")).unwrap().contains(&String::from("a5")),true);
            game.make_move(String::from("a4"), String::from("a5"));//Regular 1-step(white)
            assert_eq!(game.get_possible_moves(String::from("b7")).unwrap().contains(&String::from("b5")),true);
            game.make_move(String::from("b7"), String::from("b5"));//Black double-step
            assert_eq!(game.get_possible_moves(String::from("a5")).unwrap().contains(&String::from("b6")),true);
            game.make_move(String::from("a5"), String::from("b6"));//En passant
            assert_eq!(game.white_turn,false);
        }
        
        #[test]
        fn game_in_progress_after_init() {
            //The game state should be InProgress just after creation
            let game = Game::new();
            assert_eq!(game.get_game_state(), GameState::InProgress);
        }
        #[test]
        fn kings_at_correct_index() {
            //The white king should be at index 4.
            let game = Game::new();
            assert_eq!(game.board[4].variant, "king");
            assert_eq!(game.board[4].is_white, true);
            assert_eq!(game.board[20].variant, "king");
            assert_eq!(game.board[20].is_white, false);
        }
        
        #[test]
        fn skolmatt() {//Can't find proper translation, 'school mat' is terrible
        let mut game = Game::new();
        game.make_move(String::from("e2"), String::from("e3"));
        game.make_move(String::from("a7"), String::from("a5"));
        game.make_move(String::from("f1"), String::from("c4"));
        game.make_move(String::from("a5"), String::from("a4"));
        game.make_move(String::from("d1"), String::from("h5"));
        game.make_move(String::from("a8"), String::from("a7"));
        game.make_move(String::from("a5"), String::from("b6"));
        game.make_move(String::from("h5"), String::from("f7"));
        game.print_game_state();
        assert_eq!(game.get_game_state(), GameState::GameOver);
    }

    #[test]
    fn promotion() {
        let mut game = Game::new();
        game.make_move(String::from("b2"), String::from("b4"));
        game.make_move(String::from("a7"), String::from("a6"));
        game.make_move(String::from("b4"), String::from("b5"));
        game.make_move(String::from("a6"), String::from("a5"));
        game.make_move(String::from("b5"), String::from("b6"));
        game.make_move(String::from("a5"), String::from("a4"));
        game.make_move(String::from("b6"), String::from("c7"));
        game.make_move(String::from("a4"), String::from("a3"));
        game.make_move(String::from("c7"), String::from("b8"));
        game.print_game_state();
        assert_eq!(game.board[9].variant, String::from("pawn"));
        game.set_promotion(String::from("queen"));
        game.print_game_state();
        assert_eq!(game.board[9].variant, String::from("queen"));
    }
}