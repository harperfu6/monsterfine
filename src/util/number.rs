use rust_decimal::Decimal;

pub enum Number {
    Int(i32),
    Decimal(Decimal),
}
