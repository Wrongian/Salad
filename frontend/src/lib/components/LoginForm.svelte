<script lang="ts">
    import { login } from "../../scripts/queries";
    let username: string = "";
    let password: string = "";
</script>

<div class="form">
    <div>
        <label for="text" class="block text-sm font-medium text-gray-500">Username</label>
        <div>
            <input id="text" type="text" bind:value={username}/>
        </div>
    </div>
    <div>
        <label for="password" class="block text-sm font-medium">Password</label>
        <div>
            <input id="password" type="password" class="rounded" bind:value={password}/>
        </div>
    </div>
    <div>
        <button type="submit" class="flex w-full justify-center rounded-md"
        on:click={() => login(username, password)}></button>
    </div>
</div>