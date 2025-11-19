pub mod prelude;

pub mod actor;
pub mod address;
pub mod category;
pub mod city;
pub mod country;
pub mod customer;
pub mod film;
pub mod film_actor;
pub mod film_category;
pub mod film_text;
pub mod inventory;
pub mod language;
pub mod payment;
pub mod rental;
pub mod sea_orm_active_enums;
pub mod staff;
pub mod store;

seaography::register_entity_modules!([
    actor,
    address,
    category,
    city,
    country,
    customer,
    film,
    film_actor,
    film_category,
    film_text,
    inventory,
    language,
    payment,
    rental,
    staff,
    store,
]);
seaography::register_active_enums!([sea_orm_active_enums::Rating,]);
