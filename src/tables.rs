// src/tables.rs
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Good {
    pub goods_id: i32,
    pub material_code: String,
    pub goods_name: String,
    pub description: Option<Vec<String>>,
    pub price: rust_decimal::Decimal,
    pub volumn_l: rust_decimal::Decimal,
    pub mass_g: rust_decimal::Decimal,
    pub mass_base: i16,
    pub volumn_base: i16,
}

#[derive(Debug, Clone)]
pub struct GoodsSearchParams {
    pub goods_id: Option<i32>,
    pub material_code: Option<String>,
    pub goods_name: Option<String>,
    pub price: Option<rust_decimal::Decimal>,
    pub volumn_l: Option<rust_decimal::Decimal>,
    pub mass_g: Option<rust_decimal::Decimal>,
    pub min_volumn_l: Option<rust_decimal::Decimal>,
    pub max_volumn_l: Option<rust_decimal::Decimal>,
    pub min_mass_g: Option<rust_decimal::Decimal>,
    pub max_mass_g: Option<rust_decimal::Decimal>,
    pub min_price: Option<rust_decimal::Decimal>,
    pub max_price: Option<rust_decimal::Decimal>,
}

impl GoodsSearchParams {
    pub fn new() -> Self {
        Self {
            goods_id: None,
            material_code: None,
            goods_name: None,
            price: None,
            volumn_l: None,
            mass_g: None,
            min_volumn_l: None,
            max_volumn_l: None,
            min_mass_g: None,
            max_mass_g: None,
            min_price: None,
            max_price: None,
        }
    }

    pub fn is_get_all(&self) -> bool {
        matches!(self.goods_name.as_deref(), Some("*"))
            || matches!(self.material_code.as_deref(), Some("*"))
    }
}

#[derive(Clone)]
pub struct GoodsTable {
    pool: PgPool,
}

impl GoodsTable {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn search(&self, params: GoodsSearchParams) -> Result<Vec<Good>, sqlx::Error> {
        // Handle get all case
        if params.is_get_all() {
            return self.get_all().await;
        }

        // Build dynamic query with parameterized statements to prevent SQL injection
        let mut query = "SELECT goods_id, material_code, goods_name, description, price, volumn_l, mass_g, mass_base, volumn_base FROM goods WHERE 1=1".to_string();
        let mut bind_count = 0;
        let mut conditions = Vec::new();

        // Build WHERE conditions safely using parameterized queries
        if params.goods_id.is_some() {
            bind_count += 1;
            conditions.push(format!(" AND goods_id = ${}", bind_count));
        }

        if params.material_code.is_some() && !params.is_get_all() {
            bind_count += 1;
            conditions.push(format!(" AND material_code ILIKE ${}", bind_count));
        }

        if params.goods_name.is_some() && !params.is_get_all() {
            bind_count += 1;
            conditions.push(format!(" AND goods_name ILIKE ${}", bind_count));
        }

        if params.price.is_some() {
            bind_count += 1;
            conditions.push(format!(" AND price = ${}", bind_count));
        }

        if params.volumn_l.is_some() {
            bind_count += 1;
            conditions.push(format!(" AND volumn_l = ${}", bind_count));
        }

        if params.mass_g.is_some() {
            bind_count += 1;
            conditions.push(format!(" AND mass_g = ${}", bind_count));
        }

        if params.min_volumn_l.is_some() {
            bind_count += 1;
            conditions.push(format!(" AND volumn_l >= ${}", bind_count));
        }

        if params.max_volumn_l.is_some() {
            bind_count += 1;
            conditions.push(format!(" AND volumn_l <= ${}", bind_count));
        }

        if params.min_mass_g.is_some() {
            bind_count += 1;
            conditions.push(format!(" AND mass_g >= ${}", bind_count));
        }

        if params.max_mass_g.is_some() {
            bind_count += 1;
            conditions.push(format!(" AND mass_g <= ${}", bind_count));
        }

        if params.min_price.is_some() {
            bind_count += 1;
            conditions.push(format!(" AND price >= ${}", bind_count));
        }

        if params.max_price.is_some() {
            bind_count += 1;
            conditions.push(format!(" AND price <= ${}", bind_count));
        }

        // Append conditions to query
        query.push_str(&conditions.join(""));
        query.push_str(" ORDER BY goods_id ASC");

        // Build and execute query with proper parameter binding
        let mut sql_query = sqlx::query_as::<_, Good>(&query);

        // Bind parameters in the same order as they were added
        if let Some(goods_id) = params.goods_id {
            sql_query = sql_query.bind(goods_id);
        }

        if let Some(material_code) = params.material_code {
            if material_code != "*" {
                sql_query = sql_query.bind(format!("%{}%", material_code));
            }
        }

        if let Some(goods_name) = params.goods_name {
            if goods_name != "*" {
                sql_query = sql_query.bind(format!("%{}%", goods_name));
            }
        }

        if let Some(price) = params.price {
            sql_query = sql_query.bind(price);
        }

        if let Some(volumn_l) = params.volumn_l {
            sql_query = sql_query.bind(volumn_l);
        }

        if let Some(mass_g) = params.mass_g {
            sql_query = sql_query.bind(mass_g);
        }

        if let Some(min_volumn_l) = params.min_volumn_l {
            sql_query = sql_query.bind(min_volumn_l);
        }

        if let Some(max_volumn_l) = params.max_volumn_l {
            sql_query = sql_query.bind(max_volumn_l);
        }

        if let Some(min_mass_g) = params.min_mass_g {
            sql_query = sql_query.bind(min_mass_g);
        }

        if let Some(max_mass_g) = params.max_mass_g {
            sql_query = sql_query.bind(max_mass_g);
        }

        if let Some(min_price) = params.min_price {
            sql_query = sql_query.bind(min_price);
        }

        if let Some(max_price) = params.max_price {
            sql_query = sql_query.bind(max_price);
        }

        sql_query.fetch_all(&self.pool).await
    }

    async fn get_all(&self) -> Result<Vec<Good>, sqlx::Error> {
        sqlx::query_as::<_, Good>(
            "SELECT goods_id, material_code, goods_name, description, price, volumn_l, mass_g, mass_base, volumn_base FROM goods ORDER BY goods_id ASC"
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn verify_table_access(&self) -> Result<(), sqlx::Error> {
        // Since customer role only has SELECT permission, we should not try to create tables
        // The table should already exist and be created by an admin user
        // We'll just verify the table exists by doing a simple query
        
        let result = sqlx::query("SELECT 1 FROM goods LIMIT 1")
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => {
                // Table exists and is accessible
                Ok(())
            }
            Err(e) => {
                // Table doesn't exist or no permission
                tracing::warn!("Cannot access goods table - table may not exist or insufficient permissions: {}", e);
                // Return error so the application can handle it appropriately
                Err(e)
            }
        }
    }
}