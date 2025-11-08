use clap::{Parser, Subcommand, Args, ValueEnum};

#[derive(Parser, Debug)]
#[command(name="ssh-r", version, about="SSH-r(ust) Management Tool")]
pub struct Cli {
    /// Increase verbosity
    #[arg(short, long)]
    pub verbose: bool,

    /// Print config path and exit
    #[arg(long)]
    pub print_config_path: bool,

    #[command(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Connect to an alias. Extra args after `--` are passed to ssh.
    Connect {
        /// Alias name
        alias: String,
        /// Additional args passed to ssh (after user@host)
        #[arg(trailing_var_arg=true)]
        ssh_args: Vec<String>,
    },

    /// Add or update an alias
    Add(AddArgs),

    /// List all aliases
    List,

    /// Print the ssh command for an alias
    Export {
        alias: String
    },

    /// Remove an alias
    Remove {
        alias: String
    },

    /// Compare current version with latest GitHub release
    CheckUpdates,

    /// Print version
    Version,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    #[arg(long)]
    pub name: String,
    #[arg(long)]
    pub host: String,
    #[arg(long)]
    pub user: String,
    #[arg(long, default_value_t = 22)]
    pub port: u16,
    #[arg(long, value_enum)]
    pub auth: AuthType,
    /// Identity file for cert auth
    #[arg(long)]
    pub identity_file: Option<String>,
    /// Password for password auth (optional; ssh may prompt instead)
    #[arg(long)]
    pub password: Option<String>,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum AuthType { Cert, Password }
