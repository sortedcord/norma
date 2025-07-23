use pipewire as pw;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pw::init();

    let main_loop = pw::MainLoop::new().expect("Can't create main loop");
    let context = pw::Context::new(&main_loop).expect("Can't create context");

    let core = context.connect(None).expect("Can't connect to PipeWire");

    // We'll just list all nodes first
    core.get_registry().add_listener_local()
        .global(|global| {
            if let pw::registry::GlobalType::Node = global.type_() {
                println!("Node: id={} name={:?}", global.id(), global.name());
            }
        });

    println!("Listing available nodes...");
    main_loop.run();
    Ok(())
}