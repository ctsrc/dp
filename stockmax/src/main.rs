use std::ffi::OsString;
use std::io::{BufRead, Write};
use std::process::exit;

use anyhow::{Result, anyhow};

fn main () -> Result<()>
{
  // XXX: We accept the the output file path through an environment variable
  //      external behavior compatibility with the HackerRank C++ skeleton.
  let out_env_f = match std::env::var_os("OUTPUT_PATH").as_ref().map(OsString::as_os_str) {
    None => None,
    Some(os_str) => {
      match os_str.to_str() {
        Some("-") => {
          // XXX: On Unix, single dash conventionally means use stdout, and we use
          //      that meaning here as well.
          //      This diverges from the behavior of the HackerRank C++ skeleton,
          //      but that's ok, we only *mostly* want to externally behave the same,
          //      meaning we don't need to *100%* externally behave the same.
          None
        },
        _ => {
          match std::fs::File::create(os_str) {
            Ok(f) => {
              // XXX: We don't use surrounding `' here because {:?} of OsStr wraps values in quotes.
              eprintln!("Output will be written to file {:?}.", os_str);
              Some(f)
            },
            Err(e) => {
              // XXX: We don't use surrounding `' here because {:?} of OsStr wraps values in quotes.
              eprintln!("Failed to create or truncate file {:?}: {}", os_str, e);
              exit(1);
            },
          }
        },
      }
    },
  };

  // TODO: out_arg_f, but error if paths provided via arg and env at same time.

  let mut out_f = out_env_f;

  let stdin = std::io::stdin();
  let mut lines_in = stdin.lock().lines();

  let t: usize = lines_in.next().expect("Need to know number of lines of data.")?.parse()?;

  for _ in 0..t {
    let n: usize = lines_in.next().expect("Need to know n.")?.parse()?;
    let v: Vec<_> = lines_in.next().expect("Need numbers.")?.split(" ")
      .flat_map(str::parse::<u32>).collect();
    if v.len() != n {
      return Err(anyhow!("Incorrect number of numbers."))
    }
    let res = stockmax::stockmax(v);
    match out_f.as_mut() {
      Some(out_f) => out_f.write_fmt(format_args!("{}\n", res))?,
      None => println!("{}", res),
    };
  }

  Ok(())
}
