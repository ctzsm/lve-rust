pub mod first_app;

use crate::first_app::Run;

fn main() {
    let first_app = crate::first_app::FirstApp {
        w: 800,
        h: 600,
        window_name: "Hello Vulkan!",
    };

    first_app.run();
}
