use e9571_lib1::e9571_lib1::{parse_float, conversion_num};

pub mod e9571_math_lib {
    use super::*;

    /// 数值超高精度计算 (Count_Add)
    /// Calculates the percentage change between Real and Close, with 6 decimal places
    pub fn count_add(real: &str, close: &str) -> String {
        let close_val = parse_float(close);
        if close_val == 0.0 {
            return "0".to_string();
        }
        let real_val = parse_float(real);
        let price = (real_val - close_val) / close_val * 100.0;
        conversion_num(price, 6)
    }

    /// 专用涨跌占比计算 (Count_Amo_Add)
    /// Calculates the percentage change between Real and Close, with 2 decimal places
    pub fn count_amo_add(real: &str, close: &str) -> String {
        let close_val = parse_float(close);
        if close_val == 0.0 {
            return "0".to_string();
        }
        let real_val = parse_float(real);
        let price = (real_val - close_val) / close_val;
        conversion_num(price, 2)
    }

    /// 浮点数百分比计算 (Count_Add_float)
    /// Calculates the percentage change between Real and Close, returning a float
    pub fn count_add_float(real: f64, close: f64) -> f64 {
        if close == 0.0 {
            return 0.0;
        }
        ((real - close) / close) * 100.0
    }

    /// 判断两个数字相差是否在 5% 以内 (IsWithinFivePercent)
    /// Returns true if the difference between a and b is within 5% of the larger value
    pub fn is_within_five_percent(a: f64, b: f64) -> bool {
        let diff = (a - b).abs();
        let max_val = a.abs().max(b.abs());
        diff / max_val < 0.05
    }

    /// 计算两个浮点数之间的绝对距离 (CalculateDistance)
    /// Returns the absolute difference between two floats
    pub fn calculate_distance(float1: f64, float2: f64) -> f64 {
        (float1 - float2).abs()
    }
}