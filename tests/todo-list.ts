import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { TodoList } from '../target/types/todo_list';

describe('todo-list', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.TodoList as Program<TodoList>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
