use crate::types::{Currency, CurrencyId, RateRelation};

#[derive(Clone, Debug)]
pub struct TestCurrency {
  id: CurrencyId,
  code: &'static str,
}

impl From<&TestCurrency> for Currency {
  fn from(value: &TestCurrency) -> Self {
    Self {
      id: value.id,
      code: value.code.to_string(),
    }
  }
}

pub fn test_currencies_iter() -> impl Iterator<Item = Currency> {
  TEST_CURRENCIES.iter().map(|x| x.into())
}

pub fn test_rates_iter() -> impl Iterator<Item = RateRelation> {
  TEST_RATES.iter().cloned()
}

pub const TEST_CURRENCIES: &[TestCurrency] = &[
  TestCurrency { id: 0, code: "USD" },
  TestCurrency { id: 1, code: "CAD" },
  TestCurrency { id: 2, code: "EUR" },
  TestCurrency { id: 3, code: "GBP" },
  TestCurrency { id: 4, code: "WOA" },
];

pub const TEST_RATES: &[RateRelation] = &[
  RateRelation {
    id: 0,
    from_currency: 0,
    to_currency: 1,
    rate: 2.0,
  },
  RateRelation {
    id: 1,
    from_currency: 1,
    to_currency: 2,
    rate: 3.0,
  },
  RateRelation {
    id: 2,
    from_currency: 2,
    to_currency: 3,
    rate: 5.0,
  },
  RateRelation {
    id: 3,
    from_currency: 4,
    to_currency: 2,
    rate: 11.0,
  },
];
