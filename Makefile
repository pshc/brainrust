bf: brainrust.rs
	rustc $< -o $@

clean:
	rm -f -- bf
