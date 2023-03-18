use crate::test_values::{TEST_CURRENCIES, TEST_RATES};
use crate::types::{Currency, CurrencyId, ExchangePath, RateBreadcrumb, RateId, RateRelation};
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct ExchangeGraph {
  currencies: HashMap<CurrencyId, Currency>,
  rates: HashMap<RateId, RateRelation>,
}

impl ExchangeGraph {
  pub fn new_test_graph() -> Self {
    let currencies = HashMap::from_iter(
      TEST_CURRENCIES
        .iter()
        .map(|c| c.into())
        .map(|c: Currency| (c.id, c)),
    );

    let rates = HashMap::from_iter(TEST_RATES.iter().map(|c| (c.id, c.clone())));
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
    let mut queue: VecDeque<(CurrencyId, Vec<RateBreadcrumb>)> = VecDeque::new();
    let mut visited: HashSet<CurrencyId> = HashSet::new();
    queue.push_back((root_curr_id, vec![]));

    while let Some((current_curr_id, breadcrumbs_so_far)) = queue.pop_front() {
      visited.insert(current_curr_id);

      let next_steps = self
        .rates
        .values()
        .filter_map(|rate| match rate {
          _ if rate.from_curr == current_curr_id && !visited.contains(&rate.to_curr) => {
            visited.insert(current_curr_id);
            Some((rate, false))
          }
          _ if rate.to_curr == current_curr_id && !visited.contains(&rate.from_curr) => {
            Some((rate, true))
          }
          _ => None,
        })
        // direct steps take priority
        .sorted_unstable_by(|(_, is_a_backwards), (_, is_b_backwards)| {
          Ord::cmp(is_b_backwards, is_a_backwards)
        })
        .map(|(next_rate, backwards)| {
          let next_curr = match backwards {
            false => next_rate.to_curr,
            true => next_rate.from_curr,
          };

          let mut new_breadcrumbs = breadcrumbs_so_far.clone();
          new_breadcrumbs.push(RateBreadcrumb {
            rate_id: next_rate.id,
            backwards,
          });

          (next_curr, new_breadcrumbs)
        });

      queue.extend(next_steps);

      result.push(ExchangePath {
        target_currency: current_curr_id,
        breadcrumbs: breadcrumbs_so_far,
      });
    }
    result
  }
}

pub fn format_breadcrumb(graph: &ExchangeGraph, b: &RateBreadcrumb) -> String {
  let rate_edge = graph.rates.get(&b.rate_id).unwrap();
  let from_code = graph.get_currency_code(rate_edge.from_curr).unwrap();
  let to_code = graph.get_currency_code(rate_edge.to_curr).unwrap();
  let arrow = match b.backwards {
    false => "->",
    true => "<-",
  };
  format!("{} {} {}", from_code, arrow, to_code)
}

pub fn format_path(graph: &ExchangeGraph, path: &ExchangePath) -> String {
  path
    .breadcrumbs
    .iter()
    .map(|b| format_breadcrumb(graph, b))
    .join(", ")
}
