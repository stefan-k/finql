use super::AssetHandler;
///! Data handler trait for market quotes
use super::DataError;
use crate::currency::Currency;
use crate::quote::{MarketDataSource, Quote, Ticker};
use chrono::{DateTime, Utc};

/// Handler for globally available market quotes data
pub trait QuoteHandler: AssetHandler {
    // insert, get, update and delete for market data sources
    fn insert_md_source(&mut self, source: &MarketDataSource) -> Result<usize, DataError>;
    fn get_md_source_id(&mut self, source: &str) -> Option<usize>;
    fn insert_if_new_md_source(&mut self, source: &MarketDataSource) -> Result<usize, DataError> {
        match self.get_md_source_id(&source.name) {
            Some(id) => Ok(id),
            None => self.insert_md_source(source),
        }
    }
    fn get_md_source_by_id(&mut self, id: usize) -> Result<MarketDataSource, DataError>;
    fn get_all_md_sources(&mut self) -> Result<Vec<MarketDataSource>, DataError>;
    fn update_md_source(&mut self, source: &MarketDataSource) -> Result<(), DataError>;
    fn delete_md_source(&mut self, id: usize) -> Result<(), DataError>;

    // insert, get, update and delete for market data sources
    fn insert_ticker(&mut self, ticker: &Ticker) -> Result<usize, DataError>;
    fn get_ticker_id(&mut self, ticker: &str) -> Option<usize>;
    fn insert_if_new_ticker(&mut self, ticker: &Ticker) -> Result<usize, DataError> {
        match self.get_md_source_id(&ticker.name) {
            Some(id) => Ok(id),
            None => self.insert_ticker(ticker),
        }
    }
    fn get_ticker_by_id(&mut self, id: usize) -> Result<Ticker, DataError>;
    fn get_all_ticker_for_source(&mut self, source_id: usize) -> Result<Vec<Ticker>, DataError>;
    fn update_ticker(&mut self, ticker: &Ticker) -> Result<(), DataError>;
    fn delete_ticker(&mut self, id: usize) -> Result<(), DataError>;

    // insert, get, update and delete for market data sources
    fn insert_quote(&mut self, quote: &Quote) -> Result<usize, DataError>;
    fn get_last_quote_before(
        &mut self,
        asset_name: &str,
        time: DateTime<Utc>,
    ) -> Result<(Quote, Currency), DataError>;

    fn get_all_quotes_for_ticker(&mut self, ticker_id: usize) -> Result<Vec<Quote>, DataError>;
    fn update_quote(&mut self, quote: &Quote) -> Result<(), DataError>;
    fn delete_quote(&mut self, id: usize) -> Result<(), DataError>;

    // Get and set cash rounding conventions by currency
    // This method never throws, if currency could not be found in table, return 2 by default instead
    fn get_rounding_digits(&mut self, currency: Currency) -> i32;
    fn set_rounding_digits(&mut self, currency: Currency, digits: i32) -> Result<(), DataError>;
}
