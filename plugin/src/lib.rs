use blackjack::HalfEdgeMesh;

wit_bindgen_rust::import!("../blackjack.wit");
wit_bindgen_rust::export!("../blackjack_plugin.wit");

struct BlackjackPlugin {}
impl blackjack_plugin::BlackjackPlugin for BlackjackPlugin {
    fn plugin_main(mesh: wit_bindgen_rust::Handle<crate::HalfEdgeMesh>) {
        blackjack::log(&format!(
            "Vertex 0 has position {:?}",
            mesh.vertex_position(0)
        ));
    }
}
