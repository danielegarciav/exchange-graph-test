use crate::test_values::{test_currencies_iter, test_rates_iter};
use crate::types::{Currency, CurrencyId, ExchangePath, ExchangeStep, RateId, RateRelation};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct ExchangeGraph {
  currencies: HashMap<CurrencyId, Currency>,
  rates: HashMap<RateId, RateRelation>,
}

impl ExchangeGraph {
  pub fn new_test_graph() -> Self {
    let currencies = HashMap::from_iter(test_currencies_iter().map(|c| (c.id, c)));
    let rates = HashMap::from_iter(test_rates_iter().map(|c| (c.id, c)));
    Self { currencies, rates }
  }

  pub fn get_currency_by_id(&self, id: &CurrencyId) -> Option<&Currency> {
    self.currencies.get(id)
  }

  pub fn get_currency_code(&self, id: CurrencyId) -> Option<&String> {
    self.currencies.get(&id).map(|c| &c.code)
  }

  pub fn compute_all_paths(&self, root_curr_id: CurrencyId) -> Vec<ExchangePath> {
    let mut result = Vec::new();
    let mut queue: VecDeque<(CurrencyId, Vec<ExchangeStep>)> = VecDeque::new();
    let mut visited: HashSet<CurrencyId> = HashSet::new();
    queue.push_back((root_curr_id, vec![]));

    while let Some((current_curr_id, steps_so_far)) = queue.pop_front() {
      visited.insert(current_curr_id);

      let next_steps_to_enqueue = self
        .rates
        .values()
        .filter_map(|rate| match rate {
          _ if rate.from_curr == current_curr_id && !visited.contains(&rate.to_curr) => {
            Some((rate.to_curr, rate.id, false))
          }
          _ if rate.to_curr == current_curr_id && !visited.contains(&rate.from_curr) => {
            Some((rate.from_curr, rate.id, true))
          }
          _ => None,
        })
        // direct steps take priority
        .sorted_unstable_by(|(_, _, is_a_backwards), (_, _, is_b_backwards)| {
          Ord::cmp(is_b_backwards, is_a_backwards)
        })
        .unique_by(|(next_curr_id, _, _)| *next_curr_id)
        .map(|(next_curr_id, next_rate_id, backwards)| {
          let mut new_step_list = steps_so_far.clone();

          new_step_list.push(ExchangeStep {
            rate_id: next_rate_id,
            backwards,
          });

          (next_curr_id, new_step_list)
        });

      queue.extend(next_steps_to_enqueue);

      result.push(ExchangePath {
        target_currency: current_curr_id,
        steps: steps_so_far,
      });
    }
    result
  }
}

pub fn format_step(graph: &ExchangeGraph, step: &ExchangeStep) -> String {
  let rate_edge = graph.rates.get(&step.rate_id).unwrap();
  let from_code = graph.get_currency_code(rate_edge.from_curr).unwrap();
  let to_code = graph.get_currency_code(rate_edge.to_curr).unwrap();
  match step.backwards {
    false => format!("{} -> {}", from_code, to_code),
    true => format!("{} => {}", to_code, from_code),
  }
}

pub fn format_path(graph: &ExchangeGraph, path: &ExchangePath) -> String {
  path
    .steps
    .iter()
    .map(|step| format_step(graph, step))
    .join(", ")
}
