<script lang="ts">
  import { Link } from "svelte-navigator";
  import type { WalletError } from "../../walletError";
  import { raise_error } from "../../walletError";
  import { network_profile } from "../../networks";
  import type { NetworkProfile} from "../../networks";
  import { invoke } from "@tauri-apps/api/tauri";
  import { routes } from "../../routes";
  import { notify_success } from "../../walletNotify";
  import { _ } from "svelte-i18n";

  // default playlist which is provided in TauriWallet.
  let playlist_json_url = "https://raw.githubusercontent.com/metazoa-labs/anima/seed_peers/main/fullnode_seed_playlist.json";

  function updateNetwork() {
    // check input data
    // submit
    invoke("override_playlist", { url: playlist_json_url })
      .then((res: NetworkProfile) => {
        network_profile.set(res);
        notify_success("Network Settings Updated");

      })
      .catch((error) => {
        raise_error(error as WalletError, false, "updateNetwork");
      });
  }

</script>

<main>
  <form id="account-form">
    <fieldset class="uk-fieldset">
      <div class="uk-margin uk-inline-block uk-width-1-1">
        <span> {$_("settings.network_settings.playlist")} </span>
        <input
          class="uk-input"
          type="text"
          placeholder={playlist_json_url}
          bind:value={playlist_json_url}
        />
      </div>

      <div>
        <span
          on:click={updateNetwork}
          class="uk-button uk-button-primary uk-align-right"
          id="add-btn">{$_("settings.network_settings.btn_submit")}</span
        >
        <Link to={routes.home}>
          <span class="uk-button uk-button-default uk-align-right">{$_("settings.network_settings.btn_cancel")}</span>
        </Link>
      </div>
    </fieldset>
  </form>
</main>
