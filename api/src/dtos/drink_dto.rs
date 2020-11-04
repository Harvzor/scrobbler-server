use core::models::drink::Drink;
use core::models::drink::DrinkWithCount;

#[derive(Serialize)]
pub struct DrinkDto {
    pub id: i32,
    pub name: String,
    pub count: i32,
    pub colour: String,
    pub deleted: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DrinkDtoPost {
    pub name: String,
    pub colour: String,
}

impl DrinkDto {
    pub fn from_drink(drink: Drink) -> DrinkDto {
        DrinkDto {
            id: drink.id,
            name: drink.name,
            count: 0,
            colour: drink.colour,
            deleted: drink.deleted,
        }
    }
    pub fn from_drink_with_count(drink_with_count: DrinkWithCount) -> DrinkDto {
        DrinkDto {
            id: drink_with_count.id,
            name: drink_with_count.name,
            count: drink_with_count.count,
            colour: drink_with_count.colour,
            deleted: drink_with_count.deleted,
        }
    }
}
