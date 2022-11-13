use clap::{Parser, Subcommand};
use colored::Colorize;
use matrix_sdk::{ruma, Client};
mod send;
mod util;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The Matrix homeserver to send requests to.
    #[arg(long, env("MATRIX_HOMESERVER"))]
    matrix_homeserver: String,

    /// The Matrix user ID to authenticate with.
    #[arg(long, env("MATRIX_MXID"))]
    matrix_mxid: String,

    /// The Matrix account password to authenticate with.
    #[arg(long, env("MATRIX_PASSWORD"))]
    matrix_password: String,

    /// Log out when finished.
    #[arg(long)]
    logout: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a message.
    Send {
        /// The message to send.
        #[arg(long)]
        msg: String,

        /// The room to send the message to.
        #[arg(long)]
        room: String,

        /// Process the message as a markdown string.
        #[arg(long)]
        markdown: bool,

        /// Don't join the room if the user isn't already in it.
        #[arg(long)]
        no_join: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mxid = match <&ruma::UserId>::try_from(cli.matrix_mxid.as_str()) {
        Ok(mxid) => mxid,
        Err(err) => {
            hw_msg::errorln!(
                "Failed to parse MXID '{}'. [{}]",
                cli.matrix_mxid.green(),
                err
            );
            quit::with_code(exitcode::USAGE);
        }
    };
    let client = match Client::builder()
        .homeserver_url(cli.matrix_homeserver)
        .build()
        .await
    {
        Ok(client) => client,
        Err(err) => {
            hw_msg::errorln!("Unable to create client. [{}]", err);
            quit::with_code(exitcode::UNAVAILABLE);
        }
    };

    if let Err(err) = client
        .login_username(mxid, &cli.matrix_password)
        .send()
        .await
    {
        hw_msg::errorln!("Unable to login. [{}]", err);
        quit::with_code(exitcode::USAGE);
    }

    if let Err(exit_code) = util::sync_client(&client).await {
        quit::with_code(exit_code);
    }

    let exit_code = match cli.command {
        Commands::Send {
            msg,
            room,
            markdown,
            no_join,
        } => send::send(&client, msg, room, markdown, no_join).await,
    };

    if cli.logout {
        if let Err(err) = client.logout().await {
            hw_msg::errorln!("Failed to log out. [{}]", err);
        }
    }

    quit::with_code(exit_code);
}
