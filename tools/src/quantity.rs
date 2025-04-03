#[derive(Clone, Copy, Debug, PartialEq, serde::Deserialize)]
pub enum Unit {
    Foot,
    Gram,
    Liter,
    Meter,
    USDollar,
}

/// `Quantity` measures the amount of a resource (Input or Output).
#[derive(Clone, Copy, PartialEq, serde::Deserialize)]
pub struct Quantity {
    pub amount: f32,
    pub unit: Option<Unit>,
}

impl std::default::Default for Quantity {
    fn default() -> Self {
        Self {
            amount: 1.0,
            unit: None,
        }
    }
}

impl std::fmt::Debug for Quantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.unit {
            None => write!(f, "{:#}", self.amount),
            Some(unit) => write!(f, "{:#} {:?}", self.amount, unit),
        }
    }
}

impl std::ops::Mul<f32> for Quantity {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            unit: self.unit,
            amount: self.amount * rhs,
        }
    }
}

impl std::ops::Mul<usize> for Quantity {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self {
        Self {
            unit: self.unit,
            amount: self.amount * (rhs as f32),
        }
    }
}

impl std::ops::Add<&Quantity> for Quantity {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self {
        assert_eq!(self.unit, rhs.unit);
        Self {
            unit: self.unit,
            amount: self.amount + rhs.amount,
        }
    }
}

impl std::ops::AddAssign<usize> for Quantity {
    fn add_assign(&mut self, rhs: usize) {
        assert_eq!(self.unit, None);
        self.amount += rhs as f32;
    }
}
