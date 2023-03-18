use crate::test_values::{test_currencies_iter, test_rates_iter};
use crate::types::{
  Currency, CurrencyId, ExchangePath, ExchangeStep, RateId, RateRelation, StepDirection,
};
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

  pub fn compute_all_paths(&self, root_currency_id: CurrencyId) -> Vec<ExchangePath> {
    let mut result = Vec::new();
    let mut queue: VecDeque<(CurrencyId, Vec<ExchangeStep>)> = VecDeque::new();
    let mut visited: HashSet<CurrencyId> = HashSet::new();
    queue.push_back((root_currency_id, vec![]));

    while let Some((active_currency_id, steps_so_far)) = queue.pop_front() {
      visited.insert(active_currency_id);

      let next_steps_to_enqueue = self
        .rates
        .values()
        .filter_map(|rate| match rate {
          _ if rate.from_currency == active_currency_id && !visited.contains(&rate.to_currency) => {
            Some((rate.to_currency, rate.id, StepDirection::Direct))
          }
          _ if rate.to_currency == active_currency_id && !visited.contains(&rate.from_currency) => {
            Some((rate.from_currency, rate.id, StepDirection::Inverse))
          }
          _ => None,
        })
        // direct steps take priority
        .sorted_unstable_by_key(|(_, _, direction)| *direction)
        .unique_by(|(next_currency_id, _, _)| *next_currency_id)
        .map(|(next_currency_id, next_rate_id, direction)| {
          let mut new_step_list = steps_so_far.clone();

          new_step_list.push(ExchangeStep {
            rate_id: next_rate_id,
            direction,
          });

          (next_currency_id, new_step_list)
        });

      queue.extend(next_steps_to_enqueue);

      result.push(ExchangePath {
        target_currency: active_currency_id,
        steps: steps_so_far,
      });
    }
    result
  }
}

pub fn format_step(graph: &ExchangeGraph, step: &ExchangeStep) -> String {
  let rate_rel = graph.rates.get(&step.rate_id).unwrap();
  let from_code = graph.get_currency_code(rate_rel.from_currency).unwrap();
  let to_code = graph.get_currency_code(rate_rel.to_currency).unwrap();
  match step.direction {
    StepDirection::Direct => format!("{} -> {}", from_code, to_code),
    StepDirection::Inverse => format!("{} => {}", to_code, from_code),
  }
}

pub fn format_path(graph: &ExchangeGraph, path: &ExchangePath) -> String {
  path
    .steps
    .iter()
    .map(|step| format_step(graph, step))
    .join(", ")
}
