#[macro_export]
macro_rules! eq {
  ($a:expr, $b:expr) => {
    let a = &$a;
    let b = &$b;

    let debug_a = format!("{:?}", &a);
    let debug_b = format!("{:?}", &b);

    if a != b {
      ::text_diff::print_diff(&debug_a, &debug_b, " ");
      eprintln!("a = {:?}", a);
      eprintln!("b = {:?}", b);
      panic!("$a != $b");
    }
  };
}