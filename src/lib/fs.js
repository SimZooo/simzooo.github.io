import file_system from "$lib/fs.json"

export function get_home_dir(user) {
    let users = file_system.metadata.users;
    for (const curr_user of users) {
        if (curr_user.user === user) {
            return curr_user.home;
        }
    }
    return "/";
}

export function check_dir_exists(file_path) {
    let curr_path = file_system.system;
    let parts = file_path.split("/").filter((dir) => dir !== "");

    for (const part of parts) {
        curr_path = curr_path.children.find((child) => child.name === part);
        if (!curr_path) return false;
    }

    return true;
}

export function change_dir(file_path, curr_dir) {
    let parts = file_path.split("/");
    let new_path = curr_dir.split("/").filter((dir) => dir !== "");
    console.log(new_path);

    for (const part of parts) {
        if (part === "..") {
            new_path.pop();
            console.log(new_path)
        } else {
            new_path.push(part);
        }
    }

    new_path = new_path.filter((dir) => dir !== "");
    return "/" + new_path.join("/");
}

export function get_dir(dir) {
    let parts = dir.split("/").filter(Boolean);
    let curr_path = file_system.system;

    for (const part of parts) {
        if (curr_path.type !== "directory") {
            return undefined;
        };

        let next = curr_path.children.find(child => child.name === part);
        curr_path = next;
    }

    return curr_path;
}

export function get_full_path(dir) {
}

export function get_file(dir) {
    let parts = dir.split("/").filter(Boolean);
    let path = get_dir((parts.slice(0, -1)).join("/"));

    let file = path.children.find(child => child.name === parts.slice(-1)[0] && child.type === "file");
    return file;
}

export function list_dir(dir) {
    let dir_children = get_dir(dir);
    let text = `  Total: ${dir_children.children.length}`;

    for (const child of dir_children.children) {
        text += `\n  ${child.name}`;
    }

    return text;
}

export function get_file_system() {
    return file_system;
}