## Interesting findings

### IVM is still very slow.

Benchmark results on MacBook Pro, M2 Pro chip.

| Time (s) | Description |
| ------ | -------- |
| 4.894  | Vine, 1 core |
| 1.083  | Vine, 12 cores |
| 0.002  | Rust, 1 core |


### Vine executes more instructions

Interesting is that Vine executes `268_254_825` interactions while Rust seems to only execute around `27_000_000` instructions.
However, when looking at the page rank implementaion the computatino depth of the Vine program is only `97_547`.
This seems extremly small and if this number is accurate and the parallism could actually be utilized, significant performance could potentially be gained.