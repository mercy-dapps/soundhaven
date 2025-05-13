import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { randomBytes } from "crypto";
import { BN } from "bn.js";
import { Soundhaven } from "../target/types/soundhaven";

describe("soundhaven", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.soundhaven as Program<Soundhaven>;

  const connection = provider.connection;

  const profile_sample = {
    name: "Mercy",
    profile_img_avatar:
      "https://api.dicebear.com/9.x/adventurer/svg?seed=mercy",
    description: "Music lover",
    is_artist: false,
  };

  const profile_artist_sample = {
    name: "Calvin",
    profile_img_avatar:
      "https://api.dicebear.com/9.x/adventurer/svg?seed=calvin",
    description: "Music artist",
    is_artist: true,
  };

  const song_sample = {
    song_title: "Rude",
    song_url: "",
    song_thumbnail_url: "",
  };

  const playlist_sample = {
    playlist_title: "Love",
    playlist_description: "Love songs",
    playlist_thumbnail_url: "",
    playlist_visibility: true,
  };

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      signature,
      ...block,
    });

    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );

    return signature;
  };

  const [user, artist] = Array.from({ length: 2 }, () => Keypair.generate());

  const user_seed = new BN(randomBytes(8));
  const artist_seed = new BN(randomBytes(8));

  const profile = PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), user.publicKey.toBuffer()],
    program.programId
  )[0];

  const profile_artist = PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), artist.publicKey.toBuffer()],
    program.programId
  )[0];

  const song_id = new BN(randomBytes(8));

  const song = PublicKey.findProgramAddressSync(
    [
      Buffer.from("song"),
      artist.publicKey.toBuffer(),
      song_id.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  )[0];

  const playlist_id = new BN(randomBytes(8));

  const playlist = PublicKey.findProgramAddressSync(
    [
      Buffer.from("playlist"),
      user.publicKey.toBuffer(),
      playlist_id.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  )[0];

  let accounts = {
    user: user.publicKey,
    profile,
    song,
    playlist,
  };

  it("airdrop", async () => {
    let tx = new Transaction();
    tx.instructions = [
      ...[user, artist].map((a) =>
        SystemProgram.transfer({
          fromPubkey: provider.publicKey,
          toPubkey: a.publicKey,
          lamports: 10 * LAMPORTS_PER_SOL,
        })
      ),
    ];

    await provider.sendAndConfirm(tx, []).then(log);
  });

  it("create two profiles - user and artist", async () => {
    // user account here
    let { name, profile_img_avatar, description, is_artist } = profile_sample;
    const tx_user = await program.methods
      .createProfile(
        user_seed,
        name,
        profile_img_avatar,
        description,
        is_artist
      )
      .accounts({
        ...accounts,
      })
      .signers([user])
      .rpc();

    console.log("Your transaction signature", tx_user);
    const profileAcc = await program.account.profile.fetch(profile);

    console.log(profileAcc);

    // artist account here
    const tx_artist = await program.methods
      .createProfile(
        artist_seed,
        profile_artist_sample.name,
        profile_artist_sample.profile_img_avatar,
        profile_artist_sample.description,
        profile_artist_sample.is_artist
      )
      .accounts({
        user: artist.publicKey,
        // profile,
        // song,
        // playlist,
      })
      .signers([artist])
      .rpc();

    console.log("Your transaction signature", tx_artist);
    const profileArtistAcc = await program.account.profile.fetch(
      profile_artist
    );

    console.log(profileArtistAcc);
  });

  it("create song", async () => {
    let { song_title, song_url, song_thumbnail_url } = song_sample;

    const tx = await program.methods
      .createSong(song_id, song_title, song_url, song_thumbnail_url)
      .accounts({
        user: artist.publicKey,
      })
      .signers([artist])
      .rpc();

    console.log("Your transaction signature", tx);

    const songAcc = await program.account.song.fetch(song);

    console.log("song created", songAcc);
  });

  it("create playlist", async () => {
    let {
      playlist_title,
      playlist_description,
      playlist_thumbnail_url,
      playlist_visibility,
    } = playlist_sample;

    const tx = await program.methods
      .createPlaylist(
        playlist_id,
        playlist_title,
        playlist_description,
        playlist_thumbnail_url,
        playlist_visibility
      )
      .accounts({ ...accounts })
      .signers([user])
      .rpc();

    console.log("Your transaction signature", tx);

    const playlistAcc = await program.account.playlist.fetch(playlist);

    console.log("playlist created", playlistAcc);
  });

  // it("follow an artist", async () => {
  //   const artist_key = await program.account.profile.fetch(profile_artist);

  //   console.log(artist_key.profileOwner);

  //   const follow_profile_artist = PublicKey.findProgramAddressSync(
  //     [Buffer.from("profile"), artist_key.profileOwner.toBuffer()],
  //     program.programId
  //   )[0];

  //   console.log(follow_profile_artist);

  //   const tx = await program.methods
  //     .follow(artist_key.profileOwner)
  //     .accountsPartial({
  //       user: user.publicKey,
  //       profile: user.publicKey,
  //       followProfile: follow_profile_artist,
  //     })
  //     // .accounts({...accounts})
  //     .signers([user])
  //     .rpc();

  //   console.log("Your transaction signature", tx);

  //   const artistAcc = await program.account.profile.fetch(profile_artist);

  //   console.log("followed artist", artistAcc);
  // });

  it("like song", async () => {
    const tx = await program.methods
      .like(song)
      .accounts({ ...accounts })
      .signers([user])
      .rpc();

    console.log("Your transaction signature", tx);

    const songAcc = await program.account.song.fetch(song);

    console.log("liked song", songAcc);
  });
});
