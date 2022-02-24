use clap::Parser;
use std::io;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// the figure name. Valid values are `cow` and `stegosaurus`
    #[clap(short, long)]
    figure: Option<String>,
}

// print_figure given a figure name prints it.
// Currently accepts `cow` and `stegosaurus`.
fn print_figure(figure: Option<&str>) {
    let cow: &str = r#"        \ ^__^
        \ (oo)\_______
          (__)\       )\/\
             ||----w |
             ||     ||
    "#;

    let stegosaurus = r#"   \                      .       .
    \                    / ` + "`" + `.   .' "
     \           .---.  <    > <    >  .---.
      \          |    \  \ - ~ ~ - /  /    |
    _____           ..-~             ~-..-~
   |     |   \~~~\\.'                    ` + "`" + `./~~~/
  ---------   \__/                         \__/
 .'  O    \     /               /       \  "
(_____,    ` + "`" + `._.'               |         }  \/~~~/
 ` + "`" + `----.          /       }     |        /    \__/
       ` + "`" + `-.      |       /      |       /      ` + "`" + `. ,~~|
           ~-.__|      /_ - ~ ^|      /- _      ` + "`" + `..-'
                |     /        |     /     ~-.     ` + "`" + `-. _  _  _
                |_____|        |_____|         ~ - . _ _ _ _ _>

"#;
    if let Some("stegosaurus") = figure {
        println!("{}", stegosaurus);
    } else {
        println!("{}", cow);
    }
}

// tab_to_spaces converts all tabs found in the strings
// found in the `lines` slice to 4 spaces, to prevent misalignments in
// counting the runes
fn tab_to_spaces(lines: &mut [String]) {
    lines.iter_mut().for_each(|line| {
        *line = line.replace("\t", "    ");
    });
}

// calculate_max_width given a slice of strings returns the length of the
// string with max length
fn calculate_max_width(lines: &[String]) -> usize {
    lines.iter().map(|line| line.chars().count()).max().unwrap()
}

// normalize_string_length takes a slice of strings and appends
// to each one a number of spaces needed to have them all the same number
// of runes
fn normalize_string_length(lines: Vec<String>, max_width: usize) -> Vec<String> {
    lines
        .iter()
        .map(|line| {
            format!(
                "{}{}",
                line.trim(),
                " ".repeat(max_width - line.chars().count())
            )
        })
        .collect()
}

// buildBalloon takes a slice of strings of max width maxwidth
// prepends/appends margins on first and last line, and at start/end of each line
// and returns a string with the contents of the balloon
fn build_balloon(lines: Vec<String>, max_width: usize) -> String {
    let borders = ["/", "\\", "\\", "/", "|", "<", ">"];
    let top = format!(" {}", "-".repeat(max_width + 1));
    let bottom = format!(" {}", "-".repeat(max_width + 1));
    let n = lines.len();
    let mut ret = vec![];

    ret.push(top);
    if n == 1 {
        ret.push(format!("{} {} {}", borders[5], lines[0], borders[6]));
    } else {
        for (i, line) in lines.iter().enumerate() {
            ret.push(if i == 0 {
                format!("{} {} {}", borders[0], line, borders[1])
            } else if i == n - 1 {
                format!("{} {} {}", borders[2], line, borders[3])
            } else {
                format!("{} {} {}", borders[4], line, borders[4])
            });
        }
    }
    ret.push(bottom);

    ret.join("\n")
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut lines = vec![];

    loop {
        let mut line = String::new();
        let n = io::stdin().read_line(&mut line)?;
        if n == 0 {
            break;
        }
        lines.push(line);
    }

    tab_to_spaces(&mut lines);
    let max_width = calculate_max_width(&lines);
    let messages = normalize_string_length(lines, max_width);
    let balloon = build_balloon(messages, max_width);
    println!("{}", balloon);
    print_figure(args.figure.as_deref());

    Ok(())
}
