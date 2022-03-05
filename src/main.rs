mod commands;
mod event_handler;
mod type_map;

use dotenv::dotenv;
use event_handler::Handler;
use serenity::{client::bridge::gateway::GatewayIntents, Client};
use sqlx::PgPool;
use type_map::pg::PgMap;

#[tokio::main]
async fn main() {
	dotenv().ok();
	let token =
		dotenv::var("DISCORD_TOKEN").expect("No DISCORD_TOKEN found in the environment variables");

	let application_id: u64 = dotenv::var("APPLICATION_ID")
		.expect("No APPLICATION_ID found in the environment variables")
		.parse()
		.expect("APPLICATION_ID is not a valid u64");

	let database_url =
		dotenv::var("DATABASE_URL").expect("No DATABASE_URL found in the environment variables");
	let mut client = Client::builder(token)
		.event_handler(Handler)
		.application_id(application_id)
		.intents(GatewayIntents::empty())
		.await
		.expect("Error creating client");
	{
		let mut client_data = client.data.write().await;
		let pool = PgPool::connect(&database_url)
			.await
			.expect("Failed to connect to the PostgreSQL database");
		client_data.insert::<PgMap>(pool);
	}
	if let Err(err) = client.start_autosharded().await {
		println!("Client error: {:?}", err);
	}
}
