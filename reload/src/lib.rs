use red4ext_rs::call;
use red4ext_rs::RttiSystem;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())

        // FIXME: put absolute path of your log files
        .chain(fern::log_file("reload-output.log")?)
        .apply()?;
    Ok(())
}

#[no_mangle]
extern "C" fn interact() -> i32 {
    setup_logger().unwrap();

    log::info!("Log like this. red4ext::log doesn't work here.");
    let size = 40;
    let ret = call!("OperatorAdd;Int32Int32;Int32" (size, 4i32) -> i32).unwrap();

    return ret;
}
