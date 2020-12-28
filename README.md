# running process:

first:build forever/spawn_forever/monitor three programs: 

```
cargo build --all --release
```

the binary file in the path:

```
target/release
```


second: 

mv forever to /use/local/bin path

```
mv target/release/forever /usr/local/bin/
```

third:

run spawn_forever programe

```
target/release/spawn_forever
```


forth:
spawn some forever process

input arg to spawn_forever and enter.then start a new forever process


fifth:
start monitor with the process name(forever) as input argument
and U can see the console print process info
```
target/release/monitor forever
``` 

sixth:
then if U kill some forever process,the the monitor will restart the process
with the same input argument. if U add some forever,the monitor will refresh the
process info too.