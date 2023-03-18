pub mod exchange_graph;
pub mod test_values;
pub mod types;

use crate::exchange_graph::{format_path, ExchangeGraph};

fn main() {
  let graph = ExchangeGraph::new_test_graph();
  let usd = graph.get_currency_by_id(&0).unwrap();
  let paths = graph.compute_all_paths(usd.id);

  println!("From {}...", usd.code);
  for path in paths {
    if path.target_currency == usd.id {
      continue;
    }

    let to = graph.get_currency_by_id(&path.target_currency).unwrap();
    println!("...to {}: {}", to.code, format_path(&graph, &path));
  }
}

#[cfg(test)]
mod test {
  #[test]
  fn it_works() {
    println!("it works!");
  }
}
