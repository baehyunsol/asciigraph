use hmath::{Ratio, BigInt};

pub fn format_ratio(n: &Ratio) -> String {

    if n.abs().lt_rat(&THOUSAND) {
        n.to_approx_string(8)
    }

    else {
        let bi = n.truncate_bi();

        if bi.abs().lt_bi(&BILLION) {
            bi.to_string()
        }

        else {
            bi.to_scientific_notation(4)
        }

    }

}

lazy_static::lazy_static! {
    pub static ref THOUSAND: Ratio = Ratio::from_i32(1000);
    pub static ref BILLION: BigInt = BigInt::from_i32(1_000_000_000);
}