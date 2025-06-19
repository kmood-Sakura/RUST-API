// src/request.rs
use crate::tables::GoodsSearchParams;
use axum::extract::Query;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsQueryParams {
    pub goods_id: Option<String>,
    pub material_code: Option<String>,
    pub goods_name: Option<String>,
    pub price: Option<String>,
    pub volumn_l: Option<String>,
    pub mass_g: Option<String>,
    pub min_volumn_l: Option<String>,
    pub max_volumn_l: Option<String>,
    pub min_mass_g: Option<String>,
    pub max_mass_g: Option<String>,
    pub min_price: Option<String>,
    pub max_price: Option<String>,
}

impl GoodsQueryParams {
    pub fn validate_and_parse(self) -> Result<GoodsSearchParams, String> {
        let mut search_params = GoodsSearchParams::new();

        // Validate and parse goods_id
        if let Some(goods_id_str) = self.goods_id {
            if !Self::is_safe_integer(&goods_id_str) {
                return Err("Invalid goods_id format - contains unsafe characters".to_string());
            }
            search_params.goods_id = Some(
                goods_id_str
                    .parse::<i32>()
                    .map_err(|_| "Invalid integer format for goods_id".to_string())?,
            );
        }

        // Validate and parse material_code
        if let Some(material_code) = self.material_code {
            if !Self::is_safe_string(&material_code) {
                return Err("Invalid material_code - contains unsafe characters".to_string());
            }
            search_params.material_code = Some(material_code);
        }

        // Validate and parse goods_name
        if let Some(goods_name) = self.goods_name {
            if !Self::is_safe_string(&goods_name) {
                return Err("Invalid goods_name - contains unsafe characters".to_string());
            }
            search_params.goods_name = Some(goods_name);
        }

        // Validate and parse price
        if let Some(price_str) = self.price {
            if !Self::is_safe_decimal(&price_str) {
                return Err("Invalid price format - contains unsafe characters".to_string());
            }
            search_params.price = Some(
                rust_decimal::Decimal::from_str(&price_str)
                    .map_err(|_| "Invalid decimal format for price".to_string())?,
            );
        }

        // Validate and parse volumn_l
        if let Some(volumn_l_str) = self.volumn_l {
            if !Self::is_safe_decimal(&volumn_l_str) {
                return Err("Invalid volumn_l format - contains unsafe characters".to_string());
            }
            search_params.volumn_l = Some(
                rust_decimal::Decimal::from_str(&volumn_l_str)
                    .map_err(|_| "Invalid decimal format for volumn_l".to_string())?,
            );
        }

        // Validate and parse mass_g
        if let Some(mass_g_str) = self.mass_g {
            if !Self::is_safe_decimal(&mass_g_str) {
                return Err("Invalid mass_g format - contains unsafe characters".to_string());
            }
            search_params.mass_g = Some(
                rust_decimal::Decimal::from_str(&mass_g_str)
                    .map_err(|_| "Invalid decimal format for mass_g".to_string())?,
            );
        }

        // Validate and parse min_volumn_l
        if let Some(min_volumn_l_str) = self.min_volumn_l {
            if !Self::is_safe_decimal(&min_volumn_l_str) {
                return Err("Invalid min_volumn_l format - contains unsafe characters".to_string());
            }
            search_params.min_volumn_l = Some(
                rust_decimal::Decimal::from_str(&min_volumn_l_str)
                    .map_err(|_| "Invalid decimal format for min_volumn_l".to_string())?,
            );
        }

        // Validate and parse max_volumn_l
        if let Some(max_volumn_l_str) = self.max_volumn_l {
            if !Self::is_safe_decimal(&max_volumn_l_str) {
                return Err("Invalid max_volumn_l format - contains unsafe characters".to_string());
            }
            search_params.max_volumn_l = Some(
                rust_decimal::Decimal::from_str(&max_volumn_l_str)
                    .map_err(|_| "Invalid decimal format for max_volumn_l".to_string())?,
            );
        }

        // Validate and parse min_mass_g
        if let Some(min_mass_g_str) = self.min_mass_g {
            if !Self::is_safe_decimal(&min_mass_g_str) {
                return Err("Invalid min_mass_g format - contains unsafe characters".to_string());
            }
            search_params.min_mass_g = Some(
                rust_decimal::Decimal::from_str(&min_mass_g_str)
                    .map_err(|_| "Invalid decimal format for min_mass_g".to_string())?,
            );
        }

        // Validate and parse max_mass_g
        if let Some(max_mass_g_str) = self.max_mass_g {
            if !Self::is_safe_decimal(&max_mass_g_str) {
                return Err("Invalid max_mass_g format - contains unsafe characters".to_string());
            }
            search_params.max_mass_g = Some(
                rust_decimal::Decimal::from_str(&max_mass_g_str)
                    .map_err(|_| "Invalid decimal format for max_mass_g".to_string())?,
            );
        }

        // Validate and parse min_price
        if let Some(min_price_str) = self.min_price {
            if !Self::is_safe_decimal(&min_price_str) {
                return Err("Invalid min_price format - contains unsafe characters".to_string());
            }
            search_params.min_price = Some(
                rust_decimal::Decimal::from_str(&min_price_str)
                    .map_err(|_| "Invalid decimal format for min_price".to_string())?,
            );
        }

        // Validate and parse max_price
        if let Some(max_price_str) = self.max_price {
            if !Self::is_safe_decimal(&max_price_str) {
                return Err("Invalid max_price format - contains unsafe characters".to_string());
            }
            search_params.max_price = Some(
                rust_decimal::Decimal::from_str(&max_price_str)
                    .map_err(|_| "Invalid decimal format for max_price".to_string())?,
            );
        }

        Ok(search_params)
    }

