use ncurses::*;

use crate::game::{GAME_HEIGHT, GAME_WIDTH, FieldCell, GameState};

const BLOCK: chtype = ' ' as chtype | A_REVERSE();

mod input {
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub enum Character {
        ASCII(char),
        Control(i32),
    }

    pub fn read() -> Character {
        let ch = super::getch();
        if ch < 127 {
            Character::ASCII(ch as u8 as char)
        } else {
            Character::Control(ch)
        }
    }

    pub fn read_yes_no() -> bool {
        loop {
            match read() {
                Character::ASCII('y') => return true,
                Character::ASCII('n') => return false,
                _ => {},
            }
        }
    }
}

mod controls {
    use super::input::Character;
    use super::input::Character::*;
    use ncurses;
    pub const PAUSE: Character = ASCII('p');
    pub const QUIT: Character = ASCII('q');
    pub const BOTTOM: Character = ASCII(' ');
    pub const LEFT: Character = Control(ncurses::KEY_LEFT);
    pub const RIGHT: Character = Control(ncurses::KEY_RIGHT);
    pub const ROTATE: Character = Control(ncurses::KEY_UP);
    pub const DOWN: Character = Control(ncurses::KEY_DOWN);
}

pub struct Ui {
    game_window: WINDOW,
    score_window: WINDOW,
    state: GameState,
}

impl Ui {
    pub fn new() -> Ui {
        Ui::initialize_cursess();
        Ui::initialize_colors();
        Ui::print_title();
        Ui {
            game_window: Ui::create_game_window(),
            score_window: Ui::create_score_window(),
            state: GameState::new(),
        }
    }

    pub fn game_loop(&mut self) {
        loop {
            self.state.clock_tick();
            self.handle_input();
            self.update();
            if self.state.is_lost() {
                self.prompt_new_game();
            }
        }
    }

    fn handle_input(&mut self) {
        use controls::*;
        match input::read() {
            LEFT => self.state.move_left(),
            RIGHT => self.state.move_right(),
            DOWN => self.state.move_down(),
            BOTTOM => self.state.move_bottom(),
            ROTATE => self.state.rotate(),
            QUIT => self.quit(),
            PAUSE => while input::read() != PAUSE {},
            _ => {},
        }
    }

    fn initialize_cursess() {
        initscr();
        cbreak();                // unbuffered input
        keypad(stdscr(), true);  //  special keys
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); 
        timeout(50);            // wait 50ms for input
    }

    fn initialize_colors() {
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

    fn print_title() {
        let start = COLS() / 2 - 26;
        let mut x = start;
        let mut y = 1;
        for c in TITLE.chars() {
            match c {
                '\n' => { y += 1; x = start },
                ' ' => x += 1,
                '1'..='7' => { 
                    attron(COLOR_PAIR(c as i16 - 48));
                    mvaddch(y, x, BLOCK); 
                    attroff(COLOR_PAIR(c as i16 - 48));
                    x += 1 
                }, 
                _ => { mvaddch(y, x, c as chtype); x += 1 },
            }
        }
    }

    fn create_score_window() -> WINDOW {
        let x = COLS() / 2 + 4;
        let width = COLS() - x;
        newwin(10, width, 7, x)
    }

    fn create_game_window() -> WINDOW {
        let x = COLS() / 2 - 19;
        let height = GAME_HEIGHT as i32 + 2;
        let width = GAME_WIDTH as i32 * 2 + 2;
        newwin(height, width, 7, x)
    }

    fn update_game_window(&self) {
        box_(self.game_window, 0, 0);
        for y in 0..GAME_HEIGHT {
            for x in 0..GAME_WIDTH {
                let (c, col) = match self.state.get(y, x) {
                    FieldCell::Empty => (' ' as chtype, 0),
                    FieldCell::Occupied(p) => (BLOCK, p as i16 + 1),
                };
                wattron(self.game_window, COLOR_PAIR(col));
                mvwaddch(self.game_window, y as i32 + 1, x as i32 * 2 + 1, c);
                mvwaddch(self.game_window, y as i32 + 1, x as i32 * 2 + 2, c);
                wattroff(self.game_window, COLOR_PAIR(col));
            }
        }
        wrefresh(self.game_window);
    }

    fn update_score_window(&self) {
        wclear(self.score_window);
        mvwprintw(self.score_window, 1, 0, &format!("level: {}", self.state.level));
        mvwprintw(self.score_window, 2, 0, &format!("score: {}", self.state.score));
        mvwprintw(self.score_window, 4, 0, CONTROLS);
        wrefresh(self.score_window);
    }

    fn update(&self) {
        self.update_game_window();
        self.update_score_window();
    }

    fn quit(&self) {
        endwin();
        std::process::exit(0);
    }

    fn prompt_new_game(&mut self) {
        mvwprintw(self.score_window, 1, 0, &format!("You lost :( score: {}", self.state.score));
        mvwprintw(self.score_window, 2, 0, "play another game? (y/n)");
        if input::read_yes_no() {
            self.state = GameState::new();
            self.update();
        } else {
            self.quit();
        }
    }
}

const CONTROLS: &str = "\
LEFT/RIGHT: move left/right
UP: rotate piece
DOWN: move down
SPACE: fast down
P: pause game
Q: quit";

const TITLE: &str = "\
111111  222222  333333  4444444  55  666666
  11    22        33    44   44  55  66    
  11    22222     33    444444   55  666666
  11    22        33    44   44  55      66
  11    222222    33    44    44 55  666666 (Rust)";
