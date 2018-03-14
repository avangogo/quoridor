extern crate gtk;
extern crate gdk;
extern crate relm;
use self::gtk::*;
use std::sync::{Arc,Mutex};
//use std::rc::Rc;
//use std::cell::RefCell;
use board::*;
//use types::*;
use gui::*;
use self::relm::*;
use self::relm::Widget;
//use self::gdk::EventMask;

//use std;
use std::thread;
use std::boxed::Box;
use std::time::*;
use player_example::*;
use player::*;
use std::env;

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


impl Widget for Win {
    // type of the root widget.
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, _model: Self::Model) -> Self {
        // Create the view using the normal GTK+ method calls.
        let vbox = gtk::Box::new(self::gtk::Orientation::Vertical, 0);
        
        let size = 9;
        let wall_count = 30;
        let game = Arc::new(Mutex::new(Board::new(size, wall_count)));

        let board = vbox.add_widget::<BoardView,_>(relm, game.clone());

        let window = Window::new(WindowType::Toplevel);
        window.set_title("Corridor");

        // Update button
        let plus_button = Button::new_with_label("Update");
        vbox.add(&plus_button);

        let argv : Vec<String> = env::args().collect();
        let argv_string  = &(format!("{:?}", argv))[..];
        
        let label = Label::new(argv_string);
        vbox.add(&label);
        
        // =====================================
        // Game loop
        thread::spawn(move || {
            let mut players : [ Box<Player>; 2] =
                [ Box::new(TomRandom::new(size, wall_count, true)),
                  Box::new(TomRandom::new(size, wall_count, false)) ];
            for i in 0..2 {
                if argv.len() > i+1 {
                    let script = &argv[i+1][..];
                    println!("Script for player {}: {}", i, script);
                    players[i] =
                        Box::new(ProgramPlayer::new(script, size, wall_count, i==0));
                }
            }
            
            loop {
                let i = game.lock().unwrap().active_player;
                println!("Turn of Player {}", i);
                match players[i].output() {
                    Some(m) => {
                        game.lock().unwrap().apply_move(m);
                        println!("{}", m);
                        players[1-i].input(m);
                        if let Some(i) = game.lock().unwrap().winner() {
                            println!("Player {} wins!", i);
                            break
                        }
                    },
                    None => (),
                }
                thread::sleep(Duration::from_millis(500)); // TEMPS ENTRE LES COUPS
            }
        });
        // ==================================
 
        window.add(&vbox);
        
        window.show_all();

        // Messages.
        connect!(relm, window, connect_delete_event(_, _),
                 return (Some(WinMsg::Quit), Inhibit(false)));
        connect!(plus_button, connect_clicked(_), board, Msg::Update);

        
        Win {
//            name: name,
            window: window,
            board_view: board,
        }
    }
}

//===================
pub fn main() {
    Win::run(()).unwrap();
}
