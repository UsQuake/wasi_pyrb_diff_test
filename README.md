# Native vs WASM Python/Ruby

 ## Introduction
 
  This project demonstrates differential test between native and WASM(WASI polyfilled version which runs on V8, JavascriptCore, SpiderMonkey) python/ruby.

 ## Setup and Requirements
 
 - Rust version **1.71.1** [2021 edition]
 - Docker(API version **1.4.0**)

 ## How to replicate this experiment
 
 ### 1. Clone this repository.
 
 - ```git clone https://github.com/UsQuake/wasi_pyrb_diff_test.git```

 ### 2. Open the path of repository.

 - ```cd path/to/clone/wasi_pyrb_diff_test```

 ### 3. Build the all the docker images of V8-WASM-python-sandbox, Native-python-sandbox, ..., etc.

 - ***You should build images with same tag with given commands***
 - ```sudo docker image build -t d8_py ./sandboxed_imgs/d8_python_wasi```
 - ```sudo docker image build -t js_py ./sandboxed_imgs/js_python_wasi```
 - ```sudo docker image build -t jsc_py ./sandboxed_imgs/jsc_python_wasi```
 - ```sudo docker image build -t na_py ./sandboxed_imgs/native_python```
 - ```sudo docker image build -t d8_rb ./sandboxed_imgs/d8_ruby_wasi```
 - ```sudo docker image build -t js_rb ./sandboxed_imgs/js_ruby_wasi```
 - ```sudo docker image build -t jsc_rb ./sandboxed_imgs/jsc_ruby_wasi```
 - ```sudo docker image build -t na_rb ./sandboxed_imgs/native_ruby```

 ### 4. Build testcase generator & driver
 - simply build once.
 - ```cargo build```

 ### 5. Then, run simply or you can simply change the code main
 
 - simply run with
 - ```sudo target/debug/main```,
