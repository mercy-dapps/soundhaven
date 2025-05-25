import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
  createAssociatedTokenAccountIdempotentInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  getMinimumBalanceForRentExemptMint,
  MINT_SIZE,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
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

  // variables declarations

  const user_seed = new BN(randomBytes(8));
  const artist_seed = new BN(randomBytes(8));
  const config_seed = new BN(randomBytes(8));
  const song_id = new BN(randomBytes(8));
  const playlist_id = new BN(randomBytes(8));

  const tokenProgram = TOKEN_PROGRAM_ID;

  const profile_sample = {
    name: "Mercy",
    profile_img_avatar:
      "https://api.dicebear.com/9.x/adventurer/svg?seed=mercy",
    description: "Music lover",
    is_artist: false,
  };

  const update_profile_sample = {
    name: "MercyDapp",
    profile_img_avatar:
      "https://api.dicebear.com/9.x/adventurer/svg?seed=mercydapp",
    description: "Builder that loves music",
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

  const [user, artist, admin, mintShn] = Array.from({ length: 4 }, () =>
    Keypair.generate()
  );

  // const userAtaSHN = getAssociatedTokenAddressSync(
  //   mintShn.publicKey,
  //   user.publicKey,
  //   false,
  //   tokenProgram
  // );

  // const adminAtaSHN = getAssociatedTokenAddressSync(
  //   mintShn.publicKey,
  //   admin.publicKey,
  //   false,
  //   tokenProgram
  // );

  const [userAtaSHN, adminAtaSHN] = [user, admin]
    .map((a) =>
        getAssociatedTokenAddressSync(
          mintShn.publicKey,
          a.publicKey,
          false,
          tokenProgram
        )
      )
    .flat();

  const vaultState = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state"), admin.publicKey.toBuffer()],
    program.programId
  )[0];

  const vault = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vaultState.toBytes()],
    program.programId
  )[0];

  const config = PublicKey.findProgramAddressSync(
    [Buffer.from("config"), config_seed.toArrayLike(Buffer, "le", 8)],
    program.programId
  )[0];

  const profile = PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), user.publicKey.toBuffer()],
    program.programId
  )[0];

  const profile_artist = PublicKey.findProgramAddressSync(
    [Buffer.from("profile"), artist.publicKey.toBuffer()],
    program.programId
  )[0];

  const song = PublicKey.findProgramAddressSync(
    [
      Buffer.from("song"),
      artist.publicKey.toBuffer(),
      song_id.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  )[0];

  const playlist = PublicKey.findProgramAddressSync(
    [
      Buffer.from("playlist"),
      user.publicKey.toBuffer(),
      playlist_id.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  )[0];

  const vaultToken = getAssociatedTokenAddressSync(
    mintShn.publicKey,
    config,
    true
  );

  let accounts = {
    admin: admin.publicKey,
    user: user.publicKey,
    mintShn: mintShn.publicKey,
    profile,
    song,
    playlist,
    vault,
    vaultState,
    vaultToken,
    userAtaSHN,
    adminAtaSHN,
    config,
    tokenProgram,
  };

  it("airdrop", async () => {
    let lamports = await getMinimumBalanceForRentExemptMint(connection);

    let tx = new Transaction();
    tx.instructions = [
      ...[user, artist, admin].map((a) =>
        SystemProgram.transfer({
          fromPubkey: provider.publicKey,
          toPubkey: a.publicKey,
          lamports: 10 * LAMPORTS_PER_SOL,
        })
      ),

      SystemProgram.createAccount({
        fromPubkey: provider.publicKey,
        newAccountPubkey: mintShn.publicKey,
        lamports,
        space: MINT_SIZE,
        programId: tokenProgram,
      }),

      ...[
        { mint: mintShn.publicKey, authority: admin.publicKey, ata: adminAtaSHN },
      ].flatMap((x) => [
        createInitializeMint2Instruction(
          x.mint,
          6,
          x.authority,
          null,
          tokenProgram
        ),
        createAssociatedTokenAccountIdempotentInstruction(
          provider.publicKey,
          x.ata,
          x.authority,
          x.mint,
          tokenProgram
        ),
        createMintToInstruction(
          x.mint,
          x.ata,
          x.authority,
          1e9,
          undefined,
          tokenProgram
        ),
      ]),
    ];

    await provider.sendAndConfirm(tx, [admin, mintShn]).then(log);
  });

  it("create two profiles - user and artist", async () => {
    // user account here
    let { name, profile_img_avatar, description, is_artist } = profile_sample;
    await program.methods
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
      .rpc()
      .then(confirm)
      .then(log);

    // artist account here
    await program.methods
      .createProfile(
        artist_seed,
        profile_artist_sample.name,
        profile_artist_sample.profile_img_avatar,
        profile_artist_sample.description,
        profile_artist_sample.is_artist
      )
      .accounts({
        user: artist.publicKey,
      })
      .signers([artist])
      .rpc()
      .then(confirm)
      .then(log);
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
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("initialize_vault", async () => {
    await program.methods
      .initializeVault()
      .accounts({ ...accounts })
      .signers([admin])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("initialize_token_vault", async () => {
    await program.methods
      .initializeTokenVault(config_seed)
      .accounts({ ...accounts })
      .signers([admin])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("update profile", async () => {
    let { name, profile_img_avatar, description } = update_profile_sample;
    const tx_user = await program.methods
      .updateProfile(user_seed, name, profile_img_avatar, description)
      .accounts({
        ...accounts,
      })
      .signers([user])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("pay to create song", async () => {
    await program.methods
      .pay()
      .accountsPartial({
        admin: admin.publicKey,
        user: artist.publicKey,
        vaultState,
        vault,
      })
      .signers([artist])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("create song", async () => {
    let { song_title, song_url, song_thumbnail_url } = song_sample;

    const tx = await program.methods
      .createSong(song_id, song_title, song_url, song_thumbnail_url)
      .accounts({ user: artist.publicKey })
      .signers([artist])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("delete playlist", async () => {
    let userAccount = await program.account.profile.fetch(profile);

    console.log(userAccount);
    let playlistAccount = await program.account.playlist.fetch(playlist);

    await program.methods
      .deletePlaylist(playlistAccount.playlistId)
      .accountsPartial({
        user: user.publicKey,
        playlist,
        profile,
      })
      .signers([user])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("claim reward - token", async () => {
    await program.methods
      .claim(new BN(1))
      .accounts({ ...accounts })
      .signers([admin, user])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("withdraw token", async () => {
    await program.methods
      .withdrawFund(new BN(1))
      .accounts({ ...accounts })
      .signers([user])
      .rpc()
      .then(confirm)
      .then(log);
  });

  // it("follow an artist", async () => {
  //   const artistAccount = await program.account.profile.fetch(profile_artist);
  //   const userAccount = await program.account.profile.fetch(profile);

  //   const tx = await program.methods
  //     .follow(userAccount.profileOwner, artistAccount.profileOwner)
  //     .accountsPartial({
  //       user: user.publicKey,
  //       profile,
  //       followProfile: profile_artist,
  //     })
  //     .signers([user])
  //     .rpc();

  //   console.log("Your transaction signature", tx);

  //   const artistAcc = await program.account.profile.fetch(profile_artist);

  //   console.log("followed artist", artistAcc);
  // });

  it("like song", async () => {
    let songAccount = await program.account.song.fetch(song);

    const tx = await program.methods
      .like(songAccount.songId, songAccount.songOwner)
      .accounts({ ...accounts })
      .signers([user])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("delete user", async () => {
    await program.methods
      .deleteProfile()
      .accounts({ ...accounts })
      .signers([user])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("delete song", async () => {
    await program.methods
      .deleteSong(song_id)
      .accounts({
        user: artist.publicKey,
      })
      .signers([artist])
      .rpc()
      .then(confirm)
      .then(log);
      
  });
});
