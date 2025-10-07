use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use wasmi_c_api::*;
use wasmi_c_api::wasm_valkind_t::WASM_I32;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        println!("--- Initializing Wasm Environment (C API) ---");

        // 1. Create a Config
        let mut config = wasm_config_new();

        // TODO: Fuel consumption not compatible with standard C API functions
        // wasmi_config_consume_fuel_set(&mut config, true);

        // 2. Create the Engine 
        let engine = wasm_engine_new_with_config(config);
        
        // Create regular wasm_store (this will also support fuel when enabled in config)
        let mut store = wasm_store_new(&engine);
        
        println!("Engine and Store created successfully");

        // 4. Load and compile the Wasm module
        let wasm_file = PathBuf::from("/home/pwang/wasm/rx-wasm-prototype/wat/test.wasm");
        println!("Attempting to read WASM module from: {:?}", wasm_file);
        let wasm_bytes = fs::read(&wasm_file)?;

        let mut wasm_byte_vec = std::mem::zeroed();
        wasm_byte_vec_new(&mut wasm_byte_vec, wasm_bytes.len(), wasm_bytes.as_ptr());

        // 5. Create the Module 
        println!("Creating Module from binary bytes...");
        let module = wasm_module_new(&mut store, &wasm_byte_vec);
        
        if module.is_none() {
            wasm_byte_vec_delete(&mut wasm_byte_vec);
            return Err(format!("Failed to create Wasm Module from {:?}", wasm_file).into());
        }
        let module = module.unwrap();

        // 5. Set up empty imports vector (no imports needed)
        let mut imports = std::mem::zeroed();
        wasm_extern_vec_new_empty(&mut imports);

        let start = Instant::now();

        // 6. Get module export types to find "finish" by name
        let mut export_types = std::mem::zeroed();
        wasm_module_exports(&module, &mut export_types);
        
        let export_types_slice = export_types.as_slice();
        println!("Found {} export types", export_types_slice.len());
        
        let mut finish_export_index = None;
        for (i, export_type) in export_types_slice.iter().enumerate() {
            if let Some(et) = export_type.as_ref() {
                let name_vec = wasm_exporttype_name(et);
                let name_slice = name_vec.as_slice();
                let name_str = std::str::from_utf8(name_slice).unwrap_or("");
                println!("Export {}: {}", i, name_str);
                if name_str == "finish" {
                    finish_export_index = Some(i);
                    break;
                }
            }
        }

        if finish_export_index.is_none() {
            return Err("finish function not found in exports".into());
        }
        let finish_index = finish_export_index.unwrap();

        // 7. Instantiate the module
        println!("Instantiating Module...");
        let instance = wasm_instance_new(&mut store, &module, &imports, None);

        // Clean up
        wasm_extern_vec_delete(&mut imports);
        wasm_byte_vec_delete(&mut wasm_byte_vec);
        wasm_exporttype_vec_delete(&mut export_types);
        wasm_module_delete(module);

        if instance.is_none() {
            return Err("Failed to create Wasm Instance".into());
        }
        let mut instance = instance.unwrap();

        // 8. Get exports
        let mut exports = std::mem::zeroed();
        wasm_instance_exports(&mut instance, &mut exports);

        println!("Instance created successfully");

        // 9. Access exports using as_slice()
        let exports_slice = exports.as_slice();
        println!("Found {} exports", exports_slice.len());

        if exports_slice.is_empty() {
            return Err("No exports found".into());
        }

        // 10. Get the finish function by the index we found
        let finish_export = exports_slice[finish_index].as_ref().ok_or("Finish export is null")?;
        let mut finish_export_owned = finish_export.clone();
        let finish_func = wasm_extern_as_func(&mut *finish_export_owned)
            .ok_or("Finish export is not a function")?;

        // 11. Prepare function call: finish() -> i32
        let mut args = std::mem::zeroed();
        wasm_val_vec_new_empty(&mut args);

        // Create result space
        let result_val = wasm_val_t {
            kind: WASM_I32,
            of: std::mem::zeroed(),
        };
        let mut results = std::mem::zeroed();
        wasm_val_vec_new(&mut results, 1, &result_val);

        // 12. Call the WASM function
        println!("Calling Wasm function: finish()...");
        let start2 = Instant::now();
        
        let trap = wasm_func_call(finish_func, &args, &mut results);

        // TODO: Implement fuel support later
        // println!("Fuel consumption enabled (reporting not implemented yet)");

        let duration2 = start2.elapsed();
        let duration = start.elapsed();

        if !trap.is_null() {
            println!("Function call trapped!");
            
            println!("\n--- Results ---");
            println!("Wasm function TRAPPED");
            println!("Total execution time:    {:?}", duration);
            println!("Function execution time: {:?}", duration2);
        } else {
            // Get the result using as_slice()
            let results_slice = results.as_slice();
            if !results_slice.is_empty() {
                let result = results_slice[0].of.i32;
                
                println!("\n--- Results ---");
                println!("Wasm function returned: {:?}", result);
                println!("Total execution time:    {:?}", duration);
                println!("Function execution time: {:?}", duration2);
            } else {
                println!("No results returned from function");
            }
        }

        // Clean up
        println!("\nCleaning up resources...");
        wasm_val_vec_delete(&mut args);
        wasm_val_vec_delete(&mut results);
        wasm_extern_vec_delete(&mut exports);
        wasm_instance_delete(instance);
        wasm_store_delete(store);
        wasm_engine_delete(engine);

        println!("SUCCESS: Actually executed WASM code!");
    }

    Ok(())
}