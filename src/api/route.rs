use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use regex::bytes::Regex;

use super::model::*;
use crate::{
    AppState,
    database::{CastingError, Entity, EntityType, Varchar},
};

#[axum::debug_handler]
pub async fn register(State(app_state): State<AppState>, Json(data): Json<RegistrationData>) -> StatusCode {
    if let Err(x) = _register(app_state, data).await {
        eprintln!("{:#?}", x);
        StatusCode::BAD_REQUEST
    } else {
        StatusCode::CREATED
    }
}

async fn _register(
    app_state: AppState,
    data: RegistrationData,
) -> Result<(), Box<dyn std::error::Error>> {
    let contact_matcher: Regex = Regex::new(r#"\+[0-9]+ [0-9]+"#).unwrap();

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(&data.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .expect("Something went wrong hashing the password")
        .to_string()
        .try_into()?;

    let display_name = data.display_name.try_into()?;
    let entity_type = match data.entity_type.to_lowercase().as_ref() {
        "umkm" => Ok(EntityType::UMKM),
        "perusahaan" => Ok(EntityType::Perusahaan),
        "komunitas" => Ok(EntityType::Komunitas),
        _ => Err(CastingError),
    }?;
    let contact_number = if contact_matcher.is_match(data.contact_number.as_bytes()) {
        data.contact_number.try_into()?
    } else {
        return Err(Box::new(CastingError));
    };
    let email = data.email.try_into()?;
    let website_url: Option<Varchar<400>> = if let Some(x) = data.website_url {
        if let Ok(y) = x.try_into() {
            Some(y)
        } else {
            return Err(Box::new(CastingError));
        }
    } else {
        None
    };

    let created_on = Utc::now().naive_utc();

    let entity = Entity {
        display_name,
        password_hash,
        entity_type,
        contact_number,
        email,
        website_url,
        created_on,
        deleted_on: None,
    };

    crate::database::insert::insert_entity(app_state.get_pool(), entity).await?;

    Ok(())
}
