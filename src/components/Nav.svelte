<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { Link, useLocation } from "svelte-navigator";
  import { signingAccount, isInit, AccountEntry, all_accounts } from "../accounts";
  import { routes } from "../routes";
  import { _ } from "../lang/i18n";
  import { init_preferences } from "../preferences";
  import AboutLink from "./about/AboutLink.svelte";
  import UIkit from "uikit";

  init_preferences();

  let my_account: AccountEntry;
  let unsubsSigningAccount;
  let unsubsAll_accounts;

  const secondaryRoutes = [
    routes.settings,
    routes.about,
    routes.developer,
    routes.keygen,
    routes.accountFromMnem,
  ];

  const location = useLocation();

  let myAccountIsOnChain = false; // assume initialized until not
  let init = false; // assume initialized until not
  onMount(async () => {
    isInit.subscribe((i) => (init = i));

    signingAccount.subscribe((myAccount) => {
      if (myAccount) {
        myAccountIsOnChain = myAccount.on_chain;
      }
    });

    unsubsSigningAccount = signingAccount.subscribe(value => my_account = value);
  });

  onDestroy(() => {
    unsubsSigningAccount && unsubsSigningAccount();
    unsubsAll_accounts && unsubsAll_accounts();
  });

  const close = () => {
    setTimeout(UIkit.offcanvas("#offcanvas-nav").hide, 100)
  }

</script>

<main>
  <div uk-toggle="target: #offcanvas-nav" style="margin-top: 5px;">
    <span uk-icon="icon: menu" />
  </div>

  {#if secondaryRoutes.includes($location.pathname)}
    <Link to={routes.home}
      ><span class="" uk-icon="icon: arrow-left; ratio: 2" /></Link
    >
  {/if}

  <div id="offcanvas-nav" uk-offcanvas="overlay: false; mode: push">

    <div class="uk-offcanvas-bar">
      <ul
        class="uk-nav uk-nav-default 
          {init && myAccountIsOnChain
            ? ''
            : 'uk-invisible'}"
      >
  
        <li on:click={() => close()}><Link to={routes.home}>{$_("nav.wallet")}</Link></li>
        <li on:click={() => close()}>
          <a href={"#"}>
            <Link to="settings" class="uk-text-muted">
              {$_("wallet.account_switcher.setting")}</Link></a>
        </li>
        <li class="uk-nav-divider"></li>
        <li on:click={() => close()}>
          <a href={"#"}>
            <Link to="dev" class="uk-text-muted">
              {$_("wallet.account_switcher.developers")}</Link></a>
        </li>
        <li class="uk-text-muted" on:click={() => close()}>
          <AboutLink />
        </li>
      </ul>
    </div>
  </div>
</main>
