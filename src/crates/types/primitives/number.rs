use std::{num::ParseIntError, string, sync::OnceLock};
/* Is needed to apply macros */
use num_format::{Locale, ToFormattedString};

#[derive(Clone, Copy)]
pub enum NumberValue {
    Integer(i64),
    Float(f64),
}

pub(crate) struct Number {
    name: string::String,
    max_safe_integer: i64,
    min_safe_integer: i64,
    min_value: f64,
    max_value: f64,
    nan: f64,
    value: NumberValue,
}

///! This implementation is for the String struct,
///! which contains the name, length, and value of the string.
///! The methods are push, replace, ends_with, concat, search, start_with, and to_uppercase.
impl Number {
    pub fn new(_value: NumberValue) -> Self {
        Number {
            name: "Number".to_string(),
            max_safe_integer: std::i64::MAX,
            min_safe_integer: std::i64::MIN,
            min_value: std::f64::MIN,
            max_value: std::f64::MAX,
            nan: std::f64::NAN,
            value: _value,
        }
    }

    pub fn to_fixed(&self, fraction_digits: usize) -> String {
        match self.value {
            NumberValue::Integer(value) => format!("{:.*}", fraction_digits, value),
            NumberValue::Float(value) => format!("{:.*}", fraction_digits, value),
        }
    }

    pub fn to_exponential(&self, fraction_digits: usize) -> String {
        match self.value {
            NumberValue::Integer(i) => format!("{:.*e}", fraction_digits, i as f64),
            NumberValue::Float(f) => format!("{:.*e}", fraction_digits, f),
        }
    }

    pub fn to_precision(&self, precision: usize) -> String {
        if precision == 0 {
            return self.to_string();
        }
        match self.value {
            NumberValue::Integer(i) => {
                let f = i as f64;
                let scale = 10_f64.powi(precision as i32);
                let rounded = (f * scale).round() / scale;
                format!("{:.1$}", rounded, precision)
            }
            NumberValue::Float(f) => {
                let scale = 10_f64.powi(precision as i32);
                let rounded = (f * scale).round() / scale;
                format!("{:.1$}", rounded, precision)
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self.value {
            NumberValue::Integer(i) => i.to_string(),
            NumberValue::Float(f) => f.to_string(),
        }
    }

    pub fn value_of(&self) -> f64 {
        match self.value {
            NumberValue::Integer(i) => i as f64,
            NumberValue::Float(f) => f,
        }
    }

    pub fn to_local_string(&self, locale_str: &str) -> String {
        let num = match self.value {
            NumberValue::Integer(i) => i,
            NumberValue::Float(f) => f as i64,
        };
        let locale = match locale_str {
            "ar_BH" => &Locale::ar_BH,
            "ar_DJ" => &Locale::ar_DJ,
            "ar_DZ" => &Locale::ar_DZ,
            "ar_EG" => &Locale::ar_EG,
            "ar_EH" => &Locale::ar_EH,
            "ar_ER" => &Locale::ar_ER,
            "ar_IL" => &Locale::ar_IL,
            "ar_IQ" => &Locale::ar_IQ,
            "ar_JO" => &Locale::ar_JO,
            "ar_KM" => &Locale::ar_KM,
            "ar_KW" => &Locale::ar_KW,
            "ar_LB" => &Locale::ar_LB,
            "ar_LY" => &Locale::ar_LY,
            "ar_MA" => &Locale::ar_MA,
            "ar_MR" => &Locale::ar_MR,
            "ar_OM" => &Locale::ar_OM,
            "ar_PS" => &Locale::ar_PS,
            "ar_QA" => &Locale::ar_QA,
            "ar_SA" => &Locale::ar_SA,
            "ar_SD" => &Locale::ar_SD,
            "ar_SO" => &Locale::ar_SO,
            "ar_SS" => &Locale::ar_SS,
            "ar_SY" => &Locale::ar_SY,
            "ar_TD" => &Locale::ar_TD,
            "ar_TN" => &Locale::ar_TN,
            "ar_YE" => &Locale::ar_YE,
            "as" => &Locale::as_,
            "asa" => &Locale::asa,
            "ast" => &Locale::ast,
            "az" => &Locale::az,
            "az_Cyrl" => &Locale::az_Cyrl,
            "az_Latn" => &Locale::az_Latn,
            "bas" => &Locale::bas,
            "be" => &Locale::be,
            "bem" => &Locale::bem,
            "bez" => &Locale::bez,
            "bg" => &Locale::bg,
            "bm" => &Locale::bm,
            "bn" => &Locale::bn,
            "bn_IN" => &Locale::bn_IN,
            "bo" => &Locale::bo,
            "bo_IN" => &Locale::bo_IN,
            "br" => &Locale::br,
            "brx" => &Locale::brx,
            "bs" => &Locale::bs,
            "bs_Cyrl" => &Locale::bs_Cyrl,
            "bs_Latn" => &Locale::bs_Latn,
            "ca" => &Locale::ca,
            "ca_AD" => &Locale::ca_AD,
            "ca_ES_VALENCIA" => &Locale::ca_ES_VALENCIA,
            "ca_FR" => &Locale::ca_FR,
            "ca_IT" => &Locale::ca_IT,
            "ccp" => &Locale::ccp,
            "ccp_IN" => &Locale::ccp_IN,
            "ce" => &Locale::ce,
            "ceb" => &Locale::ceb,
            "cgg" => &Locale::cgg,
            "chr" => &Locale::chr,
            "ckb" => &Locale::ckb,
            "ckb_IR" => &Locale::ckb_IR,
            "cs" => &Locale::cs,
            "cu" => &Locale::cu,
            "cy" => &Locale::cy,
            "da" => &Locale::da,
            "da_GL" => &Locale::da_GL,
            "dav" => &Locale::dav,
            "de" => &Locale::de,
            "de_AT" => &Locale::de_AT,
            "de_BE" => &Locale::de_BE,
            "de_CH" => &Locale::de_CH,
            "de_IT" => &Locale::de_IT,
            "de_LI" => &Locale::de_LI,
            "de_LU" => &Locale::de_LU,
            "dje" => &Locale::dje,
            "dsb" => &Locale::dsb,
            "dua" => &Locale::dua,
            "dyo" => &Locale::dyo,
            "dz" => &Locale::dz,
            "ebu" => &Locale::ebu,
            "ee" => &Locale::ee,
            "ee_TG" => &Locale::ee_TG,
            "el" => &Locale::el,
            "el_CY" => &Locale::el_CY,
            "en" => &Locale::en,
            "en_001" => &Locale::en_001,
            "en_150" => &Locale::en_150,
            "en_AE" => &Locale::en_AE,
            "en_AG" => &Locale::en_AG,
            "en_AI" => &Locale::en_AI,
            "en_AS" => &Locale::en_AS,
            "en_AT" => &Locale::en_AT,
            "en_AU" => &Locale::en_AU,
            "en_BB" => &Locale::en_BB,
            "en_BE" => &Locale::en_BE,
            "en_BI" => &Locale::en_BI,
            "en_BM" => &Locale::en_BM,
            _ => &Locale::en,
        };
        let num_located = num.to_formatted_string(locale);
        return num_located;
    }

    pub fn parse_float(s: &str) -> &'static Result<f64, std::num::ParseFloatError> {
        static PARSE_FLOAT: OnceLock<Result<f64, std::num::ParseFloatError>> = OnceLock::new();
        return PARSE_FLOAT.get_or_init(|| s.parse::<f64>());
    }

    pub fn parse_int(s: &str) -> &'static Result<i32, ParseIntError> {
        static PARSE_RESULT: OnceLock<Result<i32, ParseIntError>> = OnceLock::new();
        PARSE_RESULT.get_or_init(|| s.parse::<i64>())
    }

