Devnet Program Id: **DGpFWc2R2aMZc8rF9jSBuqBrtnQ7VdD8nCuTwJ6vNp6**

DevLoot is an **interactive** platform that allows engineers to learn Solana development with **visual aids and execution environment** on the browser so they can **learn by doing.**

<img width="1235" alt="Screenshot 2024-09-17 at 7 56 48 AM" src="https://github.com/user-attachments/assets/bdfbd7ae-6c2e-461a-8d21-42d77271e91b">


We are the **Brilliant.org** of Solana. 

Learning Solana is difficult compared to other blockchains like Ethereum because of its different structure from EVM based blockchains and the use of Rust. However, that also offers Solana to be the most efficient and composable blockchain that ever exists and we want to make new engineers journey a walk in the park.

Our concept is taken from “**Learn How to Learn**” that helps anyone to learn new concept quickly and also build a strong muscle memory foundation that can’t be forgotten using interactive learning, visualization and spaced repetition. 

The first MVP contains only the backend Anchor program that allows us to measure students progress onchain which can be verified by anyone in the future.

Feel free to refer to the [Flow chart](https://github.com/matiman/Q3T_Sol_Matty/blob/main/capstone_DevLoot/DevLoot%20-%20Userstory.docx), [Lean canvas business plan](https://github.com/matiman/Q3T_Sol_Matty/blob/main/capstone_DevLoot/Lean%20Canvas-1.png), [User stories](https://github.com/matiman/Q3T_Sol_Matty/blob/main/capstone_DevLoot/DevLoot%20-%20Userstory.docx), and [Architectural Diagram](https://github.com/matiman/Q3T_Sol_Matty/blob/main/capstone_DevLoot/Architectural%20Diagram.png).

**How the backend works ?** 

- **Course** 
    - Admins create a course, and its metadata including but not limited to 
        - course id,
        - total questions
        - total content (index from 0 to 256 where each index represents a string of content that is fetched from off chain location)
- **Student**
    - **Student Registration** 
        - **FREE**
            - Student registers to a course
            - Student Progress account is created and initially set to default values
            - Some fields include
                - Course id:
                - Current content at: (the index where they are currently in the course)
                - Total points earned: (10 points earned for every question answered in first trial and 5   points for second trail)
                - Is Completed: wether they completed a course or not.
        - **PAID** [TBD]
            - _Students are required to send 0.001 SOL to our vault from their phantom wallet._
            - _And we stake their SOL with Jito in return for JitoSOL Liquid Staking Token (LST)._
            - _When they finish their course,_
                - _We use the staking and MEV rewards from Jito to buy them BONK and send it to their wallet._
                - _They also get back their full 0.001 SOL_
    - **Student Progresses**
        - Once a student goes to the next content, student progress account is updated by increamenting the content at.
        - When they answer questions correctly we increment their points.
    - **Student Reward**
        - Once a student finishes the course (i.e when they student content at is equal to total content index of the course minus 1, we mark the course as complete. 
        - Once complete, student gets Diamond NFT if they get 80+ points or Gold NFT for < 80 points.
        - For Paid students they also get additional rewards mentioned above in the PAID section.
- **How to run the code?**
    - Requirements:
        - `Anchor 0.30.1`
        - `Solana 1.18.17,`
        - `yarn 1.22.22 , `
        - `Solana Validator`
    - Run `solana-test-validator -r`
        - Make sure the Anchor.toml file points to localnet
            - [provider]
                cluster = "localnet"  
    - To test locally, On another terminal, under /capstone_DevLoot/dev-loot/
        - `anchor build`
        - `anchor test --skip-local-validator`
    - To test on devnet, change the anchor.toml file to point to devnet
        - I already deployed it on Dev net with Program Id `DGpFWc2R2aMZc8rF9jSBuqBrtnQ7VdD8nCuTwJ6vNp6`     
        - [provider]
           cluster = "devnet"
        - Then run `anchor build`,
        - After successful build, run `anchor test`.        
    - All tests should pass. 
