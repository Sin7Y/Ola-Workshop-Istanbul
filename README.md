# Ola-workshop-Istanbul
The tutorial provides a detailed guide on completing Workshop Challenge #1 hosted by Ola at the ZK Hack in Istanbul. 
You will set up the development environment for the Ola contract, complete the voting contract logic, execute the contract, and generate and verify proofs.

# Resources You May Be Interested In
Here are some resources that can help to better understand Ola and quickly become an Ola developer.
- [Ola HomePage](https://home.olavm.org)
- [Whitepaper](https://olavm.org/)
- [OLA 101](https://www.youtube.com/watch?v=kaCiHQ6sM9A&list=PL-4FcjL--esquu-7mVq2dqJ3PyBPR-m5w)
- [Sin7y Research Articles](https://hackmd.io/@sin7y)
- [Ola General Updates](https://medium.com/@ola_zkzkvm)

# Workshop Challenge Tutorial
## Setting Up the Ola Development Environment
Create a new folder named ola_workshop, and within it, download the pre-compiled executables [here](https://github.com/Sin7Y/Ola-Workshop-Istanbul/tree/main/tool-chain) that you will need, such as ola and toy_abi, according to your different CPU architectures. Additionally, download the [script](https://github.com/Sin7Y/Ola-Workshop-Istanbul/blob/main/olaws.sh) that encapsulate various operations and place them all in the ola_workshop directory.

Ensure that the script has the permission to be executed：
````shell
chmod +x olaws.sh
chmod +x olac
chmod +x ola
````

This is a voting contract with some missing logic that needs to be completed:
````js
contract Voting {
 
    struct Proposal {
        u32 name;  
        u32 voteCount;
    }


    struct Voter {
        bool voted;  
        u32 vote;   
    }

    mapping(address => Voter) voters;

    Proposal[] proposals;

    // constructor
    fn contract_init(u32[] proposalNames_) {
       for (u32 i = 0; i < proposalNames_.length; i++) {
        // TODO: Create a new proposal and save it to the array of proposals.
        // TODO: Find the bug in contract initialization.
            proposals.push(Proposal({
                name: proposalNames_[i],
                voteCount: i
            }));
            print(proposals[i].name);
        }
    }

    fn vote_proposal(u32 proposal_)  {
        address msgSender = caller_address();
        Voter storage sender = voters[msgSender];
        // TODO: Check if the sender has already voted.
        sender.voted = true;
        sender.vote = proposal_;
        print(proposals[proposal_].name);
        assert(proposals[proposal_].name != 0, "Vote is not initialized");
        // TODO: Use the correct operator to increment voteCount.
    }


    fn winningProposal() -> (u32 winningProposal_) {
        u32 winningVoteCount = 0;
        for (u32 p = 0; p < proposals.length; p++) {
            if (proposals[p].voteCount > winningVoteCount) {
                winningVoteCount = proposals[p].voteCount;
                // TODO: Assign the correct value to winningProposal_.
            }
        }
        print(winningProposal_);
        return winningProposal_;
    }

     fn getWinnerName() -> (u32) {
        u32 winnerP = winningProposal();
        // TODO: Ensure this is accessing the correct property for the winner's name.
        print(winnerName);
        return winnerName;
     }

}

````
You can either copy it as vote.ola or download it directly from [here](https://github.com/Sin7Y/Ola-Workshop-Istanbul/blob/main/vote.ola). 

__*Attention*__
Ensure that `vote.ola` is in the same directory as `ola`, `olaws.sh`, and `toy_abi`, remember, the `ola`, `olac`, `toy_abi` are under tool-chain directory, you can choose one by your OS.

For example, mkdir a directory named `foo`, and here is the foo looks like:
```shell
%cd foo
%ls
ola    olac    olaws.sh    toy_abi    vote.ola
```

Then you can do the rest in the foo directory.

Use your favorite editor to fill in the missing logic of the contract. It is recommended to use VSCode; we have provided a VSCode plugin for Ola-lang, which you can find [here](https://marketplace.visualstudio.com/items?itemName=Sin7y.ola).


Compile the smart contract:
````shell
./olaws.sh compile vote
````
This command will create a directory named target in the current directory, compile `vote.ola`, and generate two files: `vote_asm.json` and `vote_abi.json`.

Prepare the execution parameters for contract_init:
````shell
./olaws.sh encode -f "contract_init(u32[])" -a "[1,2,3]"
````
You can see the ABI-encoded input parameters in the inputs directory.


Invoke the contract and execute the contract_init method:
````shell
./olaws.sh execute contract_init
````
This command will start olavm and pass the previously compiled contract `target/vote_asm.json` and the input parameters `inputs/input_contract_init.txt` to olavm for execution.

Then, you can sequentially encode the parameters for the vote_proposal method and the `winningProposal` method and invoke them：
````shell
./olaws.sh encode -f "vote_proposal(u32)" -a 2
./olaws.sh execute vote_proposal
./olaws.sh encode -f "winningProposal()"
./olaws.sh execute winningProposal
````

If everything goes well, you will see the message:
```shell
Congrats! You've completed your first challenge at Ola x ZK Hack Istanbul!
```

Indicating that you have successfully edited, compiled, and executed the contract. Congratulations on completing the task!


# Benchmark
| Algorithm                                       | Execution Instructions | Lines in CPU Table | Mac(8-cpu 16GB-Mem) Execution and Generate trace Time | Mac(8-cpu 16GB-Mem) Prove Time | Linux(32-cpu 256GB-Mem) Execution and Generate Trace Time | Linux(32-cpu 256GB-Mem) Prove Time |
| :---------------------------------------------- | :--------------------: | :----------------: | :---------------------------------------------------: | :----------------------------: | :-------------------------------------------------------: | :--------------------------------: |
| Calculate the 47th Fibonacci number 300 times.  |         259915         |        2^18        |                    0.274s, 1.379s                     |            109.263s            |                      1.305s, 1.273s                       |              38.521s               |
| Calculate the sqrt of 1,073,741,824 7000 times. |         238113         |        2^18        |                    1.191s, 1.335s                     |            109.873s            |                      0.697s, 1.266s                       |              38.654s               |


# Open Source Repositories
- [OlaVM](https://github.com/Sin7Y/olavm): Ola's Executor and Prover
- [Ola-lang](https://github.com/Sin7Y/ola-lang.git): Ola-lang Compiler


# Contact
Twitter: https://twitter.com/ola_zkzkvm

Discord: http://discord.gg/vDFy7YEG6j

Email: contact@olavm.org

Feedback: https://forms.gle/Sy3Aa3MBGSXEy64U8





