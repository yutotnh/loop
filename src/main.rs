extern crate nix;

use clap::{Command, CommandFactory, Parser, ValueHint};
use clap_complete::{generate, Generator, Shell};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, name = "loop")]
struct Opt {
    /// Commands to execute
    #[arg(trailing_var_arg = true, value_hint = ValueHint::CommandWithArguments)]
    command: Vec<String>,

    /// Number of times to greet
    #[arg(long, default_value_t = f64::INFINITY, value_hint = ValueHint::Other)]
    count: f64,

    /// Shell to run instead of the current shell
    #[arg(long, value_hint = ValueHint::ExecutablePath)]
    shell: Option<String>,

    /// Specify update interval
    #[arg(long, default_value_t = 0.0, value_hint = ValueHint::Other, value_name = "SECONDS")]
    interval: f64,

    /// Generate shell completion script for loop
    #[arg(long, value_enum, value_name = "SHELL")]
    completion: Option<Shell>,
}

/// 現在使用しているシェルを返す
fn shell() -> String {
    format!("/proc/{}/exe", nix::unistd::getppid())
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

fn main() {
    let args = Opt::parse();

    let shell = match args.shell {
        None => shell(),
        Some(i) => i,
    };

    let loop_count = args.count as u128;

    if let Some(shell) = args.completion {
        let mut cmd = Opt::command();
        print_completions(shell, &mut cmd);
    } else {
        for _ in 0..loop_count {
            std::process::Command::new(&shell)
                .arg("-c")
                .arg(&args.command.join(" "))
                .status()
                .expect("failed to execute process");

            // 高速化のため、実行間隔が0sの時は sleep() を実行しない
            if args.interval != 0.0 {
                std::thread::sleep(std::time::Duration::from_secs_f64(args.interval));
            }
        }
    }
}
