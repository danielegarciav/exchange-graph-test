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
    from_curr: 0,
    to_curr: 1,
    rate: 2.0,
  },
  RateRelation {
    id: 1,
    from_curr: 1,
    to_curr: 2,
    rate: 3.0,
  },
  RateRelation {
    id: 2,
    from_curr: 2,
    to_curr: 3,
    rate: 5.0,
  },
  RateRelation {
    id: 3,
    from_curr: 4,
    to_curr: 2,
    rate: 11.0,
  },
];
