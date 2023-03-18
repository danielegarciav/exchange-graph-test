pub type CurrencyId = i32;
pub type RateId = i32;

#[derive(Clone, Debug)]
pub struct Currency {
  pub id: CurrencyId,
  pub code: String,
}

#[derive(Clone, Debug)]
pub struct RateRelation {
  pub id: RateId,
  pub from_currency: CurrencyId,
  pub to_currency: CurrencyId,
  pub rate: f64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum StepDirection {
  Direct,
  Inverse,
}

#[derive(Clone, Debug)]
pub struct ExchangeStep {
  pub rate_id: RateId,
  pub direction: StepDirection,
}

#[derive(Debug)]
pub struct ExchangePath {
  pub target_currency: CurrencyId,
  pub steps: Vec<ExchangeStep>,
}
