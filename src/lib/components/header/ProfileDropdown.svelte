<script lang="ts">
  import { goto } from "$app/navigation";
  import Icon from "$lib/components/icons/Icon.svelte";
  import { updaterStore } from "$lib/stores/updater.svelte";

  interface Props {
    isAuthenticated?: boolean;
  }

  let { isAuthenticated = false }: Props = $props();

  let isOpen = $state(false);

  const toggleDropdown = () => {
    isOpen = !isOpen;
  };

  const handleGithubLogin = () => {
    console.log("GitHub OAuth login");
    // TODO: Implement OAuth
    isOpen = false;
  };

  const handleGoogleLogin = () => {
    console.log("Google OAuth login");
    // TODO: Implement OAuth
    isOpen = false;
  };

  const handleSettings = () => {
    goto("/settings");
    isOpen = false;
  };

  const handleLogout = () => {
    console.log("Logout");
    // TODO: Implement logout
    isOpen = false;
  };

  const handleCheckUpdates = async () => {
    await updaterStore.checkForUpdates();
  };

  const handleConfirmUpdate = async () => {
    await updaterStore.downloadAndInstall();
    if (updaterStore.status === "ready") {
      await updaterStore.relaunchApp();
    }
  };

  const handleCancelUpdate = () => {
    updaterStore.cancelUpdate();
  };

  // Close on click outside
  $effect(() => {
    if (!isOpen) return;

    const handleClickOutside = (e: MouseEvent) => {
      const target = e.target as HTMLElement;
      if (!target.closest("[data-profile-dropdown]")) {
        isOpen = false;
      }
    };

    document.addEventListener("click", handleClickOutside);
    return () => document.removeEventListener("click", handleClickOutside);
  });
</script>

