# How to reproduce

```
$ docker build -t iml -f Dockerfile . 
$ mkdir .cargo
$ docker run -it --rm --mount type=bind,src=$PWD,dst=/work --mount type=bind,src=$PWD/.cargo,dst=/root/.cargo iml /bin/bash
(docker) $ cd /work
(docker) $ cargo run --target-dir docker-target
```
