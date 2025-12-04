<script>
    import projects from "$lib/projects.json" with { type: "json" };
</script>

<main class="w-full h-full mt-20">
    <div class="w-1/4 mx-auto">
        <div class="w-full h-full gap-10 flex flex-col">
            {#each projects as project}
            {#if project.roadmap}
            <div class="w-full h-full gap-4 flex flex-col">
                <p class="text-2xl font-semibold">{project.title}</p>
                <div class="border-l-2 gap-10 flex flex-col ml-8">
                    {#each project.roadmap as point}
                    <p class="border-b-2 pl-5 w-fit">
                        {point}
                    </p>
                    {/each}
                </div>
            </div>
            {/if}
            {/each}
        </div>
    </div>
</main>

<style>
    main {
        overflow-x: hidden;
        font-family: "JetBrains Mono";
        scrollbar-width: none;
        -ms-overflow-style: none;
    }
    
    *::-webkit-scrollbar {
        display: none;
    }
</style>