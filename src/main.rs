extern crate markov;

use markov::Chain;
use std::env;

fn main() {
  let mut chain = Chain::new();
  for argument in env::args().skip(1) {
    chain.feed_str(argument.as_str());
  }
  println!("{}", chain.generate_str());
}