    pub fn is_integer(x: usize) -> &'static bool {
        match self.value {
            NumberValue::Integer(_) => true,
            NumberValue::Float(f) => f.fract() == 0.0,
        };
    }

    pub fn to_string_float(x: &f64) -> &'static String {
        static FLOAT_STRING: OnceLock<String> = OnceLock::new();
        FLOAT_STRING.get_or_init(|| x.to_string())
    }

    /* Start of Properties Gets */
    pub fn name(&self) -> &'static string::String {
        static NAME: OnceLock<String> = OnceLock::new();
        NAME.get_or_init(|| self.name.clone())
    }

    pub fn max_safe_integer(&self) -> &'static i64 {
        static MAX_SAFE_INTENGER: OnceLock<i64> = OnceLock::new();
        MAX_SAFE_INTENGER.get_or_init(|| self.max_safe_integer().clone())
    }

    pub fn min_safe_integer(&self) -> &'static i64 {
        static MIN_SAFE_INTENGER: OnceLock<i64> = OnceLock::new();
        MIN_SAFE_INTENGER.get_or_init(|| self.min_safe_integer().clone())
    }

    pub fn min_value(&self) -> &'static f64 {
        static MIN_VALUE: OnceLock<f64> = OnceLock::new();
        MIN_VALUE.get_or_init(|| self.min_value().clone())
    }

    pub fn max_value(&self) -> &'static f64 {
        static MAX_VALUE: OnceLock<f64> = OnceLock::new();
        MAX_VALUE.get_or_init(|| self.max_value().clone())
    }

    pub fn nan(&self) -> &'static f64 {
        static NAN: OnceLock<f64> = OnceLock::new();
        NAN.get_or_init(|| self.nan().clone())
    }
    /* End of properties gets */
}
