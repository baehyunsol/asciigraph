use hmath::Ratio;
use crate::convert::IntoRatio;

#[derive(Clone)]
pub enum SkipValue {
    None,
    Automatic,
    Manual { from: Ratio, to: Ratio }
}

impl SkipValue {

    /// Doesn't skip any data.
    pub fn none() -> Self {
        SkipValue::None
    }

    /// The engine decide whether to skip a range or not.
    pub fn automatic() -> Self {
        SkipValue::Automatic
    }

    /// Forces the engine to skip this range. It panics if `from > to`
    pub fn manual<T: IntoRatio, U: IntoRatio>(from: T, to: U) -> Self {
        let from = from.into_ratio();
        let to = to.into_ratio();
        assert!(from.leq_rat(&to));

        SkipValue::Manual {
            from, to
        }
    }

}