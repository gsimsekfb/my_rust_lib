#export ROBOMONGO_CMAKE_PREFIX_PATH="/Users/gsimsek/Qt5.9.3/5.9.3/clang_64;/Users/gsimsek/robomongo-shell;/Users/gsimsek/code/c/openssl-1.0.2o"
#export ROBOMONGO_CMAKE_PREFIX_PATH="/opt/Qt5.9.3/5.9.3/clang_64;/opt/robo-shell;/opt/openssl-1.1.1f"
export ROBOMONGO_CMAKE_PREFIX_PATH="/opt/Qt5.12.8/5.12.8/clang_64;/opt/robo-shell;/opt/openssl-1.1.1f"

export PATH="/usr/local/opt/mongodb@3.4/bin:/Users/gsimsek/Library/Android/sdk/platform-tools/:/usr/local/Cellar/ffmpeg/3.0.2/bin/:/Users/gsimsek/code/eos/build/programs/eosc/:/Users/gsimsek/code/fabric/fabric-samples/bin:$PATH"
export PATH="/usr/local/opt/llvm/bin:$PATH"

export ANDROID_HOME="/Users/gsimsek/Library/Android/sdk/"

alias ll='ls -lGh'

export WASM_LLVM_CONFIG=~/wasm-compiler/llvm/bin/llvm-config
export LLVM_DIR=/usr/local/Cellar/llvm/4.0.1/lib/cmake/llvm
export BINARYEN_ROOT=~/binaryen

export GOPATH=/usr/local/go

source "$HOME/.cargo/env"

alias code='/Applications/Visual\ Studio\ Code.app/Contents/MacOS/Electron'
alias py='python3'

alias swl='cargo build -p libra-node -p cli && NODE_ENV="test" cargo run -p libra-swarm -- --libra-node target/debug/libra-node -c ~/code/libra/swarm_temp  -n 1 -s --cli-path target/debug/cli'
alias mm='maturin develop && python3 config_prey_predator.py'
alias mr='maturin develop --release && python3 config_prey_predator.py'
alias pp='python3 config_prey_predator.py'

# 0L v5, v6 
alias sw='cargo b -p diem-node -p cli && NODE_ENV="test" cargo r -p diem-swarm -- --diem-node target/debug/diem-node -c /opt/swarm_temp -n 1 -s --cli-path target/debug/cli'
alias mn='NODE_ENV="test" cargo r -p miner -- --swarm-path /opt/swarm_temp --swarm-persona alice start'
alias bs='ol/util/build-stdlib.sh'
alias tt='NODE_ENV="test" cargo test -p diem-framework --test ol_transactional_tests'

alias cl='clear'

export RUSTC_WRAPPER=sccache

alias ta='cargo t --all'
alias tp='cargo t -- --nocapture'
alias ca='cargo c --all'