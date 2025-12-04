<script>
    import projects from "$lib/projects.json" with { type: "json" };
    
    let colors = {
        "bash": "#4EAA25",
        "css": "#264DE4",
        "html": "#E34C26",
        "javascript": "#F0DB4F",
        "typescript": "#3178C6",
        "svelte": "#FF3E00",
        "rust": "#DEA584",
        "python": "#3572A5"
    };


    function short(text, n = 250) {
        if (text.length <= n) return text;
        return text.slice(0, n) + "…";
    }

    function get_next_image(project) {
        return (project.curr_img > project.images ? project.curr_img + 1 : 1);
    }

    function get_prev_image(project) {
        return (project.curr_img <= 1 ? 1 : project.curr_img - 1);
    }
</script>

<main class="w-full h-full mt-20">
    <div class="w-1/2 mx-auto">
        {#each projects as project, i}
            <div class="mb-6">
                <div class="flex gap-1">
                    <p class="text-2xl font-semibold">{project.title}</p>
                    {#if project.link !== "none"}
                        <a class="text-xs mt-3 hover:underline" href={project.link} target="_blank" rel="noopener noreferrer">(github)</a>
                    {/if}
                </div>
                <!--<p class="text-gray-700 mt-2">{@html project.description}</p>-->
                {#if !project.more}
                    <!--
                        <div class="relative w-full h-110 place-items-center flex justify-center">
                            <img src="./examples/{project.title}/{project.curr_img}.png" class="absolute rounded w-180 z-10" alt="example image of {project.title}">
                            <img src="./examples/{project.title}/{get_prev_image(project)}.png" class="left-30 absolute rounded w-120 z-1 blur-xs" alt="example image of {project.title}">
                            <img src="./examples/{project.title}/{get_next_image(project)}.png" class="right-30 absolute rounded w-120 z-1 blur-xs" alt="example image of {project.title}">
                        </div>
                    -->
                    {#if project.description !== "none"}
                    <p class="text-gray-700 mt-2">{@html short(project.description, 220)}</p>
                    {/if}
                {:else}
                    {#if project.description !== "none"}
                    <p class="text-gray-700 mt-2">{@html project.description}</p>
                    {/if}
                {/if}
                {#if project.description !== "none"}
                    <button class="flex gap-1 font-bold hover:cursor-pointer" onclick={() => project.more = !project.more}>
                        {#if project.more}
                            <p class="">less</p>
                        {:else}
                            <p class="">more</p>
                        {/if}
                        <p style="transform: rotate({project.more ? "270" : "90"}deg); transition: 0.25s;">&gt;</p>
                    </button>
                {/if}
                <div class="flex mt-1 w-60 h-2">
                    {#each project.languages as lang}
                        <div class="w-4 h-full" style="background-color: {colors[lang.name.toLowerCase()]}; width: {lang.amount}%"></div>
                    {/each}
                </div>
                <div class="flex gap-2 mt-1">
                    {#each project.languages as lang}
                        <p style="color: {colors[lang.name.toLowerCase()]}">{lang.name}</p>
                    {/each}
                </div>
            </div>
        {/each}
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