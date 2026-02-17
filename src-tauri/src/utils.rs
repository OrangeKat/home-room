use std::env;

pub struct SupabaseConfig {
    pub url: String,
    pub key: String,
}

pub fn get_supabase_config() -> Result<SupabaseConfig, String> {
    let url = env::var("SUPABASE_URL").map_err(|_| "SUPABASE_URL not set in .env")?;
    let key = env::var("SUPABASE_KEY").map_err(|_| "SUPABASE_KEY not set in .env")?;
    Ok(SupabaseConfig { url, key })
}
