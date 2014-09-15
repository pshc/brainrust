RUSTC=rustc -D warnings

test: bf
	./bf test.txt | diff test.out -

bf: brainrust.rs
	$(RUSTC) $< -o $@

clean:
	rm -f -- bf
