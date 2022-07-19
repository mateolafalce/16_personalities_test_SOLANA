import * as anchor from "@project-serum/anchor";
const { SystemProgram } = anchor.web3;

describe("SixteenPersonalities", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SixteenPersonalities;
  const answer = anchor.web3.Keypair.generate();

  it("Testing your personality", async () => {
    const tx = await program.rpc.initialize(
      false, //i_worry_about_everything1 ?
      false, //i_make_friends_easily2 ?
      true, //i_have_a_lot_of_imagination3 ?
      false, //i_trust_others4 ?
      true, //i_complete_tasks_successfully5 ?
      false, //i_get_angry_easily6 ?
      false, //i_really_enjoy_large_parties_and_gatherings7 ?
      true, //i_think_at_is_important8 ?
      false, //i_sometimes_deceive_others_to_get_my_own_way9 ?
      true, //i_dont_like_things_to_be_a_mess_i_like_to_tidy_up10 ?
    {
      accounts: {
        answer: answer.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [answer],
    });


    console.log("Your transaction signature", tx);
  });
});
