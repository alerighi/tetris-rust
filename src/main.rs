use ncurses::*;

const GAME_WIDTH: usize = 10;
const GAME_HEIGHT: usize = 22;
const PIECE_SPAWN_POSITION: Point = Point { y: 0, x: GAME_WIDTH as i32 / 2 - 2 };

const tetris: [[[[u8; 4]; 4]; 4]; 7] = [
	[ /* I */
		[[1,0,0,0],[1,0,0,0],[1,0,0,0],[1,0,0,0]],
		[[1,1,1,1],[0,0,0,0],[0,0,0,0],[0,0,0,0]],
		[[1,0,0,0],[1,0,0,0],[1,0,0,0],[1,0,0,0]],
		[[1,1,1,1],[0,0,0,0],[0,0,0,0],[0,0,0,0]],
	], 
	[ /* O */
		[[1,1,0,0],[1,1,0,0],[0,0,0,0],[0,0,0,0]],
		[[1,1,0,0],[1,1,0,0],[0,0,0,0],[0,0,0,0]],
		[[1,1,0,0],[1,1,0,0],[0,0,0,0],[0,0,0,0]],
		[[1,1,0,0],[1,1,0,0],[0,0,0,0],[0,0,0,0]],
	], 
	[ /* L */
		[[1,1,0,0],[1,0,0,0],[1,0,0,0],[0,0,0,0]],
		[[1,1,1,0],[0,0,1,0],[0,0,0,0],[0,0,0,0]],
		[[0,1,0,0],[0,1,0,0],[1,1,0,0],[0,0,0,0]],
		[[1,0,0,0],[1,1,1,0],[0,0,0,0],[0,0,0,0]],
	], 
	[ /* J */
		[[1,1,0,0],[0,1,0,0],[0,1,0,0],[0,0,0,0]],
		[[0,0,1,0],[1,1,1,0],[0,0,0,0],[0,0,0,0]],
		[[1,0,0,0],[1,0,0,0],[1,1,0,0],[0,0,0,0]],
		[[1,1,1,0],[1,0,0,0],[0,0,0,0],[0,0,0,0]],
	], 
	[ /* T */
		[[0,1,0,0],[1,1,1,0],[0,0,0,0],[0,0,0,0]],
		[[1,0,0,0],[1,1,0,0],[1,0,0,0],[0,0,0,0]],
		[[1,1,1,0],[0,1,0,0],[0,0,0,0],[0,0,0,0]],
		[[0,1,0,0],[1,1,0,0],[0,1,0,0],[0,0,0,0]],
	], 
	[ /* S */
		[[0,1,0,0],[1,1,0,0],[1,0,0,0],[0,0,0,0]],
		[[1,1,0,0],[0,1,1,0],[0,0,0,0],[0,0,0,0]],
		[[0,1,0,0],[1,1,0,0],[1,0,0,0],[0,0,0,0]],
		[[1,1,0,0],[0,1,1,0],[0,0,0,0],[0,0,0,0]],
	], 
	[ /* Z */
		[[1,0,0,0],[1,1,0,0],[0,1,0,0],[0,0,0,0]],
		[[0,1,1,0],[1,1,0,0],[0,0,0,0],[0,0,0,0]],
		[[1,0,0,0],[1,1,0,0],[0,1,0,0],[0,0,0,0]],
		[[0,1,1,0],[1,1,0,0],[0,0,0,0],[0,0,0,0]],
	],
];

#[derive(Clone, Copy, Debug, PartialEq)]
enum PieceShape {
    I = 0, 
    O = 1, 
    L = 2, 
    J = 3, 
    T = 4, 
    S = 5, 
    Z = 6,
}

#[derive(Clone, Copy, Debug)]
enum PieceRotation {
    NORMAL = 0, 
    LEFT = 1, 
    REVERSE = 2,
    RIGHT = 3, 
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    UP, DOWN, LEFT, RIGHT,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn moved(&self, direction: Direction) -> Point {
        use Direction::*;
        match direction {
            DOWN => Point { x: self.x, y: self.y + 1 },
            UP => Point { x: self.x, y: self.y - 1},
            LEFT => Point { x: self.x - 1, y: self.y },
            RIGHT => Point { x: self.x + 1, y: self.y },
        }
    }

