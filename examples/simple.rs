use pi_hole_api::{PiHoleAPIConfig, UnauthenticatedPiHoleAPI};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = PiHoleAPIConfig::new("http://192.168.0.19".to_string());

    let version = api.get_version();
    println!("{:?}", version);
    Ok(())
}
