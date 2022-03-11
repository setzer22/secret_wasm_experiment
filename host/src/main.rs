use std::collections::HashMap;

use blackjack::{Blackjack, Vec3};
use blackjack_plugin::BlackjackPluginData;
use wasmtime::{AsContext, AsContextMut};

use crate::blackjack::BlackjackTables;

wit_bindgen_wasmtime::import!("../blackjack_plugin.wit");
wit_bindgen_wasmtime::export!("../blackjack.wit");

#[derive(Default, Copy, Clone)]
pub struct BlackjackImpl {}

#[derive(Debug)]
pub struct HalfEdgeMesh {
    vertices: HashMap<u64, Vec3>,
}

impl Blackjack for BlackjackImpl {
    type HalfEdgeMesh = HalfEdgeMesh;

    fn half_edge_mesh_vertex_position(
        &mut self,
        self_: &Self::HalfEdgeMesh,
        vid: u64,
    ) -> blackjack::Vec3 {
        self_.vertices[&vid]
    }

    fn log(&mut self, s: &str) {
        println!("{}", s);
    }
}

pub struct HostState {
    blackjack: BlackjackImpl,
    tables: BlackjackTables<BlackjackImpl>,
    plugin_data: BlackjackPluginData,
}

pub fn main() {
    use wasmtime::*;
    let engine = Engine::default();
    let module = Module::new(
        &engine,
        include_bytes!("../../plugin/target/wasm32-unknown-unknown/debug/plugin.wasm"),
    )
    .unwrap();

    let mut linker = Linker::<HostState>::new(&engine);

    let mut state = HostState {
        blackjack: BlackjackImpl::default(),
        tables: BlackjackTables::<BlackjackImpl>::default(),
        plugin_data: BlackjackPluginData::default(),
    };

    let mesh = HalfEdgeMesh {
        vertices: {
            let mut m = HashMap::default();
            m.insert(
                0,
                Vec3 {
                    x: 42.0,
                    y: 33.0,
                    z: 99.0,
                },
            );
            m
        },
    };
    let mesh_handle = state.tables.half_edge_mesh_table.insert(mesh);

    let mut store = Store::new(&engine, state);

    blackjack::add_to_linker(&mut linker, |x| (&mut x.blackjack, &mut x.tables)).unwrap();
    let (plugin, instance) =
        blackjack_plugin::BlackjackPlugin::instantiate(&mut store, &module, &mut linker, |x| {
            &mut x.plugin_data
        })
        .unwrap();

    plugin.plugin_main(&mut store, todo!()); // I feel like I almost got it 😬
}
