
use structopt::StructOpt;

#[derive (StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt
{

    input: String,
    
    #[structopt(short = "d", long)]
    dead: bool,

}

fn main() {
    let opt = Opt::from_args();
    let message = opt.input;
    let mut eyes = "       ( o o )";
    if opt.dead 
    {
        eyes = "       ( x x )";
    }
    println!("{}", message);
    println!("  \\");
    println!("   \\");
    println!("        /\\_/\\");
    println!("{}", eyes);
    println!("       =( I )=");

}
