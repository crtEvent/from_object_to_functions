use crate::zettai::zettai::Zettai;

mod exercises {
    pub mod ch01 {
        pub mod prac1_2;
        pub mod prac1_3;
    }
}
mod zettai {
    pub mod zettai;
}

#[tokio::main]
async fn main() {
    let app = Zettai::new();
    app.serve().await;
}
