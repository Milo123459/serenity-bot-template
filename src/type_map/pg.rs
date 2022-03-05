use serenity::prelude::TypeMapKey;
use sqlx::PgPool;

pub struct PgMap;
impl TypeMapKey for PgMap {
	type Value = PgPool;
}
