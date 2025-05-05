import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { Soundhaven } from "../target/types/soundhaven";

describe("soundhaven", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.soundhaven as Program<Soundhaven>;

  const profile_sample = {
    name: "Mercy",
    profile_img_avatar:
      "https://api.dicebear.com/9.x/adventurer/svg?seed=mercy",
    description: "Music lover",
    is_artist: false,
  };

  const profile = PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), provider.wallet.publicKey.toBuffer()],
    program.programId
  )[0];

  const accounts = {
    user: provider.wallet.publicKey,
    profile,
  };

  it("Is initialized!", async () => {
    let { name, profile_img_avatar, description, is_artist } = profile_sample;
    const tx = await program.methods
      .createProfile(name, profile_img_avatar, description, is_artist)
      .accounts({
        ...accounts,
      })
      .signers([provider.wallet.payer])
      .rpc();

    console.log("Your transaction signature", tx);

    const profileAcc = await program.account.profile.fetch(profile);

    console.log("profile created", profileAcc);
  });
});
