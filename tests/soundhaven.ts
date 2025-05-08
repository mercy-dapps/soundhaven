import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { BN } from "bn.js";
import { Soundhaven } from "../target/types/soundhaven";

describe("soundhaven", async () => {
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

  const song_sample = {
    song_title: "Rude",
    song_url: "",
    song_thumbnail_url: "",
  };

  const profile = PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), provider.wallet.publicKey.toBuffer()],
    program.programId
  )[0];

  let accounts = {
    user: provider.wallet.publicKey,
    profile,
  };

  it("create profile", async () => {
    let { name, profile_img_avatar, description, is_artist } = profile_sample;
    const tx = await program.methods
      .createProfile(name, profile_img_avatar, description, is_artist)
      .accounts({
        ...accounts,
      })
      .signers([provider.wallet.payer])
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("create song", async () => {
    let { song_title, song_url, song_thumbnail_url } = song_sample;

    const profileAcc = await program.account.profile.fetch(profile);

    const song = PublicKey.findProgramAddressSync(
      [
        Buffer.from("song"),
        provider.wallet.publicKey.toBuffer(),
        new BN(profileAcc.songCount).toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    )[0];

    console.log(song, profileAcc.songCount);

    const tx = await program.methods
      .createSong(song_title, song_url, song_thumbnail_url)
      .accountsPartial({
        user: provider.wallet.publicKey,
        profile,
        song,
        systemProgram: SystemProgram.programId,
      })
      .signers([provider.wallet.payer])
      .rpc();

    console.log("Your transaction signature", tx);

    const songAcc = await program.account.profile.fetch(song);

    console.log("profile created", songAcc);
  });
});
