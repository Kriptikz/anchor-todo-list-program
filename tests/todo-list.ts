import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { LAMPORTS_PER_SOL } from '@solana/web3.js';
import { TodoList } from '../target/types/todo_list';
import { assert } from 'chai';

describe('todo-list', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TodoList as Program<TodoList>;

  const airdropAmount = LAMPORTS_PER_SOL;

  const user1 = anchor.web3.Keypair.generate();

  it('Airdrop SOL!', async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user1.publicKey, airdropAmount),
      "confirmed"
    );

    let user1Balance = await provider.connection.getBalance(user1.publicKey);

    assert.equal(airdropAmount, user1Balance);
  });

  it('Initialize List Account for User', async () => {
    // Todo: Implement
  });
});
