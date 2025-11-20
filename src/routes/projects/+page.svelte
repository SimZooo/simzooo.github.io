<script>
    let colors = {
        "bash": "#4EAA25",
        "css": "#264DE4",
        "html": "#E34C26",
        "javascript": "#F0DB4F",
        "svelte": "#FF3E00",
        "rust": "#DEA584"
    };

    let show_example = {index: 0, state: false};
    
    let projects = [
        {
            title: "netlens",
            description: "A network visualization site for monitoring and analyzing low-level network traffic in real-time. Some technologies used include Svelte for the frontend and Rust for the backend packet capturing and processing. Features include dynamic graphs, detailed packet information, and customizable views. The networking was done using the low-level <strong>pnet</strong> library in Rust, with a <strong>HTTP</strong> server built using <strong>Actix-web</strong> to serve the frontend and provide real-time data via <strong>WebSockets</strong>.",
            link: "https://github.com/SimZooo/netlens",
            languages: [
                { name: "Svelte", amount: 55 },
                { name: "Rust", amount: 30 },
                { name: "JavaScript", amount: 5 },
                { name: "HTML", amount: 5 },
                { name: "CSS", amount: 5 },
            ]
        },
        {
            title: "netlens-tui",
            description: "A terminal user interface (TUI) for netlens, allowing users to monitor network traffic directly from the command line, with detailed information of low-level networking. Written in Rust. Integrates with my netlens-suite toolchain of penetration testing tools.",
            link: "https://github.com/SimZooo/netlens-suite/tree/main/tools/netlens-tui",
            languages: [{ name: "Rust", amount: 100 }]
        },
        {
            title: "nlnetwork",
            description: "A host-identification tool written in Rust, designed to discover devices on a network. Part of my netlens-suite toolchain of penetration testing tools.",
            link: "https://github.com/SimZooo/netlens-suite/tree/main/tools/nlnetwork",
            languages: [{ name: "Rust", amount: 100 }]
        },
        {
            title: "nlscan",
            description: "A port scanner tool written in Rust, designed to quickly identify open ports on target systems. Part of my netlens-suite toolchain of penetration testing tools.",
            link: "https://github.com/SimZooo/netlens-suite/tree/main/tools/nlscan",
            languages: [{ name: "Rust", amount: 100 }]
        },
        {
            title: "nltrace",
            description: "A network traffic packet sniffer written in Rust, designed to capture and analyze network packets in real-time. Part of my netlens-suite toolchain of penetration testing tools.",
            link: "https://github.com/SimZooo/netlens-suite/tree/main/tools/nltrace",
            languages: [{ name: "Rust", amount: 100 }]
        },
        {
            title: "maclookup",
            description: "A MAC address lookup tool written in Rust, designed to identify device manufacturers based on their MAC addresses.",
            link: "https://github.com/SimZooo/maclookup",
            languages: [{ name: "Rust", amount: 100 }]
        },
        {
            title: "noted",
            description: "A CLI flashcard bash script, designed to help users create, manage, and review flashcards directly from the command line.",
            link: "https://github.com/SimZooo/noted",
            languages: [{ name: "Bash", amount: 100 }]
        },
    ];
</script>

<main class="w-screen h-screen mt-15 p-5">
    <div class="w-1/2 mx-auto">
        {#each projects as project, i}
            <div class="mb-6">
                <div class="flex gap-1">
                    <p class="text-2xl font-semibold">{project.title}</p>
                    <a class="text-xs mt-3 hover:underline" href={project.link} target="_blank" rel="noopener noreferrer">(link)</a>
                </div>
                <p class="text-gray-700 mt-2">{@html project.description}</p>
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