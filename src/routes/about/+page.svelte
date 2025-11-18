<script>
    import { onMount } from "svelte";
    import { marked } from "marked";

    let markdown = $state("");

</script>

<div class="bg-[#2D3441] h-15 w-1/2 place-self-center mt-40 rounded-xl text-[#D8DDE6] text-xl items-center p-4">
    <div class="typewriter inline-block align-middle mb-10">
        <h1 class="">&gt;&nbsp;whoami</h1>
    </div>
    <!-- TODO: SCROLL ANIMATION FOR CV -->
</div>

<style>
    .typewriter h1 {
        font-family: "JetBrains Mono";
        overflow: hidden;
        border-right: 0.15em solid #D8DDE6;
        display: inline-block;

        animation:
            typing 0.5s steps(8, end),
            blink-caret 0.75s step-end infinite;
    }

    @keyframes typing {
        from {
            width: 0;
        }
        to {
            width: 100%;
        }
    }

    @keyframes blink-caret {
        from,
        to {
            border-color: transparent;
        }
        50% {
            border-color: #D8DDE6;
        }
    }
</style>