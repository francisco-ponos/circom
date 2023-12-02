use circom::input_user::Input;

fn main() -> Result<(), ()> {
    use circom::compilation_user::CompilerConfig;
    use circom::execution_user::ExecutionConfig;

    let current_path = std::path::Path::new("circom/examples");

    let user_input = Input {
        input_program: current_path.join("example.circom"),
        out_r1cs: current_path.join("out").join("example.r1cs"),
        out_json_constraints: current_path.join("out").join("example.json"),
        out_wat_code: current_path.join("out").join("example.wat"),
        out_wasm_code: current_path.join("out").join("example_wasm"),
        out_wasm_name: "example".into(),
        out_js_folder: current_path.join("out").join("example_js"),
        out_c_run_name: "example".into(),
        out_c_folder: current_path.join("out").join("example_cpp"),
        out_c_code: current_path.join("out").join("example_cpp").join("example.cpp"),
        out_c_dat: current_path.join("out").join("example.dat"),
        out_sym: current_path.join("out").join("example.sym"),
        c_flag: true,
        wasm_flag: false,
        wat_flag: false,
        r1cs_flag: true,
        sym_flag: false,
        json_constraint_flag: true,
        json_substitution_flag: false,
        main_inputs_flag: false,
        print_ir_flag: false,
        fast_flag: true,
        reduced_simplification_flag: false,
        parallel_simplification_flag: false,
        flag_old_heuristics: false,
        inspect_constraints_flag: false,
        no_rounds: 0,
        flag_verbose: false,
        prime: "bn128".into(),
        link_libraries: vec![],
    };

    let mut program_archive = circom::parser_user::parse_project(&user_input)?;

    circom::type_analysis_user::analyse_project(&mut program_archive)?;

    let config = ExecutionConfig {
        no_rounds: user_input.no_rounds(),
        flag_p: user_input.parallel_simplification_flag(),
        flag_s: user_input.reduced_simplification_flag(),
        flag_f: user_input.unsimplified_flag(),
        flag_old_heuristics: user_input.flag_old_heuristics(),
        flag_verbose: user_input.flag_verbose(),
        inspect_constraints_flag: user_input.inspect_constraints_flag(),
        r1cs_flag: user_input.r1cs_flag(),
        json_constraint_flag: user_input.json_constraints_flag(),
        json_substitution_flag: user_input.json_substitutions_flag(),
        sym_flag: user_input.sym_flag(),
        sym: user_input.sym_file().to_string(),
        r1cs: user_input.r1cs_file().to_string(),
        json_constraints: user_input.json_constraints_file().to_string(),
        prime: user_input.prime(),
    };

    let circuit = circom::execution_user::execute_project(program_archive, config)?;

    let compilation_config = CompilerConfig {
        vcp: circuit,
        debug_output: user_input.print_ir_flag(),
        c_flag: user_input.c_flag(),
        wasm_flag: user_input.wasm_flag(),
        wat_flag: user_input.wat_flag(),
        js_folder: user_input.js_folder().to_string(),
        wasm_name: user_input.wasm_name().to_string(),
        c_folder: user_input.c_folder().to_string(),
        c_run_name: user_input.c_run_name().to_string(),
        c_file: user_input.c_file().to_string(),
        dat_file: user_input.dat_file().to_string(),
        wat_file: user_input.wat_file().to_string(),
        wasm_file: user_input.wasm_file().to_string(),
        produce_input_log: user_input.main_inputs_flag(),
    };

    circom::compilation_user::compile(compilation_config)?;

    Result::Ok(())
}
