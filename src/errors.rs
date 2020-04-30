#[derive(Debug)]
pub enum MathError {
    ConversionFailure(String),
    CalculationFailure(String),
}

#[derive(Debug)]
pub enum GeometryError {
    ConversionFailure(String),
    CalculationFailure(String),
    InstantiationFailure(String),
    MutationFailure(String),
}
