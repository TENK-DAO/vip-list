
import { Workspace } from "near-willem-workspaces-ava";


const workspace = Workspace.init(async ({ root }) => {
  // Create a subaccount of the root account, like `alice.sandbox`
  // (the actual account name is not guaranteed; you can get it with `alice.accountId`)
  const alice = await root.createAccount("alice");

  // Create a subaccount of the root account, and also deploy a contract to it
  const contract = await root.createAndDeploy(
    // Subaccount name
    "vip-list",

    // Relative path (from package.json location) to the compiled contract file
    // which will be deployed to this account
    "target/wasm32-unknown-unknown/release/vip_list.wasm",

    // Provide `method` and `args` to call in the same transaction as the deploy
    {
      method: "new",
      args: { owner_id: root },
    }
  );

  // Return the accounts that you want available in subsequent tests
  // (`root` is always available)
  return { alice, contract };
});

workspace.test(
  "can add to vip list",
  async (test, { contract, root, alice }) => {
    // Don't forget to `await` your calls!
    await root.call(contract, "add_list", { account_id: root });

    await root.call(contract, "add_to_list", {
      account_id: alice,
      allowance: "42",
    });

    test.is("42",
      await contract.view("vip_allowance", { account_id: alice, list_id: root })
    );
  }
);
