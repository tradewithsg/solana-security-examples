import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

/**
 * This test demonstrates a real-world Missing Account Validation exploit:
 *
 * 1. An unauthorized user successfully updates another user's profile
 *    in the vulnerable program.
 * 2. The same attack is rejected in the secure program.
 * 3. Legitimate owners can still update their own profiles.
 *
 * NOTE:
 * `createProfile` is used only to initialize test state.
 * The security example itself focuses on `updateProfile`.
 */
describe("Missing Account Validation", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const vulnerableProgram =
    anchor.workspace.MissingAccountValidation as Program<any>;

  const secureProgram =
    anchor.workspace.MissingAccountValidationSecure as Program<any>;

  const userA = anchor.web3.Keypair.generate(); // Legitimate owner
  const userB = anchor.web3.Keypair.generate(); // Attacker

  let userAProfileVulnerable: anchor.web3.PublicKey;
  let userAProfileSecure: anchor.web3.PublicKey;

  before(async () => {
    // Fund both users
    for (const user of [userA, userB]) {
      const sig = await provider.connection.requestAirdrop(
        user.publicKey,
        2 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(sig);
    }

    // Derive PDAs for both programs
    [userAProfileVulnerable] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("profile"), userA.publicKey.toBuffer()],
        vulnerableProgram.programId
      );

    [userAProfileSecure] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("profile"), userA.publicKey.toBuffer()],
        secureProgram.programId
      );

    // Initialize profile in vulnerable program
    await vulnerableProgram.methods
      .createProfile("Alice")
      .accounts({
        profile: userAProfileVulnerable,
        owner: userA.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userA])
      .rpc();

    // Initialize profile in secure program
    await secureProgram.methods
      .createProfile("Alice")
      .accounts({
        profile: userAProfileSecure,
        owner: userA.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userA])
      .rpc();
  });

  it("❌ allows unauthorized profile update in vulnerable program", async () => {
    // Attacker (User B) updates User A's profile
    await vulnerableProgram.methods
      .updateProfile("HACKED_BY_USER_B")
      .accounts({
        profile: userAProfileVulnerable,
      })
      .signers([userB])
      .rpc();

    // Fetch profile data from vulnerable program
    const profile =
      await vulnerableProgram.account.userProfile.fetch(
        userAProfileVulnerable
      );

    // ❌ This proves the vulnerability
    expect(profile.name).to.equal("HACKED_BY_USER_B");
    console.log("❌ Vulnerable program allowed unauthorized update");
  });

  it("✅ rejects unauthorized profile update in secure program", async () => {
    try {
      // Attacker (User B) attempts the same attack
      await secureProgram.methods
        .updateProfile("SHOULD_FAIL")
        .accounts({
          profile: userAProfileSecure,
          owner: userB.publicKey, // Wrong owner
        })
        .signers([userB])
        .rpc();

      expect.fail("Unauthorized update should have failed");
    } catch (err: any) {
      // ✅ Expected behavior
      expect(err.toString()).to.include("has_one");
      console.log("✅ Secure program correctly rejected unauthorized update");

      // Ensure state was NOT modified
      const profile =
        await secureProgram.account.userProfile.fetch(
          userAProfileSecure
        );

      expect(profile.name).to.equal("Alice");
    }
  });

  it("✅ allows authorized profile update in secure program", async () => {
    // Legitimate owner updates their own profile
    await secureProgram.methods
      .updateProfile("Updated by Owner")
      .accounts({
        profile: userAProfileSecure,
        owner: userA.publicKey,
      })
      .signers([userA])
      .rpc();

    const profile =
      await secureProgram.account.userProfile.fetch(
        userAProfileSecure
      );

    expect(profile.name).to.equal("Updated by Owner");
    console.log("✅ Secure program allowed authorized update");
  });
});
