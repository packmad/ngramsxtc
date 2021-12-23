# n-grams eXTraCtor

```
cargo build --release
pyLauncher/main.py <file_path>
```



### Examples
```
> ls -lah /bin/perl
-rwxr-xr-x 2 root root 3,4M ott 19  2020 /bin/perl

> time pyLauncher/main.py /bin/perl
#4=869616
#5=695693
#6=579744
pyLauncher/main.py /bin/perl  2,69s user 0,21s system 105% cpu 2,742 total
```

```
> ls -lah /bin/gdb
-rwxr-xr-x 1 root root 8,1M ago  4  2020 /bin/gdb
> time pyLauncher/main.py /bin/gdb
#4=2110050
#5=1688040
#6=1406700
pyLauncher/main.py /bin/gdb  6,72s user 0,55s system 105% cpu 6,910 total
```
