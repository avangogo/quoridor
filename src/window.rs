extern crate gtk;
extern crate gdk;
extern crate relm;
extern crate glib;
extern crate futures;
use self::gtk::{Window,WidgetExt,ContainerExt,GtkWindowExt,Label,
                WindowType,Inhibit,LabelExt,
                Separator};
//use self::gtk::{Button,ButtonExt};
use std::sync::{Arc,Mutex};
use board::*;
use types::*;
use board_view::*;
use self::relm::*;
use self::relm::{Widget,Update,Component};
use self::futures::{Sink,Future};
//use errors::PlayerError;
use errors::PlayerError::*;

use std::thread;
use std::boxed::Box;
use std::time::*;
use player_example::Examples::*;
use player::*;
use std::env;
use self::futures::sync::mpsc;
use std::error::Error;

#[derive(Msg)]
enum WinMsg {
    Quit,
}

// Main window
struct Win {
//    name: String,
    window: Window,
    #[allow(dead_code)]
    board_view: Component<BoardView>,
    #[allow(dead_code)]
    game_widget: Component<GameWidget>,
}

impl Update for Win {
    type Model = ();
    type ModelParam = ();
    type Msg = WinMsg;

    fn model(_: &Relm<Self>, _: ()) -> () {
        ()
    }

    fn update(&mut self, event: WinMsg) {
        match event {
            WinMsg::Quit => gtk::main_quit(),
        }
    }
}

const BOARD_SIZE: usize = 9;
const WALLS: usize = 10;
const DELAY: u64 = 100; // Milliseconds

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, _model: Self::Model) -> Self {
        // Create the view using the normal GTK+ method calls.
        let vbox = gtk::Box::new(self::gtk::Orientation::Vertical, 10);
        
        let game = Arc::new(Mutex::new(Board::new(BOARD_SIZE, WALLS)));
        let clicked = Arc::new(Mutex::new(None));
        
        let board_param = (game.clone(), clicked.clone());
        let board = vbox.add_widget::<BoardView,_>(relm, board_param);

        // === params ===
        let argv : Vec<String> = env::args().collect();
        let argv_string  = &(format!("{:?}", argv))[..];
        
        // =========== Players =======
        let mut players : [Box<PlayerLauncher>; 2] =
            [ Box::new(Tom), Box::new(clicked.clone()) ];
        
        for i in 0..2 {
            if argv.len() > i+1 {
                let script = String::from(&argv[i+1][..]);
                 players[i] = Box::new(ProgramLauncher { script: script });
            }}
        
        let game_param = GameModel {
            players: players,
            param: GameParam { size: BOARD_SIZE, walls: WALLS, starts: true },
            game: game.clone(),
            board_stream: board.stream().clone(),
        };

        let game_widget = vbox.add_widget::<GameWidget,_>(relm, game_param);
        
        let window = Window::new(WindowType::Toplevel);
        window.set_title("Corridor");
        
        // Update button
//        let up_button = Button::new_with_label("Update");
//        vbox.add(&up_button);
//      connect!(up_button, connect_clicked(_), board, Msg::Update);
  
        vbox.add(&Separator::new(self::gtk::Orientation::Horizontal));
        let label = Label::new(argv_string);
        vbox.add(&label);
        
        // =====================================
        window.add(&vbox);
        
        window.show_all();

        // Messages.
        connect!(relm, window, connect_delete_event(_, _),
                 return (Some(WinMsg::Quit), Inhibit(false)));
      
        Win {
            window: window,
            board_view: board,
            game_widget: game_widget,
        }
    }
}

// ============
struct GameWidget {
    container: gtk::Box,
    model: GameModel,
    label: Label,
//    text_view: TextView,
}

struct GameModel {
    game: Arc<Mutex<Board>>,
    players: [ Box<PlayerLauncher>; 2 ],
    param: GameParam,
    board_stream: EventStream<Msg>,
}

impl Update for GameWidget {
    type Model = GameModel;
    type ModelParam = GameModel;
    type Msg = Move;

    fn model(_: &Relm<Self>, model: Self::ModelParam) -> Self::Model {
        model
    }

    fn update(&mut self, _event: Move) {
        // Write Turn of..
        let i = self.model.game.lock().unwrap().active_player;
        self.label.set_text(&format!("Turn of {}",
                                     &self.model.players[i].name()[..]));
        // Print the move
//        let buffer = self.text_view.get_buffer().unwrap(); 
//        let mut test_iter = buffer.get_bounds().0; 
//        buffer.insert(&mut test_iter, &format!("{}\n", event)[..]);
        // Update the board
        self.model.board_stream.emit(Msg::Update);
    }
}

impl Widget for GameWidget {
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.container.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let container = gtk::Box::new(self::gtk::Orientation::Vertical, 10);
        let a = format!(
            "<span foreground=\"blue\">{}</span> vs <span foreground=\"red\">{}</span>",
            model.players[0].name(),
            model.players[1].name());
        let l = Label::new(None);
        l.set_markup(a.as_str());
        container.add(&l);
        let label = Label::new(None);

//        let hbox = gtk::Box::new(self::gtk::Orientation::Vertical, 0)
        
        // label
        container.add(&label);
        container.show();
                        
        let (tx, rx) = mpsc::channel(100);

        relm.connect_exec(rx,
                          {
                              |m| {
                                  m
                              }},
                          |_x|
                          {
                              panic!("azerty")
                          });

        let launchers = [ model.players[0].box_clone(),
                          model.players[1].box_clone() ];
        
        start_game_loop(model.game.clone(), launchers, tx, &model.param);
        
            
        GameWidget {
            label: label,
            container: container,
            model: model,
  //          text_view: text_view,
        }
    }
}

fn print_player_error(id: usize, player: &Box<PlayerLauncher>, e: Box<Error>){
    println!("Player {} ({}) produced an error", id, player.name());
    println!("{}", e);
}

fn start_game_loop(game: Arc<Mutex<Board>>,
                   launchers: [ Box<PlayerLauncher>; 2 ],
                   tx: mpsc::Sender<Move>,
                   param: &GameParam) {
    let mut params = [ param.clone(), param.clone()];
    params[0].starts = true;
    params[1].starts = false;

    thread::spawn( move || {
        let mut players : [ Box<Player> ; 2 ]
            = [ launchers[0].start(params[0]),
                launchers[1].start(params[1]) ];
        let mut tx = tx;
        loop {
            thread::sleep(Duration::from_millis(DELAY));
            let i = game.lock().unwrap().active_player;
            //println!("Turn of Player {}", i);
            match players[i].wait_for_output() {
                Ok(m) => {
                    let x = game.lock().unwrap().apply_move(m);
                    match x {
                    Ok(_) => {
                        println!("{}", m);
                        players[1-i].input(m);
                        tx = tx.send(m).wait().unwrap();
                        if let Some(i) = game.lock().unwrap().winner() {
                            println!("Player {} wins!", i);
                            break
                        };;},
                    Err(e) => {
                        print_player_error(i, &launchers[i],
                                           From::from(InvalidMove(e,m)));
                        break
                    }}}
                Err(e) => {
                    print_player_error(i, &launchers[i], e);
                    break
                }
            };}
        println!("Game over");    
    });
}

//===================
pub fn main() {
    Win::run(()).unwrap();
}
