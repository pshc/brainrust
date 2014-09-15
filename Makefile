test: bf
	./bf test.txt | diff test.out -

bf: brainrust.rs
	rustc $< -o $@

clean:
	rm -f -- bf
