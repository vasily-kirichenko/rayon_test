extern crate time;

#[macro_export]
macro_rules! time { 
  ($name:expr, $x:expr) => {{
      let start = time::PreciseTime::now();
      let r = $x;
      println!("{}: {} elapsed.", $name, start.to(time::PreciseTime::now()));
      r
  }}
}