import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";

import * as anchor from "@project-serum/anchor";

// Get the PDA that is assigned authority to token account.
let pda: PublicKey = null;
const [_pda, _nonce] = await PublicKey.findProgramAddress(
  [Buffer.from(anchor.utils.bytes.utf8.encode("escrow"))],
  pg.program.programId
);

pda = _pda;

console.log("Program address", pda.toString());