    fn translated(&self, point: Point) -> Point {
        Point {
            y: self.y + point.y,
            x: self.x + point.x,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Piece {
    shape: PieceShape,
    rotation: PieceRotation,
    position: Point,
}

impl Piece {
    fn random() -> Piece {
        use PieceShape::*;
        let random_shape = match rand::prelude::random::<u64>() % 7 {
            0 => I,
            1 => O,
            2 => L,
            3 => J,
            4 => T,
            5 => S,
            6 => Z,
            _ => unreachable!(),
        };
        Piece {
            rotation: PieceRotation::NORMAL,
            shape: random_shape,
            position: PIECE_SPAWN_POSITION,
        }
    }

    fn with_rotation(&self, rotation: PieceRotation) -> Piece {
        Piece {
            shape: self.shape,
            position: self.position,
            rotation,
        }
    }

    fn with_position(&self, position: Point) -> Piece {
        Piece {
            shape: self.shape,
            rotation: self.rotation,
            position,
        }
    }

    fn moved(&self, direction: Direction) -> Piece {
        self.with_position(self.position.moved(direction))
    }

    fn rotated_left(&self) -> Piece {
        use PieceRotation::*;
        self.with_rotation(match self.rotation {
            NORMAL => LEFT,
            LEFT => REVERSE,
            REVERSE => RIGHT,
            RIGHT => NORMAL,
        })
    }

    fn rotated_right(&self) ->  Piece {
        use PieceRotation::*;
        self.with_rotation(match self.rotation {
            NORMAL => RIGHT,
            RIGHT => REVERSE,
            REVERSE => LEFT,
            LEFT => NORMAL,
        })
    }

    fn get(&self, y: usize, x: usize) -> bool {
        tetris[self.shape as usize][self.rotation as usize][y][x] != 0
    }

    fn check_limits(&self) -> bool {
        self.position.x >= 0 
            && self.position.y >= 0
            && self.position.x < GAME_WIDTH as i32
            && self.position.y < GAME_HEIGHT as i32
    }

    fn check_collision(&self, state: &GameState) -> bool {
        if !self.check_limits() {
            return false;
        }
        for y in 0..4 {
            for x in 0..4 {
                if self.get(y, x) && (
                    self.position.x as usize + x >= GAME_WIDTH
                    || self.position.y as usize + y >= GAME_HEIGHT
                    || state.is_occupied(self.position.y as usize + y, self.position.x as usize + x)
                ) {
                    return false;
                } 
            }
        }
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum FieldCell {
    Empty, 
    Occupied(PieceShape),
}

struct GameState {
    score: i32,
    level: i32,
    delay: i32,
    lost: bool,
    field: [[FieldCell; GAME_WIDTH]; GAME_HEIGHT],
    current_piece: Piece,
}

impl GameState {
    fn new() -> GameState {
        let mut game = GameState {
            field: [[FieldCell::Empty; GAME_WIDTH]; GAME_HEIGHT],
            score: 0,
            level: 1,
            delay: 0,
            current_piece: Piece::random(),
            lost: false,
        };
        game.timer_reset();
        game
    }

    fn get(&self, y: usize, x: usize) -> FieldCell {
        let p = self.current_piece.position;
        if p.y <= y as i32 && (y as i32) < p.y + 4 && p.x <= x as i32 && (x as i32) < p.x + 4 {
            if self.current_piece.get(y - p.y as usize, x - p.x as usize) {
                return FieldCell::Occupied(self.current_piece.shape);
            }
        }
        self.field[y][x]
    }

    fn is_occupied(&self, y: usize, x: usize) -> bool {
        match self.field[y][x] {
            FieldCell::Empty => false,
            FieldCell::Occupied(_) => true,
        }
    }

    fn piece_bottom(&mut self) {
        for y in 0..4 {
            for x in 0..4 {
                if self.current_piece.get(y, x) {
                    let screen_y = self.current_piece.position.y as usize + y;
                    let screen_x = self.current_piece.position.x as usize + x;
                    self.field[screen_y][screen_x] = FieldCell::Occupied(self.current_piece.shape);
                }
            }
        }

        self.eliminate_lines();
        self.add_new_piece();
    }

    fn move_left(&mut self) {
        let moved = self.current_piece.moved(Direction::LEFT);
        if moved.check_collision(self) {
            self.current_piece = moved;
        }
    }

    fn move_right(&mut self) {
        let moved = self.current_piece.moved(Direction::RIGHT);
        if moved.check_collision(self) {
            self.current_piece = moved;
        }
    }

    fn timer_reset(&mut self) {
        self.delay = 800 * 0.9f32.powi(self.level).round() as i32;

    }
    
    fn step_down(&mut self) -> bool {
        let moved = self.current_piece.moved(Direction::DOWN);
        if moved.check_collision(self) {
            self.current_piece = moved;
            true
        } else {
            false
        }
    }

    fn add_new_piece(&mut self) {
        self.current_piece = Piece::random();
        if !self.current_piece.check_collision(self) {
            self.lost = true;
        }
    }

    fn move_bottom(&mut self) {
        while self.step_down() {}
        self.piece_bottom()
    }

    fn move_down(&mut self) {
        if !self.step_down() {
            self.piece_bottom()
        }
    }

    fn rotate(&mut self) {
        let rotated = self.current_piece.rotated_right();
        if rotated.check_collision(self) {
            self.current_piece = rotated;
        }
    }

    fn clock_tick(&mut self) {
        self.timer_reset();
        self.move_down()
    }

    fn eliminate_lines(&mut self) {
        let mut eliminated: usize = 0;
        'nextline: for y in 0..GAME_HEIGHT {
            for x in 0..GAME_WIDTH {
                if self.field[y][x] == FieldCell::Empty {
                    continue 'nextline;
                }
            }

            eliminated += 1;

            // shift all lines down
            for h in (3..y+1).rev() {
                for x in 0..GAME_WIDTH {
                    self.field[h][x] = self.field[h - 1][x];
                }
            }
        }

        let points_per_line = [1, 40, 100, 300, 1200];

        self.score += points_per_line[eliminated];
        self.level = 1 + self.score / 700;

    }
}

struct GameWindow {
    window: WINDOW,
}

impl GameWindow {
    fn new(y: i32, x: i32) -> GameWindow {
        let height = GAME_HEIGHT as i32 + 2;
        let width = GAME_WIDTH as i32 * 2 + 2;
        let window = newwin(height, width, y, x);
        box_(window, 0, 0);
        refresh();
        wrefresh(window);
        GameWindow {
            window: window,
        }
    }

    fn update(&self, state: &GameState) {
        for y in 0..GAME_HEIGHT {
            for x in 0..GAME_WIDTH {
                let (c, col) = match state.get(y, x) {
                    FieldCell::Empty => (' ' as chtype, 0),
                    FieldCell::Occupied(p) => (' ' as chtype | A_REVERSE(), p as i16 + 1),
                };
                wattron(self.window, COLOR_PAIR(col));
                mvwaddch(self.window, y as i32 + 1, x as i32 * 2 + 1, c);
                mvwaddch(self.window, y as i32 + 1, x as i32 * 2 + 2, c);
                wattroff(self.window, COLOR_PAIR(col));
            }
        }
        wrefresh(self.window);
    }
}

struct ScoreWindow {
    window: WINDOW,
}

impl ScoreWindow {
    fn new(y: i32, x: i32) -> ScoreWindow {
        let height = 10;
        let width = 20;
        let window = newwin(height, width, y, x);
        box_(window, 0, 0);
        refresh();
        wrefresh(window);
        ScoreWindow {
            window: window,
        }
    }

    fn update(&self, state: &GameState) {
        mvwprintw(self.window, 1, 1, &format!("level: {}", state.level)[..]);
        mvwprintw(self.window, 2, 1, &format!("score: {}", state.score)[..]);
        mvwprintw(self.window, 3, 1, &format!("delay: {}", state.delay)[..]);
        wrefresh(self.window);
    }
}

fn initialize_curses() {
    initscr();
	cbreak();              /* unbuffered input */
	keypad(stdscr(), true);  /* for special keys */
	noecho();              /* do not echo character on screen */
	curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);       /* do not show cursor */
	timeout(50);		   /* wait 50ms for input */
    use_default_colors();
    start_color();

	init_pair(1, COLOR_CYAN, -1);
	init_pair(2, COLOR_YELLOW, -1);
	init_pair(3, 203, -1);
	init_pair(4, COLOR_BLUE, -1);
	init_pair(5, COLOR_MAGENTA, -1);
	init_pair(6, COLOR_GREEN, -1);
	init_pair(7, COLOR_RED, -1);
}

#[derive(Clone, Copy, PartialEq)]
enum InputCharacter {
    ASCII(char),
    Control(i32),
}

fn better_getch() -> InputCharacter {
    let ch = getch();
    if ch < 127 {
        InputCharacter::ASCII(ch as u8 as char)
    } else {
        InputCharacter::Control(ch)
    }
}

fn quit() {
    endwin();
    std::process::exit(0);
}

fn game_lost(score: i32) {
    endwin();
    println!("Game finished, your score: {}", score);
    std::process::exit(0);
}

fn pause() {
    while better_getch() != InputCharacter::ASCII('p') {}
}

fn main() {
    initialize_curses();
    let game_win = GameWindow::new(1, 1);
    let score_win = ScoreWindow::new(1, 25);
    let mut state = GameState::new();

    loop {
        state.delay -= 50;
        if state.delay == 0 {
            state.clock_tick();
        } else {
            use InputCharacter::*;
            match better_getch() {
                Control(KEY_LEFT) => state.move_left(),
                Control(KEY_RIGHT) => state.move_right(),
                Control(KEY_DOWN) => state.move_down(),
                Control(KEY_UP) => state.rotate(),
                ASCII(' ') => state.move_bottom(),
                ASCII('q') => quit(),
                ASCII('p') => pause(),
                _ => {},
            }
        }
        
        game_win.update(&state);
        score_win.update(&state);

        if state.lost {
            game_lost(state.score);
        }

    }
}
