<script>
	let { children } = $props();
	import "../app.css";

    let contact_text = $state("Contact");
</script>

<div class="absolute left-15 top-7 text-[#2D3441] flex gap-7">
	<a class="hover:cursor-pointer fancy-link" href="/">Home</a>
</div>
<div class="absolute right-15 top-7 text-[#2D3441] flex gap-7">
	<a class="hover:cursor-pointer fancy-link" href="/projects">Projects</a>
	<a class="hover:cursor-pointer fancy-link" href="/about">About</a>
	<a class="hover:cursor-pointer fancy-link" href="/contact" onpointerleave={() => contact_text="Contact"} onpointerenter={() => contact_text="Get in touch!"}>{contact_text}</a>
</div>

<style>
    * {
        font-family: "JetBrains Mono";
        scrollbar-width: none;
        -ms-overflow-style: none;
    }
    .fancy-link {
        position: relative;
        text-decoration: none;
        color: #333;
        font-weight: 500;
    }

    .fancy-link::after {
        content: "";
        position: absolute;
        left: 50%;
        bottom: 0;
        width: 0;
        height: 2px;
        background-color: currentColor;
        transition: all 0.3s ease;
        transform: translateX(-50%);
    }

    .fancy-link:hover::after {
        width: 100%;
    }
    
    *::-webkit-scrollbar {
        display: none;
    }

</style>

{@render children?.()}
