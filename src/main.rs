use resume_smg::{Config, StaticGenerator};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let binding = "config.json".to_string();
    let config_path = args.get(1).unwrap_or(&binding);
    let binding_static = "./dist".to_string();
    let output_dir = args.get(2).unwrap_or(&binding_static);

    let config_content = fs::read_to_string(config_path)
        .expect("Failed to read config file");
    let config: Config = serde_json::from_str(&config_content)
        .expect("Failed to parse config file");

    let generator = StaticGenerator::new(config.resume, output_dir.clone());
    generator.generate()?;

    println!("\n🎉 Static MCP site generated successfully!");
    println!("Output directory: {}", output_dir);
    println!("MCP manifest available at: {}/mcp.json", output_dir);

    Ok(())
}
