use serenity::{
	client::Context,
	http::Http,
	model::interactions::{
		application_command::{ApplicationCommand, ApplicationCommandInteraction},
		InteractionResponseType,
	},
};

pub async fn info_register(http: &Http) {
	ApplicationCommand::create_global_application_command(&http, |a| {
		a.name("info").description("Template command")
	})
	.await
	.unwrap();
}

pub async fn info_run(ctx: Context, interaction: ApplicationCommandInteraction) {
	if let Err(err) = interaction
		.create_interaction_response(&ctx.http, |r| {
			r.kind(InteractionResponseType::ChannelMessageWithSource)
				.interaction_response_data(|m| m.content("Template command"))
		})
		.await
	{
		println!("Cannot respond to slash command: {}", err);
	}
}
