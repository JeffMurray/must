	
clean: 
	rm -f *.so
	rm -f fits/*.so
	
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
	rustc --lib fits/err_fit.rs -L .
	rustc --lib strand.rs -L .
	rustc strand.rs --test -o strand-tests -L .
	./strand-tests
	rustc --lib parts.rs -L . -L fits	
	rustc parts.rs --test -o parts-tests -L . -L fits
	./parts-tests
	rustc --lib transcriptor.rs -L . -L fits
	rustc --lib stately.rs -L . -L fits	
	rustc --lib fits/file_append_slice.rs -L .
	rustc fits/file_append_slice.rs --test -o fits/file_append_slice-tests -L . -L fits
	./fits/file_append_slice-tests	
	rustc --lib fits/file_get_slice.rs -L . -L fits
	rustc fits/file_get_slice.rs --test -o fits/file_get_slice-tests -L . -L fits
	./fits/file_get_slice-tests	
	rustc --lib fits/doc_slice_prep.rs -L .
	rustc fits/doc_slice_prep.rs --test -o fits/doc_slice_prep-tests -L . -L fits
	./fits/doc_slice_prep-tests	
	rustc --lib fits/stately_tester.rs -L .
	rustc --lib fits/fits.rs -L . -L fits
	rustc fits/fits.rs --test -o fits/fits-tests -L . -L fits
	./fits/fits-tests	
	rustc --lib must_bank.rs -L . -L fits
	rustc must_bank.rs --test -o must_bank-tests -L . -L fits
	./must_bank-tests	
