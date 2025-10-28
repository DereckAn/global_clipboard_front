<script lang="ts">
  import { goto } from "$app/navigation";
  import Icon from "$lib/components/icons/Icon.svelte";

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
    class="w-10 h-10 rounded-full bg-surface border border-border hover:bg-surface-hover transition-colors flex items-center justify-center"
  >
    <Icon name="user" size={20} />
  </button>

  <!-- Dropdown menu -->
  {#if isOpen}
    <div
      class="absolute top-full right-0 mt-2 w-64 bg-surface border border-border rounded-md shadow-lg py-2 z-50"
    >
      {#if !isAuthenticated}
        <!-- OAuth login section -->
        <div class="px-4 py-2">
          <p class="text-xs text-text-muted mb-3">Sign in with</p>
          <div class="flex gap-2">
            <button
              onclick={handleGithubLogin}
              class="flex-1 flex items-center justify-center gap-2 px-3 py-2 rounded-md border border-border hover:bg-surface-hover transition-colors"
            >
              <Icon name="github" size={18} />
              <span class="text-sm">GitHub</span>
            </button>
            <button
              onclick={handleGoogleLogin}
              class="flex-1 flex items-center justify-center gap-2 px-3 py-2 rounded-md border border-border hover:bg-surface-hover transition-colors"
            >
              <Icon name="google" size={18} />
              <span class="text-sm">Google</span>
            </button>
          </div>
        </div>

        <div class="h-px bg-border my-2"></div>
      {/if}

      <!-- Settings -->
      <button
        onclick={handleSettings}
        class="w-full flex items-center gap-3 px-4 py-2 text-sm hover:bg-surface-hover transition-colors text-left"
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
