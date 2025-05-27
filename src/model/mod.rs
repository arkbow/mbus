use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Position {
    pub symbol_x: String,
    pub symbol_x_decimal: u8,
    pub symbol_y: String,
    pub symbol_y_decimal: u8,
    pub current_price: f64,
    pub bin_step: u16,
    pub active_bin_id: i32,
    pub bins: Vec<Bin>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Bin {
    pub bin_id: i32,
    pub lower_price: f64,
    pub upper_price: f64,
    pub symbol_x_amount: u64,
    pub symbol_y_amount: u64,
    pub fee_x_amount: u64,
    pub fee_y_amount: u64,
}

impl Position {
    pub fn total_x_amount(&self) -> u64 {
        self.bins.iter().map(|bin| bin.symbol_x_amount).sum()
    }

    pub fn total_x_amount_with_decimal(&self) -> f64 {
        self.total_x_amount() as f64 / 10f64.powf(self.symbol_x_decimal as f64)
    }

    pub fn total_y_amount(&self) -> u64 {
        self.bins.iter().map(|bin| bin.symbol_y_amount).sum()
    }

    pub fn total_y_amount_with_decimal(&self) -> f64 {
        self.total_y_amount() as f64 / 10f64.powf(self.symbol_y_decimal as f64)
    }

    pub fn total_fee_x_amount(&self) -> u64 {
        self.bins.iter().map(|bin| bin.fee_x_amount).sum()
    }

    pub fn total_fee_x_amount_with_decimal(&self) -> f64 {
        self.total_fee_x_amount() as f64 / 10f64.powf(self.symbol_x_decimal as f64)
    }

    pub fn total_fee_y_amount(&self) -> u64 {
        self.bins.iter().map(|bin| bin.fee_y_amount).sum()
    }

    pub fn total_fee_y_amount_with_decimal(&self) -> f64 {
        self.total_fee_y_amount() as f64 / 10f64.powf(self.symbol_y_decimal as f64)
    }
}

