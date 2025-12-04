<script>
    import { marked } from "marked";
    import { onMount } from "svelte";

    let markdown = $state("");

    onMount(async () => {
    });
</script>

<main class="w-full h-full mt-20">
    <div class="w-1/2 mx-auto">
        <div class="flex gap-1">
        </div>
    </div>
</main>

<style>
    @keyframes typing {
        from {
            width: 0ch;
        }
        to {
            width: 8ch;
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