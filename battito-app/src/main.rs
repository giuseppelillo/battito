mod error;
mod line;

use crate::error::Error;
use crate::line::MyHelper;
use battito_lib::interpreter::{interpret, RunConfig};
use battito_lib::max::Payload;
use battito_lib::SUBDIVISION_DEFAULT;
use nannou_osc as osc;
use nannou_osc::rosc::OscMessage;
use nannou_osc::{Connected, Sender};
use rustyline::completion::FilenameCompleter;
use rustyline::error::ReadlineError;
use rustyline::highlight::MatchingBracketHighlighter;
use rustyline::hint::HistoryHinter;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{ColorMode, EditMode, Editor, Helper};

pub struct Config {
    host: String,
    port: i32,
}

impl Config {
    fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn sender(&self) -> Sender<Connected> {
        osc::sender()
            .expect("Could not bind to default socket")
            .connect(&self.address())
            .expect("Could not connect to socket at address")
    }
}

fn main() {
    let config = Config {
        host: "127.0.0.1".to_string(),
        port: 1234,
    };
    let run_config = RunConfig {
        subdivision: SUBDIVISION_DEFAULT,
    };
    let sender = config.sender();
    let terminal_config = rustyline::config::Config::builder()
        .history_ignore_space(true)
        .edit_mode(EditMode::Vi)
        .color_mode(ColorMode::Enabled)
        .build();
    let p = ">> ".to_string();
    let helper = MyHelper {
        highlighter: MatchingBracketHighlighter::new(),
        colored_prompt: format!("\x1b[1;31m{}\x1b[0m", p),
        completer: FilenameCompleter::new(),
        validator: MatchingBracketValidator::new(),
        hinter: HistoryHinter {},
    };
    let mut editor = Editor::with_config(terminal_config);
    editor.set_helper(Some(helper));
    if editor.load_history("history.txt").is_err() {
        // println!("No previous history.");
    }
    loop {
        match run(&mut editor, &p, &sender, &run_config) {
            Ok(code) => {
                if code == 0 {
                    break;
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    editor.save_history("history.txt").unwrap();
}

fn run<T: Helper>(
    editor: &mut Editor<T>,
    prompt: &str,
    sender: &Sender<Connected>,
    run_config: &RunConfig,
) -> Result<usize, Error> {
    let readline = editor.readline(&prompt);
    match readline {
        Ok(line) => {
            editor.add_history_entry(line.as_str());
            let payload = interpret(&line, run_config)?;
            let packet = to_osc_message(&payload)?;

            sender.send(packet).map_err(Error::from)
        }
        Err(ReadlineError::Interrupted) => {
            // println!("CTRL-C");
            Ok(0)
        }
        Err(ReadlineError::Eof) => {
            // println!("CTRL-D");
            Ok(0)
        }
        Err(err) => {
            println!("Error: {:?}", err);
            Err(Error::InputError)
        }
    }
}

fn to_osc_message(payload: &Payload) -> Result<OscMessage, Error> {
    Ok(OscMessage {
        addr: serde_json::to_string(payload)?,
        args: None,
    })
}
