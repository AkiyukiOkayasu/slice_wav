# slice_wav

Slices and writes out a WAV file at intervals of a given number of samples.

# Install

```bash
$ cargo install --git https://github.com/AkiyukiOkayasu/slice_wav
```

# Dependencies

Depends on [SoX](https://sox.sourceforge.net/) and must be able to use the `sox` command.  
It just calls SoX's trim command internally.

# Examples

Slice a wav file every 100 samples.  
```bash
$ slice_wav input.wav --length 100
```


The --interval option allows you to specify the writing interval.  
The first slice writes out 0-100 samples, the second 50-150 samples.  
```bash
$ slice_wav input.wav --length 100 --interval 50
```
