# Testing Scenarios

In the scenarios, for simplicity, we call the chains gold (KSM token) and diamond (DOT token),
but they should be applicable to any other chains. The first scenario has detailed description about
the entire process (also see the [sequence diagram](./scenario1.html)). Other scenarios only contain
a simplified interaction focusing on things that are unique for that particular scenario.

Notation:
- kX - user X interacting with gold chain.
- `k(kX)` - gold account id of user kX (native account id; usable on gold)
- `p(kX)` - diamond account id of user kX (account id derived from `k(kX)` usable on diamond)
- [gold] ... - Interaction happens on gold (e.g. the user interacts with gold chain)
- [diamond] ... - Interaction happens on diamond

Basic Scenarios
===========================

Scenario 1: gold's Alice receiving & spending DOTs
---------------------------

gold's Alice (kAlice) receives 5 DOTs from diamond's Bob (pBob) and sends half of them to
kCharlie.

1. Generate kAlice's DOT address (`p(kAlice)`).
   See function:

   ```rust
   bp_runtime::derive_account_id(b"pdot", kAlice)
   ```

   or:

   ```rust
   let hash = bp_diamond::derive_gold_account_id(kAlice);
   let p_kAlice = bp_diamond::AccountIdConverter::convert(hash);
   ```

2. [diamond] pBob transfers 5 DOTs to `p(kAlice)`
   1. Creates & Signs a transaction with `Call::Transfer(..)`
   1. It is included in block.
   1. kAlice observers diamond chain to see her balance at `p(kAlice)` updated.

3. [gold] kAlice sends 2.5 DOTs to `p(kCharlie)`
   1. kAlice prepares:
      ```rust
        let call = diamond::Call::Balances(diamond::Balances::Transfer(p(kCharlie), 2.5DOT)).encode();
        let weight = call.get_dispatch_info().weight;
      ```

   1. kAlice prepares gold transaction:
      ```rust
      gold::Call::Messages::<Instance=diamond>::send_message(
        // dot-transfer-lane (truncated to 4bytes)
        lane_id,
        payload: MessagePayload {
          // Get from current diamond runtime (kind of hardcoded)
          spec_version: 1,
          // kAlice should know the exact dispatch weight of the call on the target
          // source verifies: at least to cover call.length() and below max weight
          weight,
          // simply bytes, we don't know anything about that on the source chain
          call,
          // origin that should be used during dispatch on the target chain
          origin: CallOrigin::SourceAccount(kAlice),
        },
        delivery_and_dispatch_fee: {
          (single_message_delivery_weight
            // source weight = X * target weight
            + convert_target_weight_to_source_weight(weight)
            + confirmation_transaction_weight
          )
          // This uses an on-chain oracle to convert weights of the target chain to source fee
          * weight_to_fee
          // additional reward for the relayer (pallet parameter)
          + relayers_fee
        },
      )
      ```

   1. [gold] kAlice sends gold transaction with the above `Call` and pays regular fees. The
      dispatch additionally reservers target-chain delivery and dispatch fees (including relayer's
      reward).

4. [gold] kAlice's transaction is included in block `B1`

### Syncing headers loop

