#[macro_use]
extern crate wei_log;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    wei_windows::init();
    wei_env::bin_init("wei-wsl");

    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let args = args.join("\", \"");
    let args = &format!("[\"{}\"]", args);
    
    for _ in 0..3 {
        let wsl_url = match std::fs::read_to_string("./wsl.dat") {
            Ok(c) => c,
            Err(e) => {
                info!("wsl.dat not found: {}", e);
                run_server()?;
                continue;
            }
        };

        let wei_wsl_server_version = format!("{}/version", wsl_url);

        let data = match ureq::get(&wei_wsl_server_version)
            .set("Content-Type", "application/json")
            .call() {
                Ok(c) => c,
                Err(e) => {
                    info!("wsl server not found: {}", e);
                    run_server()?;
                    continue;
                }
            }.into_string()?;

        if data != "wei-wsl-server" {
            info!("version not match");
            run_server()?;
            continue;
        }

        let url = format!("{}/run", wsl_url);

        let client = reqwest::blocking::Client::new();
        let args: serde_json::Value = serde_json::from_str(args)?;

        let response = match client.post(url)
            .json(&args)
            .send() {
                Ok(c) => c,
                Err(e) => {
                    info!("request run error: {}", e);
                    run_server()?;
                    continue;
                }
            };
        
        println!("{}", response.text()?);
        return Ok(());
    }
    
    Ok(())
}

fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    wei_run::command("wsl", vec!["killall", "wei-wsl-server"])?;
    wei_run::command_async("wsl", vec!["wei-wsl-server"])?;

    std::thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}
