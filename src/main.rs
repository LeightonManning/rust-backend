use warp::{Filter, log};
use serde::{Deserialize, Serialize};
mod config;
use config::API_KEY; 

#[derive(Deserialize, Serialize, Debug)]
struct Ingredient {
    aisle: String,
    amount: f32,
    id: i32,
    image: String,
    meta: Vec<String>,
    name: String,
    original: String,
    originalName: String,
    unit: String,
    unitLong: String,
    unitShort: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Recipe {
    id: i32,
    image: String,
    imageType: String,
    likes: i32,
    missedIngredientCount: i32,
    missedIngredients: Vec<Ingredient>,
    title: String,
    unusedIngredients: Vec<Ingredient>,
    usedIngredientCount: i32,
    usedIngredients: Vec<Ingredient>,
}

type RecipeResponse = Vec<Recipe>;  // The API response is a list of recipes

#[derive(Deserialize, Serialize, Debug)]
struct RecipeRequest {
    ingredients: String,
}

// Defining custom error type
#[derive(Debug)]
pub struct MyError {
    // Fields if you want
}

// Implementing std::fmt::Display for MyError
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "There was an error!")  // replace with your own error description
    }
}

// Implementing std::error::Error for MyError
impl std::error::Error for MyError {}

// Implementing warp::reject::Reject for MyError
impl warp::reject::Reject for MyError {}

async fn get_recipes(recipe_request: RecipeRequest) -> Result<impl warp::Reply, warp::Rejection> {
    // Here we make the GET request to the API
    let api_key = API_KEY; // Replace with your actual API key
    let url = format!("https://api.spoonacular.com/recipes/findByIngredients?apiKey={}&ingredients={}&number=10&limitLicense=true&ranking=1&ignorePantry=true",
        api_key, recipe_request.ingredients);
    
    let resp: RecipeResponse = reqwest::get(&url)
        .await
        .map_err(|error| {
            eprintln!("Request error: {}", error);
            warp::reject::custom(MyError{})
        })?
        .json::<RecipeResponse>()
        .await
        .map_err(|error| {
            eprintln!("Deserialization error: {}", error);
            warp::reject::custom(MyError{})
        })?;
        
    Ok(warp::reply::json(&resp))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();  // Initialize the logger
    
    let recipes_route = warp::post()
        .and(warp::path("recipes"))
        .and(warp::body::json())
        .and_then(get_recipes)
        .with(warp::log("processing a recipes request"));  // This logs details about each request

    let server = warp::serve(recipes_route).run(([127, 0, 0, 1], 3030));

    let logger = tokio::task::spawn(async {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));
        loop {
            interval.tick().await;
            println!("Server is running");
        }
    });

    let (_, _) = tokio::join!(server, logger);
}

