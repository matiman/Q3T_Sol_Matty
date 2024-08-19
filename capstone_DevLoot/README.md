DevLoot - User Stories

Google Docks Link: https://docs.google.com/document/d/1wVGIC_OXgrSrWkDeLMdQfoU17As3TCAep8PGiTd-xqY/edit?usp=sharing

DevLoot is an interactive platform that enables Web2 engineers to learn Solana development in a fun and effective way. 
Code it, visualize it, and never forget it. 

User Stories


Student
As a student, on DevLoot.com, I should be able to click “connect wallet”, so that I can see an option to connect my 
Phantom wallet.
As a student, I should be able to select the wallet options provided, and click “connect wallet”, so that I can see a 
“sign” message in order to sign a transaction to connect my wallet with DevLoot.
As a student, I should be able to click “sign”, so that my wallet is connected to DevLoot.
As a student with a connected wallet, I should be able to see Solana course content with text and visual media, so that 
I can start learning.
As a student, if my wallet is not connected, I should be redirected to the DevLoot home page, and see the “connect 
wallet” button. 
As a student, I should be able to see questions after reading the contents of the course. 
For multiple choice questions, I should be able to select one answer from the choices, and click the submit button.
If my answer is correct, I should see the “correct answer” text in green below the question and also see my score go up 
by some points depending on the difficulty of the question to the top right corner.
If my answer is incorrect, I should see “try again” text in red below the question and should be given a maximum of up 
to 3 trials. My score wouldn’t change.
If I exhausted 3 trials, and my answer is still incorrect, I should see the correct answer in blue text, and I will lose 
1pt. The page should automatically display the next content or question.
As a student, if I finish all the contents and questions for a section, I should see a pop up window with a text 
“Congratulations, you completed the course.”
If my score is above 80%, I should also receive 2 memecoins (BONK and WIF), and a DevLoot Diamond NFT into my connected 
wallet.
If my score is below 80%, I should receive 2 memecoins (BONK and WIF), and DevLoot Gold NFT into my connected wallet.
As a student who completed the first course with Gold NFT, I should see a link “Repeat Course” that will take me to the 
beginning of the course so that I can revise the course.
As a student who completed the first course with Diamond NFT, I should see a link “Next - Advanced Course” that will 
take me to the Advanced Course page.
As a Diamond student, I should see text and image content with a command line window that will allow me to run Solana 
related commands.
In the command line, I should be able to type solana related commands and be able to click the “Run” button, which will 
allow me to run the commands on a browser without installing any libraries or tools.
As a Diamond student, now I should be able to see contents as a normal student and also a command line and continue my 
learning journey.
Student Profile
As a student with a connected wallet , If I click “Profile” button:
If I am a student with no NFTs issued from DevLoot, I should see a blank profile picture, with my current points, and 
also the percentage of the course that I completed.
If I am a student with NFTs issued from DevLoot and it’s not staked, I should see my NFT as a profile picture and the 
hexadecimal part of the NFT address that points to it. 
If I am a student with 2 memecoins received, and I haven’t provided liquidity, I should see the Memecoin picture with 
its hexadecimal address.
If I am a student with NFTs issued and it’s staked, I should see the NFT and also the staking rewards amount I am 
receiving in USDC that gets updated every 24 hrs.
If I am a student with Memecoins issued and I provided liquidity to the AMM pool, I should see the Memecoins picture 
with its hexadecimal picture, and also the fees I am earning from the AMM pool IN USDC which gets updated every 24 hrs.
If I am a student with NFTs, I should be able to see the “Stake” button. When I click the “Stake” button, I should see a 
pop up window with my NFT to stake, the lock period (7 days), and “stake” button.
Once I click Stake, my NFT should be staked, and I won't be able to unstake it for 7 days. 
If I am a student with 2 Memecoins, I should be able to see the “Provide Liquidity” button. When I click the “Provide 
Liquidity” button, I should see a pop up window with my 2 Memecoins, the lock period (7 days), and the “provide 
liquidity” button.
Once I click “provide liquidity”, my 2 Memecoins should be provided, and I won't be able to withdraw it for 7 days. 
As a student who staked an NFT, I should see a “Unstake” button on the home page after 7 days of staking.
When I click “Unstake”, I should be able to see a pop up window with the maximum amount I can unstake and an “Unstake” 
button.
When I click confirm, I should be able to get back my NFT and the total amount of USDC that the staking platform 
generated into my wallet.
As a student who provided liquidity to the AMM pool, I should see a “Withdraw Liquidity” button on the home page after 7 
days of providing liquidity.
When I click “Withdraw Liquidity”, I should be able to see a pop up window with the maximum amount I can of fees I can 
withdraw and a “withdraw” button.
When I click withdraw, I should be able to get back my 2 Memecoins and the total amount of fees generated in USDC that 
the staking platform generated into my wallet.

DevLoot System
As a DevLoot System, the program should mint 1 NFT(Diamond or Gold) and 2 Memecoins(BONK and WIF) to those who completed 
the Beginner course and score 80% or above.
As a DevLoot System, each time a student is progressing, it should increment their score, and store it as a score in the 
user_course account.
As a DevLoot System, every 24hrs, it should calculate Staking points and Amm fees generated for each user who staked 
their NFT and/or provided liquidity to the AMM pools, and store it in the user_nft and user_amm account respectively. 
DevLoot DAO Withdrawal Handler.
As a DAO, it should be able to receive withdrawal requests from a student with a NOVI LP token holder with a given 
amount, so that it can calculate how much a student should get back on the token mint(USDC/SOL) they used to deposit.
As a DAO, it should calculate the amount the student needs to get back a given token, withdraw the amount from the 
vault, sell it on Jupiter (with correct slippage), calculate the fee to charge the student, and send the SOL/USDC 
tokens(minus fee) back to the student account, and burn the NOVI LP token.
As a DAO, after withdrawal success, it should call the rebalance algorithm, to rebalance current token allocation 
accordingly.
DevLoot DAO Rebalancer
As a DAO, it should accept deposit and withdrawal requests, and its algorithm should balance/swap meme tokens for 
maximum profit to its vault and students.
User Course settings
TBD
Admin Settings
Content and Questions .. TBD
Fees ..TBD


