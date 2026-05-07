
use chrono::Local;
use std::sync::Arc;

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

    pub fn startMainLoop(self: Arc<Self>){
        tracing::info!("start main loop");
        tauri::async_runtime::spawn(async move {
            self.mainLoop().await;
        });
    }

    pub async fn mainLoop(self: Arc<Self>){
        loop {
            tracing::info!("{}", Local::now());
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}