5. Relayer sees that `B1` has not yet been delivered to the target chain.
   [Sync loop code](https://github.com/paritytech/parity-bridges-common/blob/8b327a94595c4a6fae6d7866e24ecf2390501e32/relays/headers-relay/src/sync_loop.rs#L199).

1. Relayer prepares transaction which delivers `B1` and with all of the missing
   ancestors to the target chain (one header per transaction).

1. After the transaction is succesfully dispatched the diamond on-chain light client of the gold
   chain learns about block `B1` - it is stored in the on-chain storage.

### Syncing finality loop

8. Relayer is subscribed to finality events on gold. Relayer gets a finality notification for
   block `B3`.

1. The header sync informs the target chain about `B1..B3` blocks (see point 6).

1. Relayer learns about missing finalization of `B1..B3` on the target chain, see
   [finality maintenance code](https://github.com/paritytech/parity-bridges-common/blob/8b327a94595c4a6fae6d7866e24ecf2390501e32/relays/substrate/src/headers_maintain.rs#L107).

1. Relayer submits justification for `B3` to the target chain (`finalize_header`).
    See [#421](https://github.com/paritytech/parity-bridges-common/issues/421) for multiple
    authority set changes support in Relayer (i.e. what block the target chain expects, not only
    what I have).

    Relayer is doing two things:
    - syncing on demand (what blocks miss finality)
    - and syncing as notifications are received (recently finalized on-chain)

1. Eventually diamond on-chain light client of gold learns about finality of `B1`.

### Syncing messages loop

13. The relayer checks the on-chain storage (last finalized header on the source, best header on the
    target):
    - gold outbound lane
    - diamond inbound lane
    Lanes contains `latest_generated_nonce` and `latest_received_nonce` respectively. The relayer
    syncs messages between that range.

1. The relayer gets a proof for every message in that range (using the RPC of messages module)

1. The relayer creates a message delivery transaction (but it has weight, size, and count limits).
    The count limit is there to make the loop of delivery code bounded.
     ```rust
     receive_message_proof(
        relayer_id,     // account id of the source chain
        proof,          // messages + proofs (hash of source block `B1`, nonces, lane_id + storage proof)
        dispatch_weight // relayer declares how much it will take to dispatch all messages in that transaction,
      )
     ```
    The `proof` can also contain an update of outbound lane state of source chain, which indicates
    the delivery confirmation of these messages and reward payment, so that the target chain can
    truncate its unpayed rewards vector.

    The target chain stores `relayer_ids` that delivered messages because the relayer can generate
    a storage proof to show that they did indeed deliver those messages. The reward is paid on the
    source chain and we inform the target chain about that fact so it can prune these `relayer_ids`.

    It's totally fine if there are no messages, and we only include the reward payment proof
    when calling that function.

1. 🥳 the message is now delivered and dispatched on the target chain!

1. The relayer now needs to confirm the delivery to claim her payment and reward on the source
    chain.

1. The relayer creates a transaction on the source chain with call:

  ```rust
  receive_messages_delivery_proof(
    proof, // hash of the finalized target chain block, lane_id, storage proof
  )
  ```

### UI challenges

- The UI should warn before (or prevent) sending to `k(kCharlie)`!


Scenario 2: gold's Alice nominating validators with her DOTs
---------------------------

kAlice receives 10 DOTs from pBob and nominates `p(pCharlie)` and `p(pDave)`.

1. Generate kAlice's DOT address (`p(kAlice)`)
2. [diamond] pBob transfers 5 DOTs to `p(kAlice)`
3. [gold] kAlice sends a batch transaction:
  - `staking::Bond` transaction to create stash account choosing `p(kAlice)` as the controller account.
  - `staking::Nominate(vec![p(pCharlie)])` to nominate pCharlie using the controller account.


Scenario 3: gold Treasury receiving & spending DOTs
---------------------------

pBob sends 15 DOTs to gold Treasury which gold Governance decides to transfer to kCharlie.

1. Generate source account for the treasury (`kTreasury`).
2. [diamond] pBob tarnsfers 15 DOTs to `p(kTreasury)`.
2. [gold] Send a governance proposal to send a bridge message which transfers funds to `p(kCharlie)`.
3. [gold] Dispatch the governance proposal using `kTreasury` account id.

Extra scenarios
===========================

Scenario 4: gold's Alice setting up 1-of-2 multi-sig to spend from either gold or diamond
---------------------------

Assuming `p(pAlice)` has at least 7 DOTs already.

1. Generate multisig account id: `pMultiSig = multi_account_id(&[p(kAlice), p(pAlice)], 1)`.
2. [gold] Transfer 7 DOTs to `pMultiSig` using `TargetAccount` origin of `pAlice`.
3. [gold] Transfer 2 DOTs to `p(kAlice)` from the multisig:
   - Send `multisig::as_multi_threshold_1(vec![p(pAlice)], balances::Transfer(p(kAlice), 2))`

Scenario 5: gold Treasury staking & nominating validators with DOTs
---------------------------

Scenario 6: gold Treasury voting in diamond's democracy proposal
---------------------------

Potentially interesting scenarios
===========================

Scenario 7: diamond's Bob spending his DOTs by using gold chain
---------------------------

We can assume he holds KSM. Problem: he can pay fees, but can't really send (sign) a transaction?
Shall we support some kind of dispatcher?

Scenario 8: gold Governance taking over gold's Alice DOT holdings
---------------------------

We use `SourceRoot` call to transfer her's DOTs to gold treasury. Source chain root
should also be able to send messages as `CallOrigin::SourceAccount(Alice)` though.
