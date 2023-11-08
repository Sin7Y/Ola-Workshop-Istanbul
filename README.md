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
We have prepared a script for this workshop to facilitate developers in quickly setting up the development environment. You can download it here and execute it in your local workshop directory. [Click to download](https://google.com).

Ensure that the script has the permission to be executed：
````shell
chmod +x olaws.sh
````

Download the ola-lang compiler and the OlaVM executable files:
````shell
./olaws.sh -d
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
Use your favorite editor to fill in the missing logic of the contract. It is recommended to use VSCode; we have provided a VSCode plugin for Ola-lang, which you can find [here](https://marketplace.visualstudio.com/items?itemName=Sin7y.ola).


Compile the smart contract:
````shell
./olaws.sh -c --input vote.ola --output vote.json
````

Invoke the contract to vote and generate trace:
````shell
./olaws.sh -et --program vote.json --trace trace_vote.json --fname vote --args 1
````

Check vote result:
````shell
./olaws.sh -e --program vote.json --trace trace_vote.json --fname check_result
````

If everything goes well, you will see the message "Congrats! You've completed your first challenge at Ola x ZK Hack Istanbul!" indicating that you have successfully edited, compiled, and executed the contract. Congratulations on completing the task!


If everything goes well, you will see the message:
>Congrats! You've completed your first challenge at Ola x ZK Hack Istanbul!

Indicating that you have successfully edited, compiled, and executed the contract. Congratulations on completing the task!


You can opt to generate a proof using the trace that was just created:
````shell
./olaws.sh -p --trace trace_vote.json --proof proof_vote
````

And verify the proof:
````shell
./olaws.sh -v --proof proof_vote
````

# Benchmark
todo


# Open Source Repositories
- [OlaVM](https://github.com/Sin7Y/olavm): Ola's Executor and Prover
- [Ola-lang](https://github.com/Sin7Y/ola-lang.git): Ola-lang Compiler


# Contact
Twitter: https://twitter.com/ola_zkzkvm

Discord: http://discord.gg/vDFy7YEG6j

Email: contact@olavm.org





