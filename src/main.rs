fn main() -> Result<(), Box<dyn std::error::Error>> {
    wei_windows::init();
    wei_env::bin_init("wei-wsl");

    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let args = args.join("\", \"");
    let args = &format!("[\"{}\"]", args);
    
    for _ in 0..3 {
        let wsl_url = match std::fs::read_to_string("./wsl.dat") {
            Ok(c) => c,
            Err(_) => {
                run_server()?;
                continue;
            }
        };

        let wei_wsl_server_version = format!("{}/version", wsl_url);

        let data = match ureq::get(&wei_wsl_server_version)
            .set("Content-Type", "application/json")
            .call() {
                Ok(c) => c,
                Err(_) => {
                    run_server()?;
                    continue;
                }
            }.into_string()?;

        if data != "wei-wsl-server" {
            run_server()?;
            continue;
        }

        let url = format!("{}/run", wsl_url);

        let client = reqwest::blocking::Client::new();
        let args: serde_json::Value = serde_json::from_str(args)?;

        let response = client.post(url)
            .json(&args)
            .send()?;
        
        println!("{}", response.text()?);
        return Ok(());
    }
    
    Ok(())
}

fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    match wei_run::command_async("wsl", vec!["wei-wsl-server"]) {
        Ok(_) => {},
        Err(e) => {
            println!("Error: {}", e);
            return Ok(());
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}
