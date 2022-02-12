use clap::Parser;
use encoding_rs::SHIFT_JIS;
use handlebars::Handlebars;
use serde_json::json;
use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

mod command;
mod key;
use command::Command;

/// Generate AviUtl key config list
#[derive(Parser)]
#[clap(version, about)]
struct Args {
    /// AviUtl key config file (*.key)
    input_file: PathBuf,

    /// Output path
    #[clap(short, long)]
    output_file: Option<PathBuf>,

    /// Template file name
    #[clap(short, long, default_value_t = String::from("default.hbs"))]
    template: String,
}

fn main() {
    let args = Args::parse();

    let f = File::open(&args.input_file).expect("ファイルを開けませんでした");
    let mut reader = BufReader::new(f);
    const HEADER: &str = "AviUtl KeyConfigFile version 0.3   \0";
    let mut header = [0u8; 36];

    reader.read(&mut header).expect("読込に失敗しました");
    if HEADER.as_bytes() != header {
        panic!("ヘッダーが正しくない");
    }

    let mut data = [0u8; 4];
    let mut commands: Vec<Command> = Vec::new();
    while let Ok(()) = reader.read_exact(&mut data) {
        let id = (data[2] as u32) + ((data[3] as u32) << 8);
        let key = key::Key::new(data[1]);
        let has_name = (data[0] & 0x80) != 0;
        let name = if has_name {
            let mut name: Vec<u8> = Vec::new();
            reader.read_until(0, &mut name).expect("読み取り失敗");
            let (res, _, err) = SHIFT_JIS.decode(&name);
            if err {
                String::new()
            } else {
                res.into_owned()
            }
        } else {
            String::new()
        };
        let com = Command::new(data[0], key, id, name);
        commands.push(com);
    }

    commands.sort_by(|a, b| {
        let order = a.priority().cmp(&b.priority());
        match order {
            Ordering::Equal => a.id().cmp(&b.id()),
            _ => order,
        }
    });

    let mut handlebars = Handlebars::new();
    let mut template = PathBuf::from(std::env::current_exe().unwrap());
    template.pop();
    template.push("template");
    template.push(args.template);
    handlebars
        .register_template_file("template", template)
        .unwrap();

    let output = match args.output_file {
        Some(p) => p,
        None => {
            let mut s = String::from(args.input_file.to_str().unwrap());
            s.push_str(".html");
            PathBuf::from(s)
        }
    };
    let mut f = File::create(&output).expect("ファイル作成失敗");
    handlebars
        .render_to_write("template", &json!({ "commands": commands }), &mut f)
        .expect("書き込み失敗");
}
