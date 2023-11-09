#!/bin/bash

# Function to compile using 'olac'
compile() {
  local file_name="$1"
  
  # 调用olac编译器来编译文件
  olac compile "${file_name}.ola"
  
  # 检查target文件夹是否存在，如果不存在则创建
  [ ! -d "target" ] && mkdir target
  
  # 移动生成的json文件到target文件夹
  mv "${file_name}_asm.json" target/
  mv "${file_name}_abi.json" target/
}

encode() {
  # 初始化参数变量
  local function_sig=""
  local -a args=()

  # 循环遍历所有参数
  # 使用shift来逐个处理
  while (( "$#" )); do
    case "$1" in
      -f) # 如果是-f参数，下一个参数是function_sig
        if [ -n "$2" ] && [ "${2:0:1}" != "-" ]; then
          function_sig=$2
          shift 2
        else
          echo "Error: Argument for $1 is missing" >&2
          exit 1
        fi
        ;;
      -a) # 如果是-a参数，所有后续参数都是args
        while [ -n "$2" ] && [ "${2:0:1}" != "-" ]; do
          args+=("$2")
          shift
        done
        shift
        ;;
      *) # 处理未知参数
        echo "Error: Unsupported flag $1" >&2
        exit 1
        ;;
    esac
  done

  # 检查function_sig是否已提供
  if [ -z "$function_sig" ]; then
    echo "Error: function_sig is required" >&2
    exit 1
  fi

  # 构建命令
  local cmd="./toy_abi -b target/vote_abi.json -f \"$function_sig\""
  for arg in "${args[@]}"; do
    cmd+=" -a \"$arg\""
  done

  # 执行命令
  eval "$cmd"
}

execute() {
  func_name=$1
  input_file="./inputs/input_${func_name}.txt"
  output_dir="./traces"

  # Check if the input file exists
  if [[ -f "$input_file" ]]; then
    # Create the traces directory if it does not exist
    mkdir -p "$output_dir"
    # Run the command with the provided input file
    ./ola run -i target/vote_asm.json -a "$input_file" -o "${output_dir}/trace_${func_name}.json"
    if [[ $? -eq 0 && "$func_name" == "winningProposal" ]]; then
      # Check for the existence of a.json and b.json in the traces directory
      if [[ -f "${output_dir}/trace_contract_init.json" && -f "${output_dir}/trace_vote_proposal.json" && -f "${output_dir}/trace_winningProposal.json" ]]; then
        echo "Congrats! You've completed your first challenge at Ola x ZK Hack Istanbul!"
      fi
    fi
  else
    # Output msg if the input file does not exist
    echo "Input file not found, you can generate input file using encode command."
  fi
}

# Check the first argument to decide which function to call
case "$1" in
    encode)
        encode "${@:2}"
        ;;
    compile)
        compile "$2"
        ;;
    execute)
        execute "$2"
        ;;
    *)
        echo "Invalid command. Use 'compile', 'encode' or 'execute'."
        exit 1
        ;;
esac
