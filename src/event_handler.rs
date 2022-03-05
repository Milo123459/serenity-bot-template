use crate::commands::info::{info_register, info_run};
use serenity::{
	async_trait,
	model::{
		gateway::Ready,
		interactions::{
			application_command::ApplicationCommand, Interaction,
			InteractionApplicationCommandCallbackDataFlags, InteractionResponseType,
		},
	},
	prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, ctx: Context, ready: Ready) {
		println!(
			"Shard {} is now online, serving {} guilds",
			ctx.shard_id,
			ready.guilds.len()
		);

		info_register(&ctx.http).await;

		let interactions = ApplicationCommand::get_global_application_commands(&ctx.http).await;
		println!(
			"Global slash commands: {}",
			interactions
				.unwrap_or_else(|_| Vec::new())
				.iter()
				.map(|command| { command.name.clone() })
				.collect::<Vec<String>>()
				.join(", ")
		);
	}

	async fn interaction_create(&self, ctx: Context, raw_interaction: Interaction) {
		if let Some(interaction) = raw_interaction.clone().application_command() {
			// match the command
			match interaction.data.name.as_str() {
                            "info" => {
                                info_run(ctx.clone(), interaction.clone()).await
                            },
                            _ => {
                                    if let Err(err) = interaction
                                        .create_interaction_response(&ctx.http, |response| {
                                            response.kind(
                                                InteractionResponseType::ChannelMessageWithSource,
                                            )
                                            .interaction_response_data(|data| {
                                                data.flags(
                                                    InteractionApplicationCommandCallbackDataFlags::EPHEMERAL,
                                                );
                                                data.content("Command hasn't been implemented yet! This shouldn't happen, so please join the support server to report it!")
                                            })
                                        })
                                        .await
                                    {
                                        println!("Cannot respond to slash command: {}", err);
                                    }
                            }
                        }
		}
	}
}
