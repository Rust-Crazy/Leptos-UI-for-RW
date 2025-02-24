#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    realworld_leptos::setup::init_app(None).await;
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
   
}
