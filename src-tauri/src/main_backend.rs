
use chrono::Local;

pub struct MainBackend{
    
}

impl MainBackend{
    pub fn new() -> Self{
        Self{}
    }
    pub fn helloWorld(&self) -> String{
        tracing::info!("helloworld");
        "hello world".to_string()
    }

    pub fn startMainLoop(self){
        tracing::info!("start main loop");
        std::thread::spawn(move || {
            self.mainLoop();
        });
    }

    pub fn mainLoop(self){
        loop {
            tracing::info!("{}", Local::now());
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}