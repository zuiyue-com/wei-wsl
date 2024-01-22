fn main() -> Result<(), Box<dyn std::error::Error>> {
    wei_windows::init();
    wei_env::bin_init("wei-wsl");
    
    Ok(())
}