    // SQL injection prevention - validate input contains only safe characters for integers
    fn is_safe_integer(input: &str) -> bool {
        // Allow only digits and negative signs for integers
        !input.is_empty() 
            && input.chars().all(|c| c.is_ascii_digit() || c == '-')
            && input.matches('-').count() <= 1  // Only one negative sign allowed
            && !input.starts_with("--")  // Prevent SQL comment injection
    }

    // SQL injection prevention - validate input contains only safe characters for strings
    fn is_safe_string(input: &str) -> bool {
        // Explicitly reject SQL injection attempts
        let dangerous_patterns = [
            "select", "insert", "update", "delete", "drop", "alter", "create",
            "union", "script", "--", "/*", "*/", "'", "\"", ";", "\\",
            "exec", "execute", "sp_", "xp_", "declare", "cast", "convert",
            "truncate", "grant", "revoke", "into", "from", "where", "having",
            "order by", "group by", "information_schema", "sys.", "pg_",
            "waitfor", "delay", "sleep", "benchmark", "extractvalue",
            "updatexml", "ascii", "char(", "chr(", "substring", "mid(",
            "concat", "||", "&&", "|", "&", "^", "~", "<<", ">>",
            "0x", "null", "true", "false", "if(", "case when", "else",
            "then", "end", "exists", "not exists", "in(", "not in",
            "between", "like", "regexp", "rlike", "sounds like"
        ];
        
        let input_lower = input.to_lowercase();
        
        // Check for dangerous patterns
        for pattern in &dangerous_patterns {
            if input_lower.contains(pattern) {
                return false;
            }
        }
        
        // Additional character validation - only allow safe characters
        input.chars().all(|c| {
            c.is_alphanumeric() 
            || c.is_whitespace() 
            || matches!(c, '.' | '-' | '_' | '*' | '(' | ')' | '[' | ']' | '+' | '/' | '@' | '#')
        })
    }

    // SQL injection prevention - validate input contains only safe characters for decimals
    fn is_safe_decimal(input: &str) -> bool {
        // Allow only digits, decimal points, and negative signs for decimals
        !input.is_empty()
            && input.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '-')
            && input.matches('.').count() <= 1  // Only one decimal point allowed
            && input.matches('-').count() <= 1  // Only one negative sign allowed
            && !input.starts_with("--")  // Prevent SQL comment injection
            && !input.ends_with(".")  // Prevent incomplete decimals
            && !input.starts_with(".")  // Prevent incomplete decimals
    }
}

// Helper function to extract query parameters safely
pub fn extract_query_params(query: Query<HashMap<String, String>>) -> GoodsQueryParams {
    let params = query.0;
    
    GoodsQueryParams {
        goods_id: params.get("goods_id").cloned(),
        material_code: params.get("material_code").cloned(),
        goods_name: params.get("goods_name").cloned(),
        price: params.get("price").cloned(),
        volumn_l: params.get("volumn_l").cloned(),
        mass_g: params.get("mass_g").cloned(),
        min_volumn_l: params.get("min_volumn_l").cloned(),
        max_volumn_l: params.get("max_volumn_l").cloned(),
        min_mass_g: params.get("min_mass_g").cloned(),
        max_mass_g: params.get("max_mass_g").cloned(),
        min_price: params.get("min_price").cloned(),
        max_price: params.get("max_price").cloned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_integer_validation() {
        // Valid integers
        assert!(GoodsQueryParams::is_safe_integer("123"));
        assert!(GoodsQueryParams::is_safe_integer("-456"));
        assert!(GoodsQueryParams::is_safe_integer("0"));
        
        // Invalid integers
        assert!(!GoodsQueryParams::is_safe_integer("123.45"));
        assert!(!GoodsQueryParams::is_safe_integer("123abc"));
        assert!(!GoodsQueryParams::is_safe_integer("--123"));
        assert!(!GoodsQueryParams::is_safe_integer("123; DROP TABLE"));
        assert!(!GoodsQueryParams::is_safe_integer(""));
    }

    #[test]
    fn test_safe_decimal_validation() {
        // Valid decimals
        assert!(GoodsQueryParams::is_safe_decimal("123.45"));
        assert!(GoodsQueryParams::is_safe_decimal("-67.89"));
        assert!(GoodsQueryParams::is_safe_decimal("100"));
        assert!(GoodsQueryParams::is_safe_decimal("0.123"));
        
        // Invalid decimals
        assert!(!GoodsQueryParams::is_safe_decimal("123.45.67"));
        assert!(!GoodsQueryParams::is_safe_decimal("123abc"));
        assert!(!GoodsQueryParams::is_safe_decimal("--123"));
        assert!(!GoodsQueryParams::is_safe_decimal(".123"));
        assert!(!GoodsQueryParams::is_safe_decimal("123."));
        assert!(!GoodsQueryParams::is_safe_decimal(""));
    }

    #[test]
    fn test_safe_string_validation() {
        // Valid strings
        assert!(GoodsQueryParams::is_safe_string("iPhone"));
        assert!(GoodsQueryParams::is_safe_string("Samsung Galaxy"));
        assert!(GoodsQueryParams::is_safe_string("*"));
        assert!(GoodsQueryParams::is_safe_string("APL-123"));
        
        // Invalid strings (SQL injection attempts)
        assert!(!GoodsQueryParams::is_safe_string("'; DROP TABLE users; --"));
        assert!(!GoodsQueryParams::is_safe_string("SELECT * FROM"));
        assert!(!GoodsQueryParams::is_safe_string("UNION SELECT"));
        assert!(!GoodsQueryParams::is_safe_string("INSERT INTO"));
        assert!(!GoodsQueryParams::is_safe_string("/* comment */"));
        assert!(!GoodsQueryParams::is_safe_string("exec sp_"));
    }
}