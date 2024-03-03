// Include the clap crate for parsing command-line arguments
use clap::Parser;

// Define a struct that represents the command-line arguments.
// The derive macro for Parser automagically turns this struct into a command-line argument parser.
#[derive(Parser, Debug)]
// Set metadata for the command-line tool.
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet. This will be provided by the user as a command-line argument.
    // The 'short' and 'long' attributes define the flags for this argument. '-n' for short, '--name' for long.
    #[arg(short, long)]
    name: String,

    /// Number of times to greet. Also provided by the user, with a default value if not specified.
    // Short flag '-c', long flag '--count', and a default value if the user doesn't provide one.
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    // Parse the command-line arguments provided by the user.
    let args = Args::parse();

    // Repeat the greeting for the number of times specified by the 'count' argument.
    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }

    // If you plan to connect to a database, here's where you might set up the connection.
    // For example, if using the `sqlx` crate, you would create a database pool here and
    // potentially pass it to other parts of your application where database access is needed.
    // let db_pool = sqlx::MySqlPool::connect("mysql://...").await?;
    // You would need to add async handling to your main function for this to work.
}
