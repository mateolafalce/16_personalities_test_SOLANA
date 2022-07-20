import * as anchor from "@project-serum/anchor";
const { SystemProgram } = anchor.web3;

describe("Sixteen Personalities Test", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.SixteenPersonalities;
  const answer = anchor.web3.Keypair.generate();


  it("Creating account", async () => {
    const tx = await program.rpc.createQuiz(
      provider.wallet.publicKey,
    {
      accounts: {
        answer: answer.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [answer],
    });
    console.log("Your PDA has been created, check the signature", tx);
  });


  it("Testing your personality", async () => {
    const txe = await program.rpc.initialize(
      provider.wallet.publicKey,
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
      false, //i_often_feel_sad11 ?
      false, //i_like_to_take_charge_of_situations_and_events12 ?
      true, //i_experience_deep_and_varied_emotions13 ?
      false, //i_love_to_help_others14 ?
      true, //i_always_keep_my_promises15 ?
      false, //i_find_it_difficult_to_approach_others16 ?
      true, //i_am_always_busy_always_on_the_go17 ?
      false, //i_prefer_variety_to_routine18 ?
      true, //i_love_a_good_argument_a_good_fight19 ?
      true, //i_work_very_hard20 ?
    {
      accounts: {
        answer: answer.publicKey,
        authority: provider.wallet.publicKey,
      }
    });
    console.log("The result of your personality test is here:", txe);
  });

});