<div class="relative" data-profile-dropdown>
  <!-- Trigger button - Circle with user icon -->
  <button
    onclick={toggleDropdown}
    class="size-8 rounded-xl bg-surface border border-border hover:bg-surface-hover transition-colors flex items-center justify-center"
  >
    <Icon name="user" size={16} />
  </button>

  <!-- Dropdown menu -->
  {#if isOpen}
    <div
      class="absolute top-full mt-2 w-32 px-3 bg-surface backdrop-blur-xs border border-border rounded-md shadow-lg py-2 z-50"
    >
      {#if !isAuthenticated}
        <!-- OAuth login section -->
        <div class="flex gap-2">
          <button
            onclick={handleGithubLogin}
            class="flex-1 flex items-center justify-center gap-2 px-3 py-2 rounded-md bg-white/10 hover:bg-surface-hover transition-colors"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              xmlns:xlink="http://www.w3.org/1999/xlink"
              version="1.1"
              id="Capa_1"
              x="0px"
              y="0px"
              viewBox="0 0 24 24"
              style="enable-background:new 0 0 24 24;"
              xml:space="preserve"
              width="20"
              height="20"
              fill="currentColor"
            >
              <g>
                <path
                  style="fill-rule:evenodd;clip-rule:evenodd;"
                  d="M12,0.296c-6.627,0-12,5.372-12,12c0,5.302,3.438,9.8,8.206,11.387   c0.6,0.111,0.82-0.26,0.82-0.577c0-0.286-0.011-1.231-0.016-2.234c-3.338,0.726-4.043-1.416-4.043-1.416   C4.421,18.069,3.635,17.7,3.635,17.7c-1.089-0.745,0.082-0.729,0.082-0.729c1.205,0.085,1.839,1.237,1.839,1.237   c1.07,1.834,2.807,1.304,3.492,0.997C9.156,18.429,9.467,17.9,9.81,17.6c-2.665-0.303-5.467-1.332-5.467-5.93   c0-1.31,0.469-2.381,1.237-3.221C5.455,8.146,5.044,6.926,5.696,5.273c0,0,1.008-0.322,3.301,1.23   C9.954,6.237,10.98,6.104,12,6.099c1.02,0.005,2.047,0.138,3.006,0.404c2.29-1.553,3.297-1.23,3.297-1.23   c0.653,1.653,0.242,2.873,0.118,3.176c0.769,0.84,1.235,1.911,1.235,3.221c0,4.609-2.807,5.624-5.479,5.921   c0.43,0.372,0.814,1.103,0.814,2.222c0,1.606-0.014,2.898-0.014,3.293c0,0.319,0.216,0.694,0.824,0.576   c4.766-1.589,8.2-6.085,8.2-11.385C24,5.669,18.627,0.296,12,0.296z"
                />
                <path
                  d="M4.545,17.526c-0.026,0.06-0.12,0.078-0.206,0.037c-0.087-0.039-0.136-0.121-0.108-0.18   c0.026-0.061,0.12-0.078,0.207-0.037C4.525,17.384,4.575,17.466,4.545,17.526L4.545,17.526z"
                />
                <path
                  d="M5.031,18.068c-0.057,0.053-0.169,0.028-0.245-0.055c-0.079-0.084-0.093-0.196-0.035-0.249   c0.059-0.053,0.167-0.028,0.246,0.056C5.076,17.903,5.091,18.014,5.031,18.068L5.031,18.068z"
                />
                <path
                  d="M5.504,18.759c-0.074,0.051-0.194,0.003-0.268-0.103c-0.074-0.107-0.074-0.235,0.002-0.286   c0.074-0.051,0.193-0.005,0.268,0.101C5.579,18.579,5.579,18.707,5.504,18.759L5.504,18.759z"
                />
                <path
                  d="M6.152,19.427c-0.066,0.073-0.206,0.053-0.308-0.046c-0.105-0.097-0.134-0.234-0.068-0.307   c0.067-0.073,0.208-0.052,0.311,0.046C6.191,19.217,6.222,19.355,6.152,19.427L6.152,19.427z"
                />
                <path
                  d="M7.047,19.814c-0.029,0.094-0.164,0.137-0.3,0.097C6.611,19.87,6.522,19.76,6.55,19.665   c0.028-0.095,0.164-0.139,0.301-0.096C6.986,19.609,7.075,19.719,7.047,19.814L7.047,19.814z"
                />
                <path
                  d="M8.029,19.886c0.003,0.099-0.112,0.181-0.255,0.183c-0.143,0.003-0.26-0.077-0.261-0.174c0-0.1,0.113-0.181,0.256-0.184   C7.912,19.708,8.029,19.788,8.029,19.886L8.029,19.886z"
                />
                <path
                  d="M8.943,19.731c0.017,0.096-0.082,0.196-0.224,0.222c-0.139,0.026-0.268-0.034-0.286-0.13   c-0.017-0.099,0.084-0.198,0.223-0.224C8.797,19.574,8.925,19.632,8.943,19.731L8.943,19.731z"
                />
              </g>
            </svg>

            <!-- <span class="text-sm">GitHub</span> -->
          </button>
          <button
            onclick={handleGoogleLogin}
            class="flex-1 flex items-center justify-center gap-2 px-3 py-2 rounded-md bg-white/10 hover:bg-surface-hover transition-colors"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              xmlns:xlink="http://www.w3.org/1999/xlink"
              version="1.1"
              id="Capa_1"
              x="0px"
              y="0px"
              viewBox="0 0 24 24"
              style="enable-background:new 0 0 24 24;"
              xml:space="preserve"
              width="20"
              height="20"
              fill="currentColor"
            >
              <g>
                <path
                  d="M12.479,14.265v-3.279h11.049c0.108,0.571,0.164,1.247,0.164,1.979c0,2.46-0.672,5.502-2.84,7.669   C18.744,22.829,16.051,24,12.483,24C5.869,24,0.308,18.613,0.308,12S5.869,0,12.483,0c3.659,0,6.265,1.436,8.223,3.307L18.392,5.62   c-1.404-1.317-3.307-2.341-5.913-2.341C7.65,3.279,3.873,7.171,3.873,12s3.777,8.721,8.606,8.721c3.132,0,4.916-1.258,6.059-2.401   c0.927-0.927,1.537-2.251,1.777-4.059L12.479,14.265z"
                />
              </g>
            </svg>
            <!-- <span class="text-sm">Google</span> -->
          </button>
        </div>

        <div class="h-px bg-border my-2"></div>
      {/if}

      <!-- Update Section -->
      <div class="mb-2">
        {#if updaterStore.status === "idle" || updaterStore.status === "not-available" || updaterStore.status === "error"}
          <button
            onclick={handleCheckUpdates}
            class="flex w-full items-center gap-3 px-2 py-1.5 rounded-md text-sm bg-white/10 hover:bg-surface-hover transition-colors text-left"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="1.5em"
              height="1.5em"
              viewBox="0 0 24 24"
              ><path
                fill="currentColor"
                d="M12 21q-1.875 0-3.512-.712t-2.85-1.925t-1.925-2.85T3 12t.713-3.512t1.924-2.85t2.85-1.925T12 3q2.05 0 3.888.875T19 6.35V4h2v6h-6V8h2.75q-1.025-1.4-2.525-2.2T12 5Q9.075 5 7.038 7.038T5 12t2.038 4.963T12 19q2.625 0 4.588-1.7T18.9 13h2.05q-.375 3.425-2.937 5.713T12 21m2.8-4.8L11 12.4V7h2v4.6l3.2 3.2z"
              /></svg
            >
            <span>Update</span>
          </button>
          {#if updaterStore.status === "not-available"}
            <p class="text-xs text-text-muted px-2 mt-1">Already up to date</p>
          {/if}
          {#if updaterStore.status === "error" && updaterStore.error}
            <div class="flex items-start gap-1 px-2 mt-1">
              <p class="text-xs text-danger flex-1">{updaterStore.error}</p>
              <button
                onclick={() => updaterStore.clearError()}
                class="size-4 flex items-center justify-center rounded hover:bg-surface-hover transition-colors text-text-muted hover:text-danger"
              >
                <Icon name="x" size={10} />
              </button>
            </div>
          {/if}
        {:else if updaterStore.status === "checking"}
          <div
            class="flex items-center gap-3 px-2 py-1.5 text-sm text-text-muted"
          >
            <div
              class="size-4 border-2 border-primary border-t-transparent rounded-full animate-spin"
            ></div>
            <span>Checking...</span>
          </div>
        {:else if updaterStore.status === "available"}
          <div class="space-y-2">
            <p class="text-xs text-text-muted px-2">
              v{updaterStore.newVersion} available
            </p>
            <div class="flex gap-2 px-2">
              <button
                onclick={handleConfirmUpdate}
                class="flex-1 flex items-center justify-center gap-1 px-2 py-1.5 rounded-md bg-primary/20 text-green-400/60 text-sm hover:bg-primary/30 transition-colors"
              >
                <Icon name="check" size={14} />
              </button>
              <button
                onclick={handleCancelUpdate}
                class="flex items-center justify-center px-2 py-1.5 rounded-md bg-white/10 text-text-muted text-sm hover:bg-surface-hover transition-colors"
              >
                <Icon name="x" size={14} />
              </button>
            </div>
          </div>
        {:else if updaterStore.status === "downloading"}
          <div class="px-2 py-1.5 space-y-2">
            <div class="flex items-center gap-2 text-sm text-text-muted">
              <div
                class="size-4 border-2 border-primary border-t-transparent rounded-full animate-spin"
              ></div>
              <span>Downloading...</span>
            </div>
            <div class="h-1 bg-surface-hover rounded-full overflow-hidden">
              <div
                class="h-full bg-primary transition-all duration-300"
                style="width: {Math.min(updaterStore.downloadProgress, 100)}%"
              ></div>
            </div>
          </div>
        {:else if updaterStore.status === "ready"}
          <button
            onclick={() => updaterStore.relaunchApp()}
            class="flex w-full items-center gap-3 px-2 py-1.5 rounded-md text-sm bg-primary/20 text-primary hover:bg-primary/30 transition-colors text-left"
          >
            <Icon name="file" size={16} />
            <span>Restart to Update</span>
          </button>
        {/if}
      </div>

      <div class="h-px bg-border my-2"></div>

      <!-- Settings -->
      <button
        onclick={handleSettings}
        class=" flex w-full items-center gap-3 px-2 py-1.5 bg-white/10 rounded-md text-sm hover:bg-surface-hover transition-colors text-left"
      >
        <Icon name="settings" size={16} />
        <span>Settings</span>
      </button>

      {#if isAuthenticated}
        <!-- Logout -->
        <div class="h-px bg-border my-2"></div>
        <button
          onclick={handleLogout}
          class="w-full flex items-center gap-3 px-4 py-2 text-sm hover:bg-surface-hover transition-colors text-left text-danger"
        >
          <Icon name="logOut" size={16} />
          <span>Log out</span>
        </button>
      {/if}
    </div>
  {/if}
</div>
