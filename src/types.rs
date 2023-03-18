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
  pub from_curr: CurrencyId,
  pub to_curr: CurrencyId,
  pub rate: f64,
}

#[derive(Clone, Debug)]
pub struct RateBreadcrumb {
  pub rate_id: RateId,
  pub backwards: bool,
}

#[derive(Debug)]
pub struct ExchangePath {
  pub target_currency: CurrencyId,
  pub breadcrumbs: Vec<RateBreadcrumb>,
}
