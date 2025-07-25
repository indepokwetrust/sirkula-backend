use sqlx::{Pool, Sqlite};

use super::model::*;

pub async fn insert_entity(pool: &Pool<Sqlite>, entity: Entity) -> Result<(), sqlx::Error> {
    let display_name = String::from(entity.display_name);
    let password_hash = String::from(entity.password_hash);
    let entity_type = entity.entity_type.to_string();
    let contact_number = String::from(entity.contact_number);
    let email = String::from(entity.email);
    let website_url: Option<String> = entity.website_url.map(|x| x.into());
    let created_on = entity.created_on.and_utc().timestamp();
    let deleted_on = entity.deleted_on.map(|x| x.and_utc().timestamp());

    sqlx::query!(
        r#"
        INSERT INTO entity (
            id, display_name, password_hash, entity_type,
            contact_number, email, website_url,
            created_on, deleted_on
        )
        VALUES (
            (SELECT IFNULL(MAX(id), 0) + 1 FROM Entity),
            ?, ?, ?, ?, ?, ?, ?, ?
        )
        "#,
        display_name,
        password_hash,
        entity_type,
        contact_number,
        email,
        website_url,
        created_on,
        deleted_on
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_donation_post(
    pool: &Pool<Sqlite>,
    post: DonationPost,
) -> Result<(), sqlx::Error> {
    let donator_entity_id: i64 = post.donator_entity_id.into();
    let item_name = String::from(post.item_name);
    let category = String::from(post.category);
    let pickup_location_latitude = post.pickup_location_latitude;
    let pickup_location_longitude = post.pickup_location_longitude;
    let pickup_address = String::from(post.pickup_address);
    let pickup_city = String::from(post.pickup_city);
    let pickup_location_note = post.pickup_location_note.map(|x| String::from(x));
    let amount: i64 = post.amount.try_into().unwrap();
    let measurement_unit = post.measurement_unit.to_string();
    let description = post.description.map(|x| String::from(x));
    let created_on = post.created_on.and_utc().timestamp();
    let deleted_on = post.deleted_on.map(|x| x.and_utc().timestamp());
    let completed_on = post.completed_on.map(|x| x.and_utc().timestamp());

    sqlx::query!(
        r#"
        INSERT INTO donation_post (
            id, donator_entity_id, item_name, category,
            pickup_location_latitude, pickup_location_longitude,
            pickup_address, pickup_city, pickup_location_note,
            amount, measurement_unit, description, created_on,
            deleted_on, completed_on
        )
        VALUES (
            (SELECT IFNULL(MAX(id), 0) + 1 FROM Entity),
            ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
        )
        "#,
        donator_entity_id,
        item_name,
        category,
        pickup_location_latitude,
        pickup_location_longitude,
        pickup_address,
        pickup_city,
        pickup_location_note,
        amount,
        measurement_unit,
        description,
        created_on,
        deleted_on,
        completed_on,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_request(pool: &Pool<Sqlite>, request: Request) -> Result<(), sqlx::Error> {
    let requester_entity_id: i64 = request.requester_entity_id.into();
    let donation_post_id: i64 = request.donation_post_id.into();
    let intent = request.intent.map(|x| String::from(x));
    let created_on = request.created_on.and_utc().timestamp();
    let deleted_on = request.deleted_on.map(|x| x.and_utc().timestamp());
    let accepted_on = request.deleted_on.map(|x| x.and_utc().timestamp());
    let completed_on = request.completed_on.map(|x| x.and_utc().timestamp());

    sqlx::query!(
        r#"
        INSERT INTO request (
            id, requester_entity_id, donation_post_id, intent,
            created_on, deleted_on, accepted_on, completed_on
        )
        VALUES (
            (SELECT IFNULL(MAX(id), 0) + 1 FROM Entity),
            ?, ?, ?, ?, ?, ?, ?
        )
        "#,
        requester_entity_id,
        donation_post_id,
        intent,
        created_on,
        deleted_on,
        accepted_on,
        completed_on
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_profile_image(
    pool: &Pool<Sqlite>,
    profile_image: ProfileImage,
) -> Result<(), sqlx::Error> {
    let entity_id: i64 = profile_image.entity_id.into();
    let image_url = String::from(profile_image.image_url);

    sqlx::query!(
        r#"
        INSERT INTO profile_image (
        id, entity_id, image_url
        )
        VALUES (
            (SELECT IFNULL(MAX(id), 0) + 1 FROM Entity),
            ?, ?
        )
        "#,
        entity_id,
        image_url
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn insert_donation_image(
    pool: &Pool<Sqlite>,
    donation_image: DonationPostImage,
) -> Result<(), sqlx::Error> {
    let donation_post_id: i64 = donation_image.donation_post_id.into();
    let image_url = String::from(donation_image.image_url);

    sqlx::query!(
        r#"
        INSERT INTO donation_post_image (
        id, donation_post_id, image_url
        )
        VALUES (
            (SELECT IFNULL(MAX(id), 0) + 1 FROM Entity),
            ?, ?
        )
        "#,
        donation_post_id,
        image_url
    )
    .execute(pool)
    .await?;

    Ok(())
}
