.PHONY: clean debug

boggle: boggle.rs
	rustc -O boggle.rs

boggle-dbg: boggle.rs
	rustc -o boggle-dbg boggle.rs

debug: boggle-dbg

clean:
	rm -f boggle boggle-dbg
