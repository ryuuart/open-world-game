use syphon::metal_server::SyphonMetalServer;

fn main() {
    let syphon_server = SyphonMetalServer::new("Open World Game");

    syphon_server.stop();
}
