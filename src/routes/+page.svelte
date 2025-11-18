<script>
    import { onDestroy, onMount, tick } from "svelte";
    import {
        get_file,
        list_dir,
        get_home_dir,
        get_file_system,
        get_dir
    } from "$lib/fs.js";
    import DOMPurify from "dompurify";

    let input = $state("");
    let content = $state([]);
    let terminal_element;
    let cursor_pos = $state({ x: 0, y: 0 });
    let cursor_idx = $state(0);
    let curr_dir = $state(get_home_dir("simen"));

    onMount(async () => {
        document.addEventListener("keydown", handle_key_press);
    });

    async function handle_key_press(e) {
        if (e.key.length == 1) {
            input += e.key;
            let temp = DOMPurify.sanitize(input);
            input = temp;
            cursor_idx += 1;
        } else {
            switch (e.key) {
                case "Backspace": {
                    input = input.slice(0, -1);
                    cursor_idx = Math.max(0, cursor_idx - 1);
                    break;
                }
                case "Enter": {
                    handle_execute_cmd();
                    input = "";
                    cursor_idx = 0;
                }
            }
        }

        await tick();
        terminal_element?.scrollTo({
            top: terminal_element.scrollHeight,
            behavior: "instant",
        });
    }

    async function handle_execute_cmd() {
        content.push(`${curr_dir} $ ${input}`);
        let parts = input.split(" ").filter(Boolean);
        switch (parts[0]) {
            case "clear": {
                content = [];
                break;
            }
            case "ls": {
                if (parts[1]) {
                    content.push(list_dir(parts[1]));
                } else {
                    content.push(list_dir(curr_dir));
                }
                break;
            }
            case "cat": {
                let file_path = parts[1];

                if (file_path.startsWith("./")) {
                    let temp_path = `${curr_dir}/${file_path.slice(2, file_path.length)}`;
                    file_path = temp_path;
                } else if (/^[A-Za-z]/.test(file_path.at(0))) {
                    let temp_path = `${curr_dir}/${file_path}`;
                    file_path = temp_path;
                }

                let file_info = get_file(file_path);
                let temp = await (await fetch(file_info.static_file)).text();
                content.push(temp);
                break;
            }
            case "cd": {
                let file_path = parts[1];
                if (file_path.startsWith("./")) {
                    let temp_path = `${curr_dir}/${file_path.slice(2, file_path.length)}`;
                    file_path = temp_path;
                } else if (/^[A-Za-z]/.test(file_path.at(0))) {
                    let temp_path;
                    if (curr_dir === "/") {
                        temp_path = `/${file_path}`
                    } else {
                        temp_path = `${curr_dir}/${file_path}`;
                    }
                    file_path = temp_path;
                }

                let path_exists = get_dir(file_path);
                console.log(path_exists)

                if (path_exists === undefined) {
                    break;
                }

                if (file_path && path_exists != "undefined") {
                    curr_dir = file_path;
                }
                break;
            }
        }
    }
</script>

<main class="w-screen h-screen flex justify-center text-[#D8DDE6] items-center">
    <div class="absolute text-5xl z-10 text-[#2D3441] top-25 typewriter">
        <h1>&gt; Simen Mathiesen</h1>
    </div>
    <div
        id="terminal"
        class="absolute w-1/2 h-1/2 grid place-self-center justify-between bg-[#2D3441] rounded-xl grid-rows-[auto_1fr_auto] grid-cols-1"
    >
        <div class="w-full h-8 flex justify-end pr-2 pt-2 gap-2">
            <div class="w-3 h-3 bg-[#61C553] rounded-full"></div>
            <div class="w-3 h-3 bg-[#F2BF52] rounded-full"></div>
            <div class="w-3 h-3 bg-[#EC6A60] rounded-full"></div>
        </div>
        <div
            class="w-full flex-1 rounded-xl flex flex-col pr-4 pl-4 overflow-y-auto min-h-0"
            bind:this={terminal_element}
        >
            <!-- Enumerate all content -->
            {#each content as c}
                <div class="">
                    <pre>{c}</pre>
                </div>
            {/each}
        </div>
        <div
            type="text"
            name=""
            id="prompt"
            class="rounded-b-xl p-4 focus:outline-none flex items-center"
        >
            <p class="text-[#03D1F6]">{curr_dir}</p>
            <p>&nbsp;$&nbsp;</p>
            {@html input}
            <span class="cursor bg-[#D8DDE6] h-[90%]">&nbsp;</span>
        </div>
    </div>
</main>

<style>
    #terminal {
        color: #d8dde6;
        font-weight: 500;
        font-size: large;
        scrollbar-width: none;
        overflow: auto;
    }

    *::-webkit-scrollbar {
        display: none;
    }

    .cursor {
        display: inline-block;
        animation: blink 1s step-start infinite;
        width: 1ch;
        height: 1.2em;
    }

    @keyframes blink {
        50% {
            background-color: transparent;
        }
    }


    .typewriter h1 {
        font-family: "JetBrains Mono";
        overflow: hidden;
        border-right: 0.15em solid #2d3441;
        white-space: nowrap;
        margin: 0 auto;
        animation:
            typing 2s steps(17, end),
            blink-caret 0.75s step-end infinite;
    }

    /* The typing effect */
    @keyframes typing {
        from {
            width: 0;
        }
        to {
            width: 100%;
        }
    }

    /* The typewriter cursor effect */
    @keyframes blink-caret {
        from,
        to {
            border-color: transparent;
        }
        50% {
            border-color: #2d3441;
        }
    }

</style>
