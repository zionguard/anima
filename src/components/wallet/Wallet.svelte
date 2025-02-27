<script lang="ts">
  import { _ } from 'svelte-i18n';
  import { onDestroy, onMount } from "svelte";
  import { Link } from "svelte-navigator";
  import {
    isRefreshingAccounts,
    all_accounts,
    signingAccount,
    isAccountsLoaded,
  } from "../../accounts";
  import { routes } from "../../routes";
  import type { AccountEntry } from "../../accounts";
  import Newbie from "./Newbie.svelte";
  import AccountsList from "./AccountsList.svelte";
  import ReminderCreate from "./ReminderCreate.svelte";
  import UIkit from "uikit";
  import Icons from "uikit/dist/js/uikit-icons";
  import { connected } from "../../networks";
  import ConnectionError from "../layout/ConnectionError.svelte";
  import AccountsListSkeleton from './AccountsListSkeleton.svelte';
  UIkit.use(Icons);

  let my_account: AccountEntry;
  let accountList: AccountEntry[] = null;
  let pendingAccounts: AccountEntry[] = [];
  let isRefreshing: boolean = true;
  let isConnected: boolean = true;
  let isLoaded: boolean = false;
  
  let unsubsConnected;
  let unsubsAll_accounts;
  let unsubsSigningAccount;
  let unsubsIsAccountsLoaded;
  let unsubsIsRefreshingAccounts;

  onMount(async () => {
    unsubsConnected = connected.subscribe(b => isConnected = b);
    unsubsAll_accounts = all_accounts.subscribe(all => {
      accountList = all;
      pendingAccounts = all.filter(x => !x.on_chain);
    });
    unsubsSigningAccount = signingAccount.subscribe(a => my_account = a);
    unsubsIsAccountsLoaded = isAccountsLoaded.subscribe(boo => isLoaded = boo);
    unsubsIsRefreshingAccounts = isRefreshingAccounts.subscribe(boo => isRefreshing = boo);   
  });

  onDestroy(async () => {
    unsubsConnected && unsubsConnected();
    unsubsAll_accounts && unsubsAll_accounts();
    unsubsSigningAccount && unsubsSigningAccount();
    unsubsIsAccountsLoaded && unsubsIsAccountsLoaded();
    unsubsIsRefreshingAccounts && unsubsIsRefreshingAccounts();
  });
</script>

<main>
  <div>
    {#if isRefreshing}
      <div style="position:relative">
        <span uk-spinner style="position:absolute; top:0px; left:0px"/>
      </div>
    {/if}

    {#if !isLoaded}
      <AccountsListSkeleton />
    {:else if accountList.length > 0}
      {#if !isConnected}
        <ConnectionError />
      {:else}
        <div uk-grid class="uk-margin uk-flex uk-flex-center">
          <img src="/anima-logo-white.svg" alt="drawing" width="200" style="opacity: 0.5"/>
        </div>       
        <AccountsList list={accountList} />
        <ReminderCreate {pendingAccounts} {isConnected} />

        <div uk-grid class="uk-margin uk-flex uk-flex-center">
          <Link to={routes.keygen}>
            <span uk-icon="icon: plus-circle; ratio: 2"></span>
          </Link>
        </div>
      {/if}
    {:else if accountList.length == 0 && !isRefreshing}
      <Newbie />
    {/if}
  </div>
</main>
