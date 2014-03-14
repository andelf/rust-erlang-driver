rust-erlang-driver
==================

Make erlang port driver (erl_driver) with Rust!

proof of concept.

# usage

    rustc mydrv.rs -o mydrv.so


```
> erl
Erlang R16B03-1 (erts-5.10.4) [source] [64-bit] [smp:2:2] [async-threads:10] [hipe] [kernel-poll:false]
Eshell V5.10.4  (abort with ^G)
1> erl_ddll:load_driver(".", "mydrv").
ok
2> open_port({spawn, "mydrv"}, []).
#Port<0.606>
3> v(2) ! {self(), {command, "welcome"}}.
{<0.33.0>,{command,"welcome"}}
4> flush().
Shell got {#Port<0.606>,{data,"Fri, 14 Mar 2014 14:39:30 CST"}}
```
