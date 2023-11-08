use std::fs::File;

use clap::{arg, Arg, Command};
use itertools::Itertools;
use ola_lang_abi::{Abi, Type, Value};
use regex::Regex;

fn main() {
    let matches = Command::new("toy_encoder")
        .version("0.0.1")
        .author("Ola@Istanbul")
        .about("Encode inputs")
        .arg(arg!(-b --abi <INPUT> "Must set a abi file"))
        .arg(arg!(-f --fn_sig <INPUT> "Must set a function signature"))
        .arg(
            Arg::new("args")
                .short('a')
                .help("args")
                .num_args(1..)
                .last(true),
        )
        .get_matches();

    let abi_path = matches
        .get_one::<String>("abi")
        .expect("ABI file is required");
    let abi: Abi = {
        let file = File::open(abi_path.as_str()).expect("failed to open ABI file");
        serde_json::from_reader(file).expect("failed to parse ABI")
    };
    let function_sig = matches
        .get_one::<String>("fn_sig")
        .expect("Method signature is required")
        .as_str();

    let args: Vec<&String> = matches
        .get_many::<String>("args")
        .unwrap_or_default()
        .collect::<Vec<_>>();

    let types = get_args_type_from_function_sig(function_sig.to_string());
    let params = get_params_from_types_and_args(types, args);

    let input = abi
        .encode_input_with_signature(function_sig, &params)
        .unwrap();

    let input_string = input.iter().join(" ");
    println!("{}", input_string);
}

fn get_args_type_from_function_sig(function_sig: String) -> Vec<String> {
    let re = Regex::new(r"\((.*?)\)").unwrap();
    if let Some(caps) = re.captures(&function_sig) {
        if let Some(params) = caps.get(1) {
            let params_str = params.as_str();
            if !params_str.is_empty() {
                return params_str
                    .split(',')
                    .filter_map(|s| {
                        let trimmed = s.trim();
                        if trimmed.is_empty() {
                            None
                        } else {
                            Some(trimmed.to_string())
                        }
                    })
                    .collect();
            }
        }
    }
    Vec::new()
}

fn get_params_from_types_and_args(types: Vec<String>, args: Vec<&String>) -> Vec<Value> {
    let mut params = Vec::new();
    for (i, &arg) in args.iter().enumerate() {
        let arg_type = types.get(i).unwrap();
        let value = match arg_type.as_str() {
            "bool" => Value::Bool(arg.parse::<bool>().unwrap()),
            "string" => Value::String(arg.to_string()),
            "u32" => Value::U32(arg.parse::<u64>().unwrap()),
            "u32[]" => {
                let numbers = get_u64_arr_from_string(arg)
                    .unwrap()
                    .iter()
                    .map(|v| Value::U32(*v))
                    .collect();
                Value::Array(numbers, Type::U32)
            }
            _ => panic!("Unsupported type: {}", arg_type),
        };
        params.push(value);
    }
    params
}

fn get_u64_arr_from_string(input: &str) -> Result<Vec<u64>, String> {
    let parts: Vec<&str> = input
        .trim_matches(|p| p == '[' || p == ']')
        .split(',')
        .collect();

    let mut numbers = Vec::new();
    for part in parts {
        match part.trim().parse::<u64>() {
            Ok(num) => numbers.push(num),
            Err(_) => return Err(format!("Unable to parse '{}'", part)),
        }
    }

    Ok(numbers)
}
