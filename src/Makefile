	
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
	rustc --lib jah_mut.rs -L .
	rustc jah_mut.rs --test -o jah_mut-tests -L .
	./jah_mut-tests	
	rustc --lib fit.rs -L .
	rustc --lib par.rs -L .
	rustc --lib file_append_json.rs -L .
	rustc file_append_json.rs --test -o file_append_json-tests -L .
	./file_append_json-tests
	rustc --lib err_fit.rs -L .
	rustc --lib strand.rs -L .
	rustc strand.rs --test -o strand-tests -L .
	./strand-tests
	rustc --lib parts.rs -L .	
	rustc parts.rs --test -o parts-tests -L .
	./parts-tests