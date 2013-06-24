
               
bootstrap: bootstrap.rs
	rustc --lib bootstrap.rs -L .

must: must.rs
	rustc --lib must.rs -L .
	
must_test: bootstrap.rs
	rustc must_test.rs -L .
	
jah_args: jah_args.rs
	rustc --lib jah_args.rs -L .

jah_spec: jah_spec.rs
	rustc --lib jah_spec.rs -L .	
	
clean: 
	rm -f *.so
	
all: clean
	rustc --lib bootstrap.rs -L .
	rustc --lib jah_args.rs -L .
	rustc --lib must.rs -L .
	rustc must_test.rs -L .
	./must_test
	rustc must.rs --test -o must-tests -L .
	./must-tests
	rustc --lib jah_spec.rs -L .
	rustc jah_args.rs --test -o jah_args-tests -L .
	./jah_args-tests
	rustc jah_spec.rs --test -o jah_spec-tests -L .
	./jah_spec-tests
	rustc --lib fit.rs -L .
	rustc --lib par.rs -L .
	rustc --lib file_append_json.rs -L .
	rustc file_append_json.rs --test -o file_append_json-tests -L .
	./file_append_json-tests
	rustc --lib err_fit.rs -L .
	
