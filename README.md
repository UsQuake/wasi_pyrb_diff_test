# Native vs WASM Python/Ruby

 ## Introduction
 
  This project demonstrates differential fuzzing(test) between native and WASM(WASI polyfilled version which runs on V8, JavascriptCore, SpiderMonkey) python/ruby.
  
 ## Explain & QnA

 ### Purpose
 
  - I want to test WASM engine that imported in browser JS engines(like V8, JavascriptCore, SpiderMonkey).
  - So, I decided to test whether WASM execution result is exactly same with native one's and whether execution result between browser js engines is exactly same with one another.

 ### What is WASI(WASM System Interface)? not WASM.

  - It was made to run WASM without browser.
  - WASI is the API replacement of posix-api/libc in normal os.
  - With WASI, You can compile and run WASM code on main function as entry point, and you can use normal libc.
  - Only WASM itself can't work without browser or VM. because it's just kind of binary bytecode file.
  - But with WASI you can execute WASM bytecode in VMs like [wasmtime](https://github.com/bytecodealliance/wasmtime). 

 ### So what is WASI-polyfill-browser? You said WASI is made to use WASM without browser?

   - Yes, but there are people who has a interest of those kind of technologies implement this, maybe. I don't know, either. :)
   - Maybe, they may thinks like below,
   - "Oh, If I implement WASI api in JS and WebAPI, then we can use WASI's sandbox features in browser!"
     
 ### How WASI-polyfill-browser python/ruby works on js debug shell without WebAPI?

  - We implement the javascript polyfill to execute python/ruby interpreters with WebAPI features without WebAPI(https://github.com/UsQuake/wasi_sandbox_generator/blob/master/base-wasi-py.js).
    
    * 1. Get the javascript async-thread pool code from dart2wasm repository(https://github.com/dart-lang/sdk/blob/main/pkg/dart2wasm/bin/run_wasm.js).
    * 2. Get the javascript wasi-polyfill code from WASI-Polyfill repository(https://github.com/bjorn3/browser_wasi_shim).
    * 3. Get the UTF-encoding class from ChatGPT(And I adjust it manually).
    * 4. Gather all of the above, and adjust to make it works.
    * 5. We mapped the all of module file of each interpreters(python, ruby) into Javascript-WASI in-memory file-system with this macro script(https://github.com/UsQuake/wasi_sandbox_generator).
      
 ## Requirements
 
 - Rust version **1.71.1** [2021 edition]
 - Docker(API version **1.4.0**)
   
 ## Setup
 
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
 - or adjust frameworks by your own purpose.
