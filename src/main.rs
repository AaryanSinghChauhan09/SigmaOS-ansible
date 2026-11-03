use anyhow::Result;
use clap::Parser;
use ansible::ansible;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    cluster: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = ansible::Config { 
        cluster_mode: cli.cluster, 
        ..Default::default() 
    };
    let mut component = ansible::new(config);
    
    component.initialize()?;
    component.deploy()?;
    
    println!("SigmaOS-ansible deployed");
    Ok(())
}
