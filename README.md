# Native vs WASM Python/Ruby Differential Fuzzing

 ## Introduction
 
  This project demonstrates differential fuzzing(test) between native and WASM(WASI polyfilled version which runs on V8, JavascriptCore, SpiderMonkey) python/ruby.
  
 ## Explanation

 ### Purpose
 
  - I want to test WASM engine that imported in browser JS engines(like V8, JavascriptCore, SpiderMonkey).
  - So, I decided to test about that WASM execution result is exactly same with native one's
  - and check execution result between browser js engines is exactly same with one another.

 ### How WASM(WASI) python/ruby works on debug shell of V8, JavascriptCore and SpiderMonkey without WebAPI

  - We implement the javascript polyfill to execute python/ruby interpreters with WebAPI features without WebAPI.
    
    * First, Get the javascript async-thread pool code from dartvm/WASM repository().
    * Second, Get the javascript wasi-polyfill code from WASI-Polyfill repository(https://github.com/bjorn3/browser_wasi_shim).
    * Third, Get the UTF-encoding class from ChatGPT(And I adjust it manually).
    * and Gather all of the above, and adjust to make it works.
    * Lastly, We mapped the all of module file of each interpreters(python, ruby) into Javascript-WASI in-memory file-system
    * with this macro script(https://github.com/UsQuake/wasi_sandbox_generator).
      
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
