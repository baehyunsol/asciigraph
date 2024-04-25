use hmath::Ratio;

#[derive(Clone)]
pub enum SkipValue {
    None,
    Automatic,
    Manual { from: Ratio, to: Ratio },
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
    pub fn manual<T: TryInto<Ratio>, U: TryInto<Ratio>>(from: T, to: U) -> Self {
        let from = from.try_into().unwrap_or(Ratio::zero());
        let to = to.try_into().unwrap_or(Ratio::zero());
        assert!(from.leq_rat(&to));

        SkipValue::Manual {
            from, to
        }
    }

    pub(crate) fn is_automatic(&self) -> bool {
        match self {
            SkipValue::Automatic => true,
            _ => false,
        }
    }
}
