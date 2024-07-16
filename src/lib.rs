use red4ext_rs::log::logger;
use red4ext_rs::{
    export_plugin, exports, global, wcstr, Exportable, GlobalExport, Plugin, SemVer, U16CStr,
};

pub struct HotReload;

impl Plugin for HotReload {
    const NAME: &'static U16CStr = wcstr!("HotReload");
    const AUTHOR: &'static U16CStr = wcstr!("sahewat");
    const VERSION: SemVer = SemVer::new(0, 1, 0);

    // exports a named global function
    fn exports() -> impl Exportable {
        exports![
            GlobalExport(global!(c"ReloadTest", reload_test)),
            // you can export global functions and classes
            // ClassExport::<MyClass>::builder()
            //     .base("IScriptable")
            //     .methods(methods![
            //         c"GetValue" => MyClass::value,
            //         c"SetValue" => MyClass::set_value,
            //     ])
            //     .build()
        ]
    }
}

export_plugin!(Example);

fn reload_test() -> String {
    // FIXME: replace this with the absolute path of the reload.dll plugin. You must manually copy it.
    let raw_path = "D:\\SteamLibrary\\steamapps\\common\\Cyberpunk 2077\\red4ext\\plugins\\red4ext-repl\\reload.dll";
    let r_path = std::path::Path::new(raw_path);

    let result = match load_and_print(r_path) {
        Ok(m) =>  m.to_string(),
        Err(e) => e.to_string()
    };

    return result.to_string();
}

fn load_and_print(path: &std::path::Path) -> Result<i32, libloading::Error> {
    unsafe {
        let lib = libloading::Library::new(path)?;
        let greet: libloading::Symbol<unsafe extern "C" fn() -> i32> = lib.get(b"interact")?;
        let output = greet();
        return Ok(output);
    }

}