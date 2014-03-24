#!/usr/bin/env escript


main(_) ->
    ok = erl_ddll:load_driver(".", "mydrv"),
    Port = open_port({spawn, "mydrv"}, []),
    Port ! {self(), {command, "welcome"}},
    receive
        Whatever ->
            io:format("Got ~p~n", [Whatever])
    end